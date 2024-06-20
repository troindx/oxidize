use oxidize::{framework::config::OxidizeConfig, modules::mongo::service::MongoOracle};
#[tokio::test]
async fn test_new_connection() {
    let config = OxidizeConfig::new().expect("Could not load env variables");
    let mongo_oracle = MongoOracle::new(config).await;
    assert!(mongo_oracle.client.is_some());
    assert!(mongo_oracle.db.is_some());
}

#[tokio::test]
async fn test_close_connection() {
    let config = OxidizeConfig::new().expect("Could not load env variables");
    let mut mongo_oracle = MongoOracle::new(config).await;
    mongo_oracle.close();
    assert!(mongo_oracle.client.is_none());
    assert!(mongo_oracle.db.is_none());
}
