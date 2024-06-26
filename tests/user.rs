mod test {
    use oxidize::framework::auth::{generate_jwt_token, generate_rsa_key_pair_pem};
    use oxidize::framework::config::OxidizeConfig;
    use oxidize::framework:: testing::{Mock, TestingRuntime};
    use oxidize::modules::mongo::service::MongoOracle;
    use oxidize::modules::user::service::UserService;
    use oxidize::modules::CRUDMongo;
    use oxidize::modules::user::dto::User;
    use rocket::http::Header;
    use rocket_db_pools::mongodb::bson::oid::ObjectId;
    use std::sync::Arc;
    use tokio;
    use rocket::{http::{ContentType, Status}, local::asynchronous::LocalResponse, uri};
    use rocket::serde::json::json;

    #[tokio::test]
    async fn test_user_service_crud_operations() {
        let config = Arc::new(OxidizeConfig::new().expect("Could not load oxidize config"));
        let mongo_oracle = Arc::new(MongoOracle::new(config).await);
        mongo_oracle.drop_database().await.expect("Error dropping database");
        let user_service = UserService::new(mongo_oracle);

        let user = User::mock();

    // Test Create Operation
    let new_user_result = user_service.create(user.to_owned()).await.expect("Failed to create user");
    println!("Inserted ID: {:?}", new_user_result.inserted_id);
    let length = new_user_result.inserted_id.as_object_id().unwrap().to_hex().len();
    assert!(length == 24, "ObjectId should be 24 charaPcters long");
    let user_id = new_user_result.inserted_id.as_object_id().expect("could not convert objectID");

    // Test Read Operation
    let retrieved_user = user_service.read(user_id.to_owned()).await.expect("Failed to read user");
    assert_eq!(retrieved_user.email, user.email);
    assert_eq!(retrieved_user.password, user.password);
    assert_eq!(retrieved_user.description, user.description);

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
        let (public, private) = generate_rsa_key_pair_pem();
        let (_, malicious_private) = generate_rsa_key_pair_pem();

        // Step 1: Create a new user
        let mut user = User::mock();
        user.public_key = public;
        
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

        // Step 4: Update the user's information without token returns 401
        let updated_user = User {
            email: user.email.clone(),
            password: String::from("updated_password"),
            description: String::from("Updated Description"),
            public_key: user.public_key.clone(),
            _id: Some(user_id.clone()),
        };

        let update_response: LocalResponse = client.put(uri!(oxidize::modules::user::controller::update_user(user_id.to_hex())))
            .header(ContentType::JSON)
            .body(json!(updated_user.to_owned()).to_string())
            .dispatch().await;
        assert_eq!(update_response.status(), Status::Unauthorized);

        // Step 4b: Update the user's information
        let token = generate_jwt_token(&user_id.to_string(), &private, chrono::Duration::hours(1)).expect("Error generating token");
        let auth_header = String::from("Bearer ") + token.as_str();
        let update_response: LocalResponse = client.put(uri!(oxidize::modules::user::controller::update_user(user_id.to_hex())))
            .header(ContentType::JSON)
            .header(Header::new("Authorization", auth_header))
            .body(json!(updated_user).to_string())
            .dispatch().await;

        assert_eq!(update_response.status(), Status::Ok);
        let updated_user_response: Option<User> = update_response.into_json().await;
        assert!(updated_user_response.is_some());
        let updated_user_response = updated_user_response.unwrap();
        assert_eq!(updated_user_response.description, "Updated Description");

        // Step 4c: updating fake user returns 404
        let token = generate_jwt_token(&user_id.to_string(), &private, chrono::Duration::hours(1)).expect("Error generating token");
        let auth_header = String::from("Bearer ") + token.as_str();
        let update_response: LocalResponse = client.put(uri!(oxidize::modules::user::controller::update_user(ObjectId::new().to_hex())))
            .header(ContentType::JSON)
            .header(Header::new("Authorization", auth_header.to_owned()))
            .body(json!(updated_user).to_string())
            .dispatch().await;

        assert_eq!(update_response.status(), Status::NotFound);

        // Step 4d: update user with wrong token generates 401
        let malicious_token = generate_jwt_token(&user_id.to_string(), &malicious_private, chrono::Duration::hours(1)).expect("Error generating token");
        let malicious_auth_header = String::from("Bearer ") + malicious_token.as_str();
        let updated_user = User {
            email: user.email.clone(),
            password: String::from("updated_password"),
            description: String::from("Updated Description"),
            public_key: user.public_key.clone(),
            _id: Some(user_id.clone()),
        };

        let update_response: LocalResponse = client.put(uri!(oxidize::modules::user::controller::update_user(user_id.to_hex())))
            .header(ContentType::JSON)
            .header(Header::new("Authorization", malicious_auth_header.to_owned()))
            .body(json!(updated_user.to_owned()).to_string())
            .dispatch().await;
        assert_eq!(update_response.status(), Status::Unauthorized);

        // Step 4e: Update the user with expired token returns 401
        let expired_token = generate_jwt_token(&user_id.to_string(), &private, chrono::Duration::hours(-1)).expect("Error generating token");
        let expired_auth_header = String::from("Bearer ") + expired_token.as_str();
        let update_response: LocalResponse = client.put(uri!(oxidize::modules::user::controller::update_user(user_id.to_hex())))
            .header(ContentType::JSON)
            .header(Header::new("Authorization", expired_auth_header))
            .body(json!(updated_user).to_string())
            .dispatch().await;

        assert_eq!(update_response.status(), Status::Unauthorized);

        // Step 5a: Delete the user returns 401 if unauthenticated
        let delete_response: LocalResponse = client.delete(uri!(oxidize::modules::user::controller::delete_user(user_id.to_hex())))
            .header(ContentType::JSON)
            .dispatch().await;

        assert_eq!(delete_response.status(), Status::Unauthorized);
        // Step 5b: Delete the user with wrong key is also rejected
        let delete_response: LocalResponse = client.delete(uri!(oxidize::modules::user::controller::delete_user(user_id.to_hex())))
            .header(ContentType::JSON)
            .header(Header::new("Authorization", malicious_auth_header))
            .dispatch().await;

        assert_eq!(delete_response.status(), Status::Unauthorized);
        // Step 5b: Delete the user with wrong key is also rejected
        let delete_response: LocalResponse = client.delete(uri!(oxidize::modules::user::controller::delete_user(user_id.to_hex())))
            .header(ContentType::JSON)
            .header(Header::new("Authorization", auth_header))
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
