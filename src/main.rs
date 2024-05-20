
#[macro_use] 
extern crate rocket;
mod modules;
use modules::mongo::service::MongoOracle;
use rocket::{State};
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;

#[launch]
async fn rocket() -> _ {
    let mongo = MongoOracle::new().await;
    let user = UserService::new(mongo).await;
    rocket::build()
                //.mount("/", routes![dispenser, tab, spending])
                .manage(user)
}
