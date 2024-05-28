
use oxidize::modules::mongo::service::MongoOracle;
use oxidize::modules::user::service::UserService;
use oxidize::modules::CRUDMongo;
use oxidize::modules::user::dto::User;
use rocket_db_pools::mongodb::bson::oid::ObjectId; 
use std::str::FromStr;
use std::sync::Arc;
use tokio;

#[tokio::test]
async fn test_user_service_crud_operations() {
    // Initialize MongoOracle
    let mongo_oracle = Arc::new(MongoOracle::new().await);

    // Initialize UserService
    let user_service = UserService::new(mongo_oracle);

    // Create a new user
    let user = User {
        email: String::from("test_user2@example.com"),
        password: String::from("test_password2"),
        description: String::from("Test Description2"),
        role: 1,
        _id: None,
    };

    // Test Create Operation
    let user_id = user_service.create(user.to_owned()).await.expect("Failed to create user");
    assert!(user_id.to_string().len() == 24, "ObjectId should be 24 characters long");
    assert!(ObjectId::from_str(&user_id.to_string()).is_ok(), "ObjectId should be a valid hex string");


    // Test Read Operation
    let retrieved_user = user_service.read(user_id.to_owned()).await.expect("Failed to read user");
    assert_eq!(retrieved_user.email, user.email);
    assert_eq!(retrieved_user.password, user.password);
    assert_eq!(retrieved_user.description, user.description);
    assert_eq!(retrieved_user.role, user.role);

    // Test Update Operation
    let mut updated_user = retrieved_user.clone();
    updated_user.description = String::from("Updated Description");
    let updated_user_result = user_service.update(updated_user.clone()).await.expect("Failed to update user");
    assert_eq!(updated_user_result.description, updated_user.description);

    // Test Delete Operation
    let delete_count = user_service.delete(user_id.clone()).await;
    assert_eq!(delete_count, 1);

    // Verify Deletion
    let deleted_user = user_service.read(user_id.clone()).await;
    assert!(deleted_user.is_none());
}