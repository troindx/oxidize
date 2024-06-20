#[macro_use] 
extern crate rocket;
use framework::app::create_rocket_instance;
use oxidize::framework;


#[launch]
async fn rocket() -> _ {
    create_rocket_instance(false).await
}


