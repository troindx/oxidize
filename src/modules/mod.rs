

use futures::Future;
use rocket_db_pools::mongodb::{bson::Bson, error::Error};

pub mod mongo;
pub mod user;

#[async_trait]
pub trait CRUD<T> {
    async fn create(&self, new: T) ->  Result<Bson,Error>;
    async fn read(&self, id: String) -> Result<T, Error>;
    async fn update<'a>(&self, item: &'a T) -> Result<T, Error>;
    async fn delete(&self, id: String) -> Result<T, Error>;
}