use std::sync::Arc;
use crate::modules::{self, mail::service::MailOracle};
use modules::{mongo::service::MongoOracle, user::service::UserService, CRUDMongo};

use super::config::OxidizeConfig;

pub struct App {
    pub users:UserService,
    pub config: Arc<OxidizeConfig>,
    pub mail: Arc<MailOracle>
}

/// Creates a valid rocket instance. Input true or false for development mode (testing)
/// ```
/// # #[tokio::main]
/// # async fn main() {
///     use rocket::local::asynchronous::Client;
///     use oxidize::framework::app::create_rocket_instance;
///     let rocket = create_rocket_instance(false).await;
///     let client = Client::tracked(rocket).await.expect("valid rocket instance");
/// # }
/// ```

pub async fn create_rocket_instance(dev_mode: bool) -> rocket::Rocket<rocket::Build> {
    env_logger::init();
    let config = Arc::new(OxidizeConfig::new().expect("Failed to load ENV VARIABLES"));
    let mongo = Arc::new(MongoOracle::new(config.clone()).await);
    let users = UserService::new(mongo.clone());
    if dev_mode {
        mongo.drop_database().await.expect("Error dropping database");
        users.initialize_db().await.expect("Error initializing database");
    }

    let mail = Arc::new(MailOracle::new(config.clone(),mongo.clone())); 

    
    let app : App = App { users, config:config.clone(), mail };
    rocket::build()
        .mount("/", crate::modules::user::controller::get_routes())
        .manage(app)
}

