use dotenv::dotenv;
use dotenv::var;
use rocket::http::hyper::uri::Port;
use rocket_db_pools::mongodb::error::Error;
use rocket_db_pools::mongodb::{Collection, Client, Database};
pub struct MongoOracle {
    pub client: Client,
    uri : String,
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
        let client = Client::with_uri_str(uri.to_owned()).await.unwrap();
        let db = client.database(&db_name);
        //let dispensers: Collection<Dispenser> = db.collection("dispensers");
        //let tabs: Collection<Tab> = db.collection("tabs");
        Self { client, uri, db_name, db, host, port }
    }
}
