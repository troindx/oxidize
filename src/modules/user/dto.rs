use chrono::{DateTime, Utc};
use rocket::form::{Errors, FromForm, FromFormField, ValueField };
use rocket::form;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::bson::Bson;
use serde_json::Number;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UserId(ObjectId);

impl From<ObjectId> for UserId {
    fn from(oid: ObjectId) -> Self {
        UserId(oid)
    }
}

impl Into<ObjectId> for UserId {
    fn into(self) -> ObjectId {
        self.0
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for UserId {
    fn from_value(field: ValueField<'v>) -> form::Result<'v, Self> {
        match ObjectId::parse_str(field.value) {
            Ok(oid) => Ok(UserId(oid)),
            Err(_) => Err(Errors::from(form::Error::validation("Invalid ObjectId format"))),
        }
    }
}

#[derive(Debug, Deserialize, Serialize,Clone, FromForm)]
pub struct User<'r> {
    pub email : &'r str ,
    pub password : &'r str,
    pub description : &'r str,
    pub role : u8,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id : Option <UserId> 
}

#[derive(Debug, Clone)]
pub enum UserRole {
    Guest,
    Registered,
    Admin
}

#[derive(Debug, FromForm, Deserialize, Serialize,Clone)]
pub struct UserDTO<'a> {
    pub user : 'a+ User,
    pub jwt_secret : String,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option <String> 
}