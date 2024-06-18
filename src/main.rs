#[macro_use] 
extern crate rocket;
use libs::app::create_rocket_instance;
pub mod modules;
pub mod libs;

#[launch]
async fn rocket() -> _ {
    create_rocket_instance(false).await
}


