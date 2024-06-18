use async_once_cell::OnceCell;
use rocket::local::asynchronous::Client;

use super::app::create_rocket_instance;
pub struct TestingRuntime {
    pub client: Client
}

impl TestingRuntime{
    async fn new() -> Self {
        let rocket = create_rocket_instance(true).await;
        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        Self { client }
    }

    pub async fn get() -> &'static TestingRuntime {
        TEST_RUNTIME.get_or_init(async {
            TestingRuntime::new().await
        }).await
    }
}

static TEST_RUNTIME: OnceCell<TestingRuntime> = OnceCell::new();


