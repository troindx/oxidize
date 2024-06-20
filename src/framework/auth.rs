use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use chrono::TimeDelta;
use jsonwebtoken::{encode, EncodingKey, Header};
use rsa::{pkcs8::LineEnding, pkcs8::der::zeroize::Zeroizing,pkcs8::EncodePrivateKey, pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey};
use jsonwebtoken::Algorithm;

pub fn generate_rsa_key_pair_pem() -> (String, Zeroizing<String>) {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Convert the private key to PEM format using PKCS#8
    let private_key_pem = priv_key.to_pkcs8_pem(LineEnding::CRLF).expect("failed to convert private key to PEM");

    // Convert the public key to PEM format
    let public_key_pem = pub_key.to_public_key_pem(LineEnding::CRLF).expect("failed to convert public key to PEM");

    (public_key_pem, private_key_pem)
}

pub fn generate_jwt_token(user_id: &str, secret_key: &str, duration:TimeDelta) -> Result<String, Box<dyn std::error::Error>> {
    //let x = chrono::Duration::hours(1);
    let claims = Claims { 
        user_id: user_id.to_owned(),
        exp: (chrono::Utc::now() + duration).timestamp() as usize,
    };

    // Encode the JWT token
    let token = encode(
        &Header::new(Algorithm::RS512),
        &claims,
        &EncodingKey::from_rsa_pem (secret_key.as_ref()).expect("Error when encoding secret key"),
    )?;

    Ok(token)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::decode;
    use jsonwebtoken::DecodingKey;
    use jsonwebtoken::Validation;
    use rocket_db_pools::mongodb::bson::oid::ObjectId;
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::pkcs8::DecodePublicKey;
    use crate::framework::app::create_rocket_instance;
    use crate::framework::app::App;
    use crate::modules::user::dto::User;
    use crate::modules::CRUDMongo;
    use super::*;

    #[test]
    fn test_generate_rsa_key_pair_pem() {
        // Generate RSA key pair in PEM format
        let (public_key_pem, private_key_pem) = generate_rsa_key_pair_pem();

        // Ensure the keys are not empty
        assert!(!public_key_pem.is_empty(), "Public key PEM should not be empty");
        assert!(!private_key_pem.is_empty(), "Private key PEM should not be empty");
        
        // Ensure the public key starts with the correct header
        assert!(public_key_pem.starts_with("-----BEGIN PUBLIC KEY-----"), "Public key PEM should start with the correct header");

        // Ensure the private key starts with the correct header
        assert!(private_key_pem.starts_with("-----BEGIN PRIVATE KEY-----"), "Private key PEM should start with the correct header");

        // Verify that the PEM formatted private key can be parsed correctly
        RsaPrivateKey::from_pkcs8_pem(&private_key_pem).expect("Failed to parse private key PEM");
        // Verify that the PEM formatted public key can be parsed correctly
        RsaPublicKey::from_public_key_pem(&public_key_pem).expect("Failed to parse public key PEM");

    }

    #[tokio::test]
    async fn test_jwt_generator() {
        let testing_runtime = create_rocket_instance(true).await;
        let app: &App = testing_runtime.state().expect("No instance of App in testing Runtime");
        let (pub_key, priv_key) = generate_rsa_key_pair_pem();
        let (_, malicious_priv_key) = generate_rsa_key_pair_pem();

        let mut user = User {
            email: String::from("Atest_user2@example.com"),
            password: String::from("atest_password2"),
            description: String::from("Test Description"),
            public_key: pub_key.to_owned(),
            role: 1,
            _id: None,
        };
        let registered_user_id = app.users.create(user.to_owned())
            .await
            .expect("Error while inserting user").inserted_id.as_object_id();
        user._id = registered_user_id;
        
        let decoding_key = DecodingKey::from_rsa_pem(pub_key.as_bytes())
            .expect("Invalid public key");

        let token = generate_jwt_token(ObjectId::to_string(&registered_user_id.unwrap()).as_str(), priv_key.as_str(), chrono::Duration::hours(1))
            .expect("Error while generating JWT Token");
        
        //Check it verifies with correct decoding key
        let decodification = decode::<Claims>(token.as_str(), &decoding_key, &Validation::new(Algorithm::RS512))
            .expect("Error when decoding Token Data");
        assert!(decodification.claims.user_id == ObjectId::to_string(&registered_user_id.unwrap()));

        //Check that it doesn't verify someone else's key
        let token = generate_jwt_token(ObjectId::to_string(&registered_user_id.unwrap()).as_str(), malicious_priv_key.as_str(), chrono::Duration::hours(1))
            .expect("Error while generating JWT Token");
        
        let decodification = decode::<Claims>(token.as_str(), &decoding_key, &Validation::new(Algorithm::RS512));
        assert!(decodification.is_err());
    }
}