mod test { 
    use oxidize::framework::app::{create_rocket_instance, App};
    use oxidize::framework::auth::{generate_jwt_token, generate_rsa_key_pair_pem};
    use oxidize::framework::config::OxidizeConfig;
    use oxidize::framework:: testing::{Mock, TestingRuntime};
    use oxidize::framework::translator::OxidizeTranslator;
    use oxidize::modules::mail::service::MailOracle;
    use oxidize::modules::mongo::service::MongoOracle;
    use oxidize::modules::{user, CRUDMongo};
    use oxidize::modules::user::dto::User;
    use rocket::http::Header;
    use rocket::local::asynchronous::Client;
    use rocket::uri;
    use rocket_db_pools::mongodb::bson::oid::ObjectId;
    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_mail_verifications() {
        let config = Arc::new(OxidizeConfig::new().expect("Could not load oxidize config"));
        let mongo = Arc::new(MongoOracle::new(config.clone()).await);
        let translator = Arc::new(OxidizeTranslator::new(config.clone()));
        let mail = MailOracle::new(config, mongo.clone(), translator );
        let mut user = User::mock();
        user._id = Some(ObjectId::new());
        mongo.drop_database().await.expect("Error dropping database");
        mail.initialize_db().await.expect("Error while initializing database");

        //Step 1: Create a verification and insert it into the system
        let verification = mail.start_verification(&user).await.expect("Could not start email verification");
        assert!(verification._id.is_some());

        let verification_v = mail.find_verification_by_email(user.email.as_str()).await
            .expect("Failed to retrieve verification");
        assert!(verification_v._id == verification._id);
        assert!(verification_v.email == user.email);

        //Step 2a: The service does not validate uncorrect secret/mail combinations
        let check_error= mail.finish_verification(&user._id.unwrap(), &verification._id.unwrap(), "thisisnotasecret").await;
        assert!(check_error.is_err());

        //Step 2b: validate the verification and all OK.
        mail.finish_verification(&user._id.unwrap(), &verification._id.unwrap(), verification.secret.as_str()).await
            .expect("Error when finishing the verification");
    }

    #[tokio::test]
    async fn test_verification_endpoint() {
        let client = &TestingRuntime::get().await.client;
        let mut user = User::mock();
        let (public_key, secret_key) = generate_rsa_key_pair_pem();
        user.public_key = public_key;
        let rocket = client.rocket();
        let app = rocket.state::<App>().expect("Could not get app state");
        user._id = app.users.create(user.clone()).await.expect("Could not create user").inserted_id.as_object_id();

        let response = client.get(uri!(oxidize::modules::mail::controller::start_verification)).dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::Unauthorized);

        let token = generate_jwt_token(&user._id.expect("no user id").to_string(), &secret_key, chrono::Duration::hours(1)).expect("Error generating token");
        let auth_header = String::from("Bearer ") + token.as_str();

        //Step 1: Start verification
        let response = client.get(uri!(oxidize::modules::mail::controller::start_verification))
        .header(Header::new("Authorization", auth_header.clone()))
        .dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
        //check that verification exists
        let verification = app.mail.find_verification_by_user_id(&user._id.unwrap()).await.expect("Could not find verification");
        assert!(verification.email == user.email);
        assert!(verification.user_id == user._id.unwrap());

        //Step 2a: Finish verification with incorrect secret throws conflict
        let response = client.get(uri!(oxidize::modules::mail::controller::finish_verification(
            id=verification._id.unwrap().to_string(), 
            secret="sijssisj")))
        .header(Header::new("Authorization", auth_header.clone()))
        .dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::Conflict);

        //Step 2b: Finish verification with incorrect id throws not found
        let response = client.get(uri!(oxidize::modules::mail::controller::finish_verification(
            id=ObjectId::new().to_string(), 
            secret=verification.secret.clone())))
        .header(Header::new("Authorization", auth_header.clone()))
        .dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::NotFound);

        //Step 2c: Finish verification with an ID that is not an objectid throws bad request
        let response = client.get(uri!(oxidize::modules::mail::controller::finish_verification(
            id="Lalalalala", 
            secret=verification.secret.clone())))
        .header(Header::new("Authorization", auth_header.clone()))
        .dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::BadRequest);

        //Step 3: Finish verification all Ok with correct secret
        let response = client.get(uri!(oxidize::modules::mail::controller::finish_verification(
            id=verification._id.unwrap().to_string(), 
            secret=verification.secret)))
        .header(Header::new("Authorization", auth_header))
        .dispatch().await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
        let verification = app.mail.find_verification_by_user_id(&user._id.unwrap()).await.expect("Could not find verification");
        assert!(verification.verified);



    }
}

