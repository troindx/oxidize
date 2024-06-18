mod test {
    use oxidize::libs::testing::TestingRuntime;
    use oxidize::modules::mongo::service::MongoOracle;
    use oxidize::modules::user::service::UserService;
    use oxidize::modules::CRUDMongo;
    use oxidize::modules::user::dto::User;
    use rocket_db_pools::mongodb::bson::oid::ObjectId;
    use std::sync::Arc;
    use tokio;
    use rocket::{http::{ContentType, Status}, local::asynchronous::LocalResponse, uri};
    use rocket::serde::json::json;

    #[tokio::test]
    async fn test_user_service_crud_operations() {
        let mongo_oracle = Arc::new(MongoOracle::new().await);
        mongo_oracle.drop_database().await.expect("Error dropping database");
        let user_service = UserService::new(mongo_oracle);

        let user = User {
            email: String::from("Atest_user2@example.com"),
            password: String::from("atest_password2"),
            description: String::from("Test Description"),
            role: 2,
            _id: None,
        };

    // Test Create Operation
    let new_user_result = user_service.create(user.to_owned()).await.expect("Failed to create user");
    println!("Inserted ID: {:?}", new_user_result.inserted_id);
    let length = new_user_result.inserted_id.as_object_id().unwrap().to_hex().len();
    assert!(length == 24, "ObjectId should be 24 characters long");
    let user_id = new_user_result.inserted_id.as_object_id().expect("could not convert objectID");

    // Test Read Operation
    let retrieved_user = user_service.read(user_id.to_owned()).await.expect("Failed to read user");
    assert_eq!(retrieved_user.email, user.email);
    assert_eq!(retrieved_user.password, user.password);
    assert_eq!(retrieved_user.description, user.description);
    assert_eq!(retrieved_user.role, user.role);

    // Test find By Email Operation
    let email_user = user_service.find_by_email(user.email.as_str()).await.expect("Failed to find user by email");
    assert_eq!(email_user.email, user.email);

    // Test Update Operation
    let mut updated_user = retrieved_user.clone();
    updated_user.description = String::from("Updated Description");
    updated_user.email = String::from("new_email@gmail.com");
    let updated_user_result = user_service.update(updated_user.to_owned()).await.expect("Failed to update user");
    assert_eq!(updated_user_result.modified_count, 1);

    // retest find By Email Operation
    let retrieved_user = user_service.find_by_email(updated_user.email.as_str()).await.expect("Failed to find the updated user by email");
    assert_eq!(retrieved_user.email, updated_user.email);

    //find by email returns none on a non-existent user with a non existent email.
    let retrieved_user = user_service.find_by_email(String::from("thisemaildoesnotexist@gmail.com").as_str()).await;
    assert_eq!(retrieved_user.is_none(), true);

    // Test update Operation updated correctly everything
    let retrieved_user = user_service.read(user_id.to_owned()).await.expect("Failed to read updated user");
    assert_eq!(retrieved_user.description, String::from("Updated Description"));

    // Test Delete Operation
    let delete_res = user_service.delete(user_id.clone()).await.expect("Failed to delete user");
    assert_eq!(delete_res.deleted_count, 1);

    // Verify Deletion
    let deleted_user = user_service.read(user_id.clone()).await;
    assert!(deleted_user.is_none());
}

#[tokio::test]
async fn test_user_controller_crud_operations() {
    let client = &TestingRuntime::get().await.client;


    // Step 1: Create a new user
    let user = User {
        email: String::from("an_example_user@example.com"),
        password: String::from("test_password2"),
        description: String::from("Test Description 2"),
        role: 2,
        _id: None,
    };
    
    let create_response: LocalResponse = client.post(uri!(oxidize::modules::user::controller::create_user))
        .header(ContentType::JSON)
        .body(json!(user).to_string())
        .dispatch().await;
    
    assert_eq!(create_response.status(), Status::Created);
    let created_user: Option<User> = create_response.into_json().await;
    assert!(created_user.is_some());
    let created_user = created_user.unwrap();
    assert_eq!(created_user.email, user.email);

    //Step 1b : Will not create a new user with an existing email, (will not create same user twice)
    let existing_user_create_response: LocalResponse = client.post(uri!(oxidize::modules::user::controller::create_user))
        .header(ContentType::JSON)
        .body(json!(user).to_string())
        .dispatch().await;
    
    assert_eq!(existing_user_create_response.status(), Status::Conflict);
    let non_existing_user: Option<User> = existing_user_create_response.into_json().await;
    assert!(non_existing_user.is_none());

    // Step 2: Read the user by ID
    let user_id = created_user._id.clone().expect("User ID should be present");
    let read_response: LocalResponse = client.get(uri!(oxidize::modules::user::controller::read_user(user_id.to_hex())))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(read_response.status(), Status::Ok);
    let read_user: Option<User> = read_response.into_json().await;
    assert!(read_user.is_some());
    assert_eq!(read_user.unwrap().email, user.email);

    // Step 2b: Returns 400 bad request if id is not an objectId
    let read_response: LocalResponse = client.get(uri!(oxidize::modules::user::controller::read_user(String::from("thisisnotanemail"))))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(read_response.status(), Status::BadRequest);

    // Step 2c: Returns 404 if id is not found
    let read_response: LocalResponse = client.get(uri!(oxidize::modules::user::controller::read_user(ObjectId::new().to_string())))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(read_response.status(), Status::NotFound);

    // Step 3: Find the user by email
    let find_response: LocalResponse = client.get(uri!(oxidize::modules::user::controller::find_user_by_email(&user.email)))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(find_response.status(), Status::Ok);
    let found_user: Option<User> = find_response.into_json().await;
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().email, user.email);

    // Step 3C: Find the user by email returns 404 if 
    let find_response: LocalResponse = client.get(uri!(oxidize::modules::user::controller::find_user_by_email(&user.email)))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(find_response.status(), Status::Ok);

    // Step 4: Update the user's information
    let updated_user = User {
        email: user.email.clone(),
        password: String::from("updated_password"),
        description: String::from("Updated Description"),
        role: 1,
        _id: Some(user_id.clone()),
    };

    let update_response: LocalResponse = client.put(uri!(oxidize::modules::user::controller::update_user))
        .header(ContentType::JSON)
        .body(json!(updated_user).to_string())
        .dispatch().await;

    assert_eq!(update_response.status(), Status::Ok);
    let updated_user_response: Option<User> = update_response.into_json().await;
    assert!(updated_user_response.is_some());
    let updated_user_response = updated_user_response.unwrap();
    assert_eq!(updated_user_response.description, "Updated Description");

    // Step 5: Delete the user
    let delete_response: LocalResponse = client.delete(uri!(oxidize::modules::user::controller::delete_user(user_id.to_hex())))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(delete_response.status(), Status::Ok);
    let deleted_id: Option<ObjectId> = delete_response.into_json().await;
    assert!(deleted_id.is_some());
    assert_eq!(deleted_id.unwrap(), user_id);

    // Ensure the user is no longer available
    let read_deleted_response: LocalResponse = client.get(uri!(oxidize::modules::user::controller::read_user(user_id.to_hex())))
        .header(ContentType::JSON)
        .dispatch().await;

    assert_eq!(read_deleted_response.status(), Status::NotFound);
}


}
