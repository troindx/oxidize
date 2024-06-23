use std::sync::Arc;
use async_trait::async_trait;
use rocket_db_pools::mongodb::bson::{ self, doc};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::options::IndexOptions;
use rocket_db_pools::mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use rocket_db_pools::mongodb::{Collection, IndexModel};
use rocket_db_pools::mongodb::error::Error;
use log::{error, warn};
use crate::modules::mongo::service::MongoOracle;
use crate::modules::CRUDMongo;
use super::dto::User;

pub struct UserService {
    pub mongo: Arc<MongoOracle>,
    pub users: Collection<User>,
}

#[async_trait]
impl CRUDMongo<User> for UserService{

    /// Creates a user and returns Some(InsertOneResult) or none if the user already exists in the system with that email
    /// 
    async fn create(&self, user: User) -> Option<InsertOneResult> {
        // Check if a user with the given email already exists
        let existing_user = self.find_by_email(&user.email).await;
        if existing_user.is_some() {
            warn!("User with email {} already exists", user.email);
            return None;
        }

        let new_user_result: Result<InsertOneResult, Error> = self
            .users
            .insert_one(user.to_owned(), None)
            .await;
        
        match new_user_result {
            Ok(resp) => Some(resp),
            Err(e) => {
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
    
    async fn find_by_email(&self, email: &str) -> Option<User> {
        let filter = doc! {"email": email};
        match self.users.find_one(filter, None).await {
            Ok(user) => user,
            Err(e) => {
                error!("Error finding user with email {}: {}", email, e);
                None
            }
        }
    }
    
    async fn update(&self, user: User) -> Option<UpdateResult> {
        let filter = doc! {"_id": &user._id};
        
        let update_doc = doc! {"$set": bson::to_document(&user).expect("Failed to convert User to Document")};
        let user_res = self
            .users
            .update_one(filter, update_doc, None)
            .await;
        match user_res {
            Ok(up_user) => Some(up_user),
            Err(e) =>{
                error!("Error updating user with id {}: {}", user._id?, e);
                None
            }  
        }
    }

    async fn delete(&self, id: ObjectId) -> Option<DeleteResult> {
        let filter = doc! {"_id": &id};
        let user = self
            .users
            .delete_one(filter, None)
            .await;
        match user {
            Ok(res) => Some(res),
            Err(e) => {
                error!("Error deleting user with id {}: {}", id, e);
                None
            }
        }
    }

    async fn initialize_db(&self) -> Result<(), Error> {
        // Create unique index on email field
        let index = IndexModel::builder().keys(doc! { "email": 1 }).
            options(IndexOptions::builder().unique(true).build()).build();

        self.users.create_index(index,None)
            .await.expect("Error creating index for email in users.");
        Ok(())
    }
}

impl UserService {
    pub fn new(mongo: Arc<MongoOracle>) -> Self {
        let db = mongo.db.as_ref().expect("Database not initialized");
        let users: Collection<User> = db.collection("users");
        mongo.add_collection("users");
        Self { mongo, users }
    }
}