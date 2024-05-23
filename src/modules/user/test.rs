use rocket_db_pools::mongodb::bson::doc;

use crate::modules::{mongo::service::MongoOracle, user::service::UserService};



#[tokio::test]
async fn test_new_user_service() {
    let mongo:MongoOracle = MongoOracle::new().await;
    let service:UserService = UserService::new(mongo.into());
    let count_result = service.users.count_documents(doc! {}, None).await;
    match count_result {
        Ok(count) => assert_eq!(count, 0),
        Err(err) => panic!("Error counting documents: {}", err)
    }
}
