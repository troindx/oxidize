use std::str::FromStr;
use std::sync::Arc;
use async_trait::async_trait;
use rocket_db_pools::mongodb::bson::{ self, bson, doc};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::results::InsertOneResult;
use rocket_db_pools::mongodb::Collection;
use rocket_db_pools::mongodb::error::Error;
use log::error;
use crate::modules::mongo::service::MongoOracle;
use crate::modules::CRUDMongo;
use super::dto::User;

pub struct UserService {
    pub mongo: Arc<MongoOracle>,
    pub users: Collection<User>,
}

#[async_trait]
impl CRUDMongo<User> for UserService{

    async fn create(&self, user: User) -> Option<ObjectId>{
        let  new_user_result: Result<InsertOneResult, Error> = self
            .users
            .insert_one(user.to_owned(), None)
            .await;
        match new_user_result{
            Ok(new_user) => new_user.inserted_id.as_object_id(),
            Err(e) =>{
                error!("Error creating user: {}", e);
                None
            } 
        }
    }

    async fn read(&self, id: ObjectId) -> Option<User> {
        let filter = doc! {"_id": &id};
        let user_result = self
            .users
            .find_one(filter, None)
            .await;
        match user_result {
            Ok(user) => user,
            Err(e) => {
                error!("Error reading user with id {}: {}", id, e);
                None
            } 
        }
    }
    
    async fn update(&self, user: User) -> Option<User> {
        let filter = doc! {"_id": &user._id};
        
        let update_doc = doc! {"$set": bson::to_document(&user).expect("Failed to convert User to Document")};
        let user_res = self
            .users
            .find_one_and_update(filter, update_doc, None)
            .await;
        match user_res {
            Ok(up_user) => up_user,
            Err(e) =>{
                error!("Error updating user with id {}: {}", user._id?, e);
                None
            }  
        }
    }

    async fn delete(&self, id: ObjectId) -> u64 {
        let filter = doc! {"_id": &id};
        let user = self
            .users
            .delete_one(filter, None)
            .await;
        match user {
            Ok(user) => user.deleted_count,
            Err(e) => {
                error!("Error deleting user with id {}: {}", id, e);
                0
            }
        }
    }
}

impl UserService {
    pub fn new(mongo: Arc<MongoOracle>) -> Self {
        let db = mongo.db.as_ref().expect("Database not initialized");
        let users: Collection<User> = db.collection("users");
        Self { mongo, users }
    }
}