use chrono::{DateTime, Utc};
use rocket::form::FromForm;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::bson::Bson;

#[derive(Debug, Deserialize, Serialize,Clone, FromForm)]
pub struct User<'r> {
    pub email : &'r str ,
    pub password : &'r str,
    pub description : &'r str,
    pub is_admin : bool,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id : Option <ObjectId> 
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Guest,
    Registered,
    Admin
}
#[derive(Debug, FromForm, Deserialize, Serialize,Clone)]
pub struct UserDTO {
    pub user : User ,
    pub jwt_secret : String,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option <String> 
}