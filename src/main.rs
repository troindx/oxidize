
#[macro_use] 
extern crate rocket;
use env_logger;
use oxidize::modules::mongo::service::MongoOracle;
use oxidize::modules::user::service::UserService;
use std::sync::Arc;

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    let mongo = MongoOracle::new().await;
    let arc_mongo = Arc::new(mongo);
    let user = UserService::new(arc_mongo);
    rocket::build()
                //.mount("/", routes![dispenser, tab, spending])
                .manage(user)
}
