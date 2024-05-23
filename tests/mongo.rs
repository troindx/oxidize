use oxidize::modules::mongo::service::MongoOracle;

#[tokio::test]
async fn test_new_mongo() {
    let mongo = MongoOracle::new().await;
    assert!(mongo.port.is_ascii());
    assert!(mongo.host.is_ascii());
}
