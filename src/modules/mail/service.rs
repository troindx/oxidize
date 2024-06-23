extern crate rand;
extern crate base64;

use std::sync::Arc;

use rand::Rng;
use rand::distributions::Alphanumeric;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::framework::config::OxidizeConfig;
use crate::modules::mongo::service::MongoOracle;
use crate::modules::user::dto::User;

use super::dto::EmailVerification;
pub struct MailOracle {
    pub config: Arc<OxidizeConfig>,
    pub mongo: Arc<MongoOracle>
}

impl MailOracle {

    pub fn new( config: Arc<OxidizeConfig>, mongo: Arc<MongoOracle> ) -> Self {
        Self {config, mongo}
    }

    pub fn generate_random_url_safe_string(&self, length: usize) -> String {
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

    pub fn start_verification(&self , user: &User){
        let key = self.generate_random_url_safe_string(self.config.env.default_email_verification_key_length);
        let verification = EmailVerification { email:user.email.clone(), secret:key };
        
    }
    
}