use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use fake::faker::internet::en::FreeEmail;
use fake::faker::internet::en::Password;
use fake::faker::lorem::en::Paragraph as Lorem;
use fake::Fake;

use crate::framework::testing::Mock;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct User {
    pub email : String ,
    pub password : String,
    pub description : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id : Option<ObjectId>,
    pub public_key : String,
}

impl Mock for User {
    fn mock() -> User {
        let(pub_key,_) = super::super::super::framework::auth::generate_rsa_key_pair_pem();
        User {
            email: FreeEmail().fake(),
            password: Password(8..20).fake(),
            description: Lorem(10..100).fake(),
            public_key: pub_key,
            _id: None,
        }
    }
}

pub enum UserRoles{
    GUEST,
    USER,
    ADMIN,
}