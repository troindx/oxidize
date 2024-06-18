use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct User {
    pub email : String ,
    pub password : String,
    pub description : String,
    pub role : u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id : Option<ObjectId>
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct UserDTO {
    pub user : User,
    pub jwt_secret : String,
    pub id : String 
}