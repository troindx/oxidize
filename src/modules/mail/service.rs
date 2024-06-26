extern crate rand;
extern crate base64;

use std::sync::Arc;

use chrono::Utc;
use log::error;
use rand::Rng;
use rand::distributions::Alphanumeric;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rocket::uri;
use rocket_db_pools::mongodb::bson::{self, doc};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::options::IndexOptions;
use rocket_db_pools::mongodb::results::{InsertOneResult, UpdateResult};
use rocket_db_pools::mongodb::{Collection, IndexModel};
use rocket_db_pools::mongodb::error::Error;
use crate::framework::config::OxidizeConfig;
use crate::framework::translator::OxidizeTranslator;
use crate::modules::mongo::service::MongoOracle;
use crate::modules::user::dto::User;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::io;
use super::dto::EmailVerification;
pub struct MailOracle {
    pub config: Arc<OxidizeConfig>,
    pub mongo: Arc<MongoOracle>,
    pub verifications: Collection<EmailVerification>,
    pub translator: Arc<OxidizeTranslator>,
}

impl MailOracle {

    pub fn new( config: Arc<OxidizeConfig>, mongo: Arc<MongoOracle>, translator: Arc<OxidizeTranslator> ) -> Self {
        let db = mongo.db.as_ref().expect("Database not initialized");
        mongo.add_collection("email_verifications");
        let verifications: Collection<EmailVerification> = db.collection("email_verifications");
        Self {config, mongo, verifications, translator}
    }

    fn generate_random_url_safe_string(&self, length: usize) -> String {
        // Generate a random string of the given length
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
    
        // Encode the random string in a URL-safe base64 format
        let mut encoded = String::new();
        URL_SAFE_NO_PAD.encode_string(&random_string, &mut encoded);
        encoded
    }

    pub async fn start_verification(&self , user: &User) -> Option<EmailVerification>{
        let secret = self.generate_random_url_safe_string(self.config.env.default_email_verification_key_length);

        let previous_verification = self.find_verification_by_user_id(&user._id.clone().expect("User id not found")).await;
        let mut verification = if previous_verification.is_some() {
            let mut tmp = previous_verification.unwrap();
            tmp.updated = Utc::now();
            tmp.verified = false;
            tmp.secret = secret;
            self.update(&tmp).await.expect("Error updating verification on start step");
            tmp
        } else {
            let mut tmp =EmailVerification { 
                user_id: user._id.clone().expect("User id not found"),
                email:user.email.clone(), 
                secret , 
                _id:None, 
                created: Utc::now(),
                updated: Utc::now(),
                verified: false
            };
            let inserted_id = self.create(&tmp).await.expect("Error saving verification on start step");
            tmp._id = Some(inserted_id);
            tmp
        };
        verification._id = Some(verification._id.clone().expect("Verification id not found"));
        self.send_verification_mail(&verification.email, verification.clone()).await;
        Some(verification)
    }


    pub async fn find_verification_by_email(&self, email: &str) -> Option<EmailVerification> {
        let filter = doc! {"email": email};
        match self.verifications.find_one(filter, None).await {
            Ok(verification) => verification,
            Err(e) => {
                error!("Error finding email verification with email {}: {}", email, e);
                None
            }
        }
    }

    pub async fn find_verification(&self, id: &ObjectId) -> Option<EmailVerification> {
        let filter = doc! {"_id": &id};
        let verification_result = self
            .verifications
            .find_one(filter, None)
            .await;
        match verification_result {
            Ok(verification) => verification,
            Err(e) => {
                error!("Error reading user with id {}: {}", id, e);
                None
            } 
        }
    }

    pub async fn find_verification_by_user_id(&self, user_id: &ObjectId) -> Option<EmailVerification> {
        let filter = doc! {"user_id": user_id};
        match self.verifications.find_one(filter, None).await {
            Ok(verification) => verification,
            Err(e) => {
                error!("Error finding user_id verification with {}: {}", user_id.to_string(), e);
                None
            }
        }
    }


