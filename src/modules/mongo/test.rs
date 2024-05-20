use crate::modules::mongo::service::MongoOracle;

#[tokio::test]
async fn test_new_mongo() {
    let mongo = MongoOracle::new().await;
    assert!(mongo.db_name.is_ascii());
    assert!(mongo.port.is_ascii());
    assert!(mongo.host.is_ascii());
}
