use std::sync::Arc;
use crate::{framework::config::OxidizeConfig, modules::mongo::service::MongoOracle};
use dotenv::dotenv;
use rocket_db_pools::mongodb::bson::Document;

#[tokio::test]
async fn test_add_and_remove_collections() {
    dotenv().ok();

    let config = Arc::new(OxidizeConfig::new().expect("Failed to load config"));
    let mongo = MongoOracle::new(config.clone()).await;

    mongo.add_collection("test_collection_1");
    mongo.add_collection("test_collection_2");

    {
        let collections = mongo.collections.lock().unwrap();
        assert!(collections.contains(&"test_collection_1".to_string()));
        assert!(collections.contains(&"test_collection_2".to_string()));
    }

    mongo.remove_collection("test_collection_1");

    {
        let collections = mongo.collections.lock().unwrap();
        assert!(!collections.contains(&"test_collection_1".to_string()));
        assert!(collections.contains(&"test_collection_2".to_string()));
    }
}

#[tokio::test]
async fn test_drop_database() {
    dotenv().ok();

    let config = Arc::new(OxidizeConfig::new().expect("Failed to load config"));
    let mongo = MongoOracle::new(config.clone()).await;

    // Create test collections
    if let Some(db) = &mongo.db {
        db.collection::<Document>("test_collection_1").insert_one(Document::new(), None).await.unwrap();
        db.collection::<Document>("test_collection_2").insert_one(Document::new(), None).await.unwrap();
    }

    // Add collections to drop
    mongo.add_collection("test_collection_1");
    mongo.add_collection("test_collection_2");

    // Drop the specified collections
    mongo.drop_database().await.unwrap();

    // Verify collections no longer exist
    if let Some(db) = &mongo.db {
        let collections: Vec<String> = db.list_collection_names(None).await.unwrap();
        assert!(!collections.contains(&"test_collection_1".to_string()));
        assert!(!collections.contains(&"test_collection_2".to_string()));
    }
}

#[tokio::test]
async fn test_new_mongo() {
    let config = OxidizeConfig::new().expect("Error reading env variables");
    let mongo = MongoOracle::new(Arc::new(config)).await;
    assert!(mongo.config.env.mongo_test_password.is_ascii());
    assert!(mongo.config.env.mongo_test_user.is_ascii());
    assert!(mongo.config.env.mongo_test_password.is_ascii());
    assert!(mongo.config.env.mongodb_host.is_ascii());

}
