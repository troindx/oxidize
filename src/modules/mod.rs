use async_trait::async_trait;

pub mod mongo;
pub mod user;

#[async_trait]
#[allow(dead_code)]
pub trait CRUD<T> {
    async fn create(&self, new: T) ->  Option<String>;
    async fn read(&self, id: String) -> Option<T>;
    async fn update(&self, item: T) -> Option<T>;
    async fn delete(&self, id: String) ->u64;
}