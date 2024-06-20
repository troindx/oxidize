use std::sync::Arc;
use crate::modules;
use modules::{mongo::service::MongoOracle, user::service::UserService, CRUDMongo};

pub struct App {
    pub users:UserService,
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
    let mongo = MongoOracle::new().await;
    let arc_mongo = Arc::new(mongo);
    if dev_mode {
        arc_mongo.drop_database().await.expect("Error dropping database");
    }
    let user_service = UserService::new(arc_mongo);
    if dev_mode {
        user_service.initialize_db().await.expect("Error initializing database");
    }
    let app : App = App { users: user_service};
    rocket::build()
        .mount("/", crate::modules::user::controller::get_routes())
        .manage(app)
}