    async fn create(&self, verification: &EmailVerification) -> Option<ObjectId> {
        let new_verification_result: Result<InsertOneResult, Error> = self
            .verifications
            .insert_one(verification, None)
            .await;
        
        match new_verification_result {
            Ok(resp) =>  resp.inserted_id.as_object_id(),
            Err(e) => {
                error!("Error creating verification: {}", e);
                None
            }
        }
    }

    pub async fn finish_verification(&self, user_id: &ObjectId, verification_id: &ObjectId, secret: &str ) -> Result<EmailVerification, io::Error> {
        let verification = self.find_verification(verification_id).await;
        if verification.is_none() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Verification not found"));
        }
        let mut  verification = verification.unwrap();
        if user_id != &verification.user_id {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Wrong user id in verification"));
        }

        if secret == verification.secret{
            verification.verified = true;
            self.update(&verification).await.expect("Error updating verification on finsih step");
            Ok(verification)
        }
        else{
            Err(io::Error::new(io::ErrorKind::InvalidData, "Verification secret does not match"))
        }

    }

    async fn update(&self, verification: &EmailVerification) -> Option<UpdateResult> {
        let filter = doc! {"_id": &{verification._id}};
        
        let update_doc = doc! {"$set": bson::to_document(&verification).expect("Failed to convert EmailVerification to Document")};
        let verify_res = self
            .verifications
            .update_one(filter, update_doc, None)
            .await;
        match verify_res {
            Ok(up_verification) => Some(up_verification),
            Err(e) =>{
                error!("Error updating user with id {}: {}", verification._id?, e);
                None
            }  
        }
    }

    pub async fn initialize_db(&self) -> Result<(), Error> {
        // Create unique index on email field
        let index = IndexModel::builder().keys(doc! { "user_id": 1 }).
            options(IndexOptions::builder().unique(true).build()).build();

        self.verifications.create_index(index,None)
            .await.expect("Error creating index for email in verifications.");
        Ok(())
    }

    pub async fn send_verification_mail(&self, mail_to: &str, verification: EmailVerification)  {
        let link = uri!(crate::modules::mail::controller::finish_verification(
            id=verification._id.unwrap().to_string(), 
            secret=verification.secret.clone()));
        if self.config.env.run_mode == "dev" {
            println!("Email verification link: {}",link.to_string());
            return;
        }
        
        self.translator.get("verify_email_body", None);
        let email_body = self.translator.get("verify_email_body", Some(vec![("link", link.to_string().into())]));
        // Define the email content and sender/recipient details
        let email = Message::builder()
        .from(self.config.env.email_sender_from.parse().unwrap())
        .reply_to(self.config.env.email_reply_to.parse().unwrap())
        .to(mail_to.parse().unwrap())
        .subject(self.translator.get("verify_email_subject", None))
        .body(email_body)
        .unwrap();

        // Define SMTP server credentials
        let creds = Credentials::new(self.config.env.smtp_user.clone(), self.config.env.smtp_password.to_string());

        // Connect to an SMTP relay server
        let mailer = SmtpTransport::relay(&self.config.env.smtp_host)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => eprintln!("Could not send email: {:?}", e),
        }
    }
    
}
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{framework::config::OxidizeConfig, modules::mongo::service::MongoOracle};

    use super::*;

    #[tokio::test]
    async fn test_ranodom_string_generator() {
        let config = Arc::new(OxidizeConfig::new().expect("Error while getting config"));
        let mongo = Arc::new(MongoOracle::new(config.clone()).await);
        let translator = Arc::new(OxidizeTranslator::new(config.clone()));
        let mail = MailOracle::new(config, mongo, translator);
        let length = 32;
        let result = mail.generate_random_url_safe_string(length);

        // URL-safe Base64 alphabet
        let url_safe_base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

        // Check that all characters in the result are URL-safe Base64 characters
        for c in result.chars() {
            assert!(url_safe_base64_chars.contains(c), "Character {} is not URL-safe", c);
        }

        // Optional: Check the length of the generated string
        // This is tricky because URL_SAFE_NO_PAD encoding length depends on the input length
        // Base64 encoding increases the length by ~4/3, so we can check for some reasonable bounds
        let expected_min_length = (length * 4 / 3) as usize;
        assert!(result.len() >= expected_min_length, "Encoded string is too short");
    }
}