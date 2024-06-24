mod test {
    use oxidize::framework::auth::{generate_jwt_token, generate_rsa_key_pair_pem};
    use oxidize::framework::config::OxidizeConfig;
    use oxidize::framework:: testing::TestingRuntime;
    use oxidize::framework::translator::OxidizeTranslator;
    use oxidize::modules::mail::service::MailOracle;
    use oxidize::modules::mongo::service::MongoOracle;
    use oxidize::modules::user::service::UserService;
    use oxidize::modules::{mail, CRUDMongo};
    use oxidize::modules::user::dto::User;
    use rocket::http::Header;
    use rocket_db_pools::mongodb::bson::oid::ObjectId;
    use std::sync::Arc;
    use tokio;
    use rocket::{http::{ContentType, Status}, local::asynchronous::LocalResponse, uri};
    use rocket::serde::json::json;

    #[tokio::test]
    async fn test_mail_verifications() {
        let config = Arc::new(OxidizeConfig::new().expect("Could not load oxidize config"));
        let mongo = Arc::new(MongoOracle::new(config.clone()).await);
        let translator = Arc::new(OxidizeTranslator::new(config.clone()));
        let mail = MailOracle::new(config, mongo.clone(), translator );
        let user = User {
            email: String::from("juash992@iddi.com"),
            password: String::from("atest_password2"),
            description: String::from("Test Description"),
            public_key: String::from("somepublickey"),
            role: 1,
            _id: None,
        };
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
        let check_error= mail.finish_verification(user.email.as_str(), "thisisnotasecret").await;
        assert!(check_error.is_err());

        //Step 2b: validate the verification and all OK.
        mail.finish_verification(user.email.as_str(), verification.secret.as_str()).await
            .expect("Error when finishing the verification");


    }
}