use std::sync::Arc;

use log::info;
use rocket_db_pools::mongodb::{ bson::Document, error::Error, Client, Database};
use std::sync::Mutex;

use crate::framework::config::OxidizeConfig;

pub struct MongoOracle {
    pub client: Option<Client>,
    pub db: Option<Database>,
    pub config: Arc<OxidizeConfig>,
    pub collections: Mutex<Vec<String>>,
}

impl MongoOracle {

    pub async fn drop_database(&self) -> Result<(), Error> {
        if let Some(db) = &self.db {
            let collections = self.collections.lock().unwrap().clone();
            for collection_name in collections {
                db.collection::<Document>(&collection_name).drop(None).await?;
            }
        }
        Ok(())
    }

    pub fn add_collection(&self, collection_name: &str) {
        let mut collections = self.collections.lock().unwrap();
        collections.push(collection_name.to_string());
    }

    pub fn remove_collection(&self, collection_name: &str) {
        let mut collections = self.collections.lock().unwrap();
        if let Some(index) = collections.iter().position(|c| c == collection_name) {
            collections.remove(index);
        }
    }

    pub async fn new( config : Arc<OxidizeConfig>) -> Self {
        let uri = format!("mongodb://{}:{}@{}:{}/{}", 
            config.env.mongo_test_user, 
            config.env.mongo_test_password, 
            config.env.mongodb_host, 
            config.env.mongodb_port, 
            config.env.mongodb_database_name);
        let client_result = Client::with_uri_str(&uri).await;
        let client = match client_result {
            Ok(cli) => {
                info!("Connected successfully to MongoDB");
                cli
            }
            Err(err) => panic!("Error when connecting to MongoDB: {}", err)
        };
        let db = client.database(&config.env.mongodb_database_name);
        Self { client: Some(client), db: Some(db), config,  collections: Mutex::new(vec![]), }
    }

    pub fn close(&mut self) {
        self.client = None;
        self.db = None;
        info!("Closed connection to MongoDB");
    }
}
