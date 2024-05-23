use dotenv;
use dotenv::var;
use log::info;
use rocket_db_pools::mongodb::{ Client, Database};
pub struct MongoOracle {
    pub client: Client,
    pub db_name : String,
    pub db: Database,
    pub host : String,
    pub port: String
}

impl MongoOracle {
    pub async fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");
        let username = var("MONGO_TEST_USER").expect("No MongoDB User has been set in ENV FILE");
        let host = var("MONGODB_HOST").expect("No MongoDB host has been set in ENV FILE");
        let password = var("MONGO_TEST_PASSWORD").expect("No MongoDB password has been set in ENV FILE");
        let db_name = var("MONGODB_DATABASE_NAME").expect("No MongoDB database name has been set in ENV FILE");
        let port = var("MONGODB_PORT").expect("No MongoDB port name has been set in ENV FILE");
        let uri = String::from("mongodb://") 
         + &username + &String::from(":") 
         + & password + &String::from("@")
         + &host + &String::from(":")
         +&port+ &String::from("/")
         + &db_name;
        let client_result = Client::with_uri_str(uri.to_owned()).await;
        let client = match client_result {
            Ok(cli) => {
                info!("Connected successfully to MongoDB");
                cli
            }
            Err(err) => panic!("Error when connecting to mongoDB: {}", err)
        };
        let db = client.database(&db_name);
        Self { client, db_name, db, host, port }
    }
}
