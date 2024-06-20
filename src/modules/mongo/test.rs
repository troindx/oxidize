use crate::{framework::config::OxidizeConfig, modules::mongo::service::MongoOracle};

#[tokio::test]
async fn test_new_mongo() {
    let config = OxidizeConfig::new().expect("Error reading env variables");
    let mongo = MongoOracle::new(config).await;
    assert!(mongo.config.env.mongo_test_password.is_ascii());
    assert!(mongo.config.env.mongo_test_user.is_ascii());
    assert!(mongo.config.env.mongo_test_password.is_ascii());
    assert!(mongo.config.env.mongodb_host.is_ascii());

}
