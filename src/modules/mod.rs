use async_trait::async_trait;
use rocket_db_pools::mongodb::bson::oid::ObjectId;

pub mod mongo;
pub mod user;

#[async_trait]
#[allow(dead_code)]
pub trait CRUDMongo<T> {
    async fn create(&self, new: T) ->  Option<ObjectId>;
    async fn read(&self, id: ObjectId) -> Option<T>;
    async fn update(&self, item: T) -> Option<T>;
    async fn delete(&self, id: ObjectId) ->u64;
}