use std::sync::Arc;

use chrono_tz::America::North_Dakota::New_Salem;
use rocket_db_pools::mongodb::bson::Bson;
use rocket_db_pools::mongodb::Cursor;
use rocket_db_pools::mongodb::Database;
use rocket_db_pools::mongodb::bson::doc;
use rocket_db_pools::mongodb::results::{ InsertOneResult};
use rocket_db_pools::mongodb::{Collection, Client};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use rocket_db_pools::mongodb::error::Error;

use crate::modules::mongo::service::MongoOracle;
use crate::modules::CRUD;

use super::dto::User;

pub struct UserService {
    mongo: Arc<MongoOracle>,
    pub users: Collection<User>,
}

impl CRUD<User> for UserService{

    async fn create<'a>(&self, user: &'a User) -> Option<User>{
        let  new_user_result: Result<InsertOneResult, Error> = self
            .users
            .insert_one(user.to_owned(), None)
            .await;
        match new_user_result{
            Ok(new_user) => Some(new_user.inserted_id),
            Err(e) => None
        }
    }

    async fn read(&self, id: ObjectId) -> Option<User> {
        let filter = doc! {"_id": id};
        let user_result = self
            .users
            .find_one(filter, None)
            .await;
        match user_result {
            Ok(user) => Some(user),
            Err(_) =>  None
        }
    }
    
    async fn update<'a>(&self, user:&'a User) -> Option<User> {
        let filter = doc! {"_id": user._id.to_owned()};
        let user = self
            .users
            .find_one(filter, None)
            .await;
        match user {
            Ok(user) => Some(user),
            Err(_) =>  None
        }
    }

    async fn delete(&self, id: ObjectId) -> Option<User> {
        let filter = doc! {"_id": id};
        let user = self
            .users
            .delete(filter, None)
            .await;
        match user {
            Ok(user) => Some(user),
            Err(_) =>  None
        }
    }
}

impl UserService {
    pub fn new(mongo : Arc<MongoOracle>) -> Self {
        let users: Collection<User> = db.collection("users");
        Self {mongo, users }
    }
}
