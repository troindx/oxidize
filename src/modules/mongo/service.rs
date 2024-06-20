use log::info;
use rocket_db_pools::mongodb::{ bson, error::Error, Client, Database};

use crate::framework::config::OxidizeConfig;

pub struct MongoOracle {
    pub client: Option<Client>,
    pub db: Option<Database>,
    pub config: OxidizeConfig,
}

impl MongoOracle {

    pub async fn drop_database(&self) -> Result<(), Error> {
        if let Some(db) = &self.db {
            let collections = db.list_collection_names(None).await?;
            for collection_name in collections {
                db.collection::<bson::Document>(&collection_name).drop(None).await?;
            }
        }
        Ok(())
    }

    pub async fn new( config : OxidizeConfig) -> Self {
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
        Self { client: Some(client), db: Some(db), config }
    }

    pub fn close(&mut self) {
        self.client = None;
        self.db = None;
        info!("Closed connection to MongoDB");
    }
}
