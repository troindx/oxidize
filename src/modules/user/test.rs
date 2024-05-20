use rocket_db_pools::mongodb::bson::doc;

use crate::modules::{mongo::service::MongoOracle, user::service::UserService};



#[tokio::test]
async fn test_new_UserService() {
    let mongo:MongoOracle = MongoOracle::new().await;
    let service:UserService = UserService::new(mongo).await;
    let count = service.users.count_documents(doc! {}, None).await?;
    assert!(count == 0);
}
