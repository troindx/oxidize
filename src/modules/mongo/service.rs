use dotenv;
use dotenv::var;
use log::info;
use rocket_db_pools::mongodb::{ bson, error::Error, Client, Database};

pub struct MongoOracle {
    pub client: Option<Client>,
    pub db_name: String,
    pub db: Option<Database>,
    pub host: String,
    pub port: String,
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

    pub async fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");
        let username = var("MONGO_TEST_USER").expect("No MongoDB User has been set in ENV FILE");
        let host = var("MONGODB_HOST").expect("No MongoDB host has been set in ENV FILE");
        let password = var("MONGO_TEST_PASSWORD").expect("No MongoDB password has been set in ENV FILE");
        let db_name = var("MONGODB_DATABASE_NAME").expect("No MongoDB database name has been set in ENV FILE");
        let port = var("MONGODB_PORT").expect("No MongoDB port name has been set in ENV FILE");
        let uri = format!("mongodb://{}:{}@{}:{}/{}", username, password, host, port, db_name);
        let client_result = Client::with_uri_str(&uri).await;
        let client = match client_result {
            Ok(cli) => {
                info!("Connected successfully to MongoDB");
                cli
            }
            Err(err) => panic!("Error when connecting to MongoDB: {}", err)
        };
        let db = client.database(&db_name);
        Self { client: Some(client), db_name, db: Some(db), host, port }
    }

    pub fn close(&mut self) {
        self.client = None;
        self.db = None;
        info!("Closed connection to MongoDB");
    }
}
