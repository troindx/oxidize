use oxidize::modules::mongo::service::MongoOracle;
#[tokio::test]
async fn test_new_connection() {
    let mongo_oracle = MongoOracle::new().await;
    assert!(mongo_oracle.client.is_some());
    assert!(mongo_oracle.db.is_some());
}

#[tokio::test]
async fn test_close_connection() {
    let mut mongo_oracle = MongoOracle::new().await;
    mongo_oracle.close();
    assert!(mongo_oracle.client.is_none());
    assert!(mongo_oracle.db.is_none());
}
