use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;

// Define the function that provides the default value for email_verified
fn default_email_verified() -> bool {
    false
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct User {
    pub email : String ,
    pub password : String,
    pub description : String,
    pub role : u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id : Option<ObjectId>,
    pub public_key : String,
    #[serde(default = "default_email_verified")]
    pub email_verified: bool,
}

impl User {
   pub fn is_email_verified(&self) -> bool {
    self.email_verified
   } 

   pub fn verify_email (&self) {
    //TODO
   }
}

pub enum UserRoles{
    GUEST,
    USER,
    ADMIN,
}