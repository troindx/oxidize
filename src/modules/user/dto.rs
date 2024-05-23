
use rocket::form::FromForm;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize,Clone, FromForm)]
pub struct User {
    pub email : String ,
    pub password : String,
    pub description : String,
    pub role : u8,
    pub _id : String
}

#[derive(Debug, FromForm, Deserialize, Serialize,Clone)]
pub struct UserDTO {
    pub user : User,
    pub jwt_secret : String,
    pub id : String 
}