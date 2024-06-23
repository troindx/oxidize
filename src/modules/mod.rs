use async_trait::async_trait;
use rocket_db_pools::mongodb::{bson::oid::ObjectId, error::Error, results::{DeleteResult, InsertOneResult, UpdateResult}};

pub mod mongo;
pub mod user;
pub mod mail;

#[async_trait]
#[allow(dead_code)]
pub trait CRUDMongo<T> {
    async fn create(&self, new: T) ->  Option<InsertOneResult>;
    async fn read(&self, id: ObjectId) -> Option<T>;
    async fn update(&self, item: T) -> Option<UpdateResult>;
    async fn delete(&self, id: ObjectId) -> Option<DeleteResult>;
    async fn find_by_email(&self, email: &str) -> Option<T>;
    async fn initialize_db(&self) -> Result<(), Error>;
}