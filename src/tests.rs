use super::rocket;
use chrono::Utc;
use chrono_tz::Tz;
use dispenser_api::bar::models::Claims;
use dispenser_api::bar::models::SpendingDTO;
use dispenser_api::bar::models::TabDTO;

use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use rocket::http::{ContentType, Status};
use dispenser_api::bar::models::{Dispenser, DispenserDTO};
use rocket::serde::json::{json, Json};
use serde_json::Value;
use jsonwebtoken::{encode, Algorithm, Header as jwtHeader, EncodingKey};

#[tokio::test]
async fn api_add_dispenser() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    let dispenser = DispenserDTO{ jwt_secret: "alalala".to_string(), flow_volume:0.5, id:None};
    let mut response: LocalResponse = client.post(uri!(super::dispenser))
        .header(ContentType::JSON)
        .body(json!(dispenser).to_string())
        .dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    let returned_dispenser : Option<Dispenser> = response.into_json().await;
    assert!(returned_dispenser.is_some());
    assert_eq!(returned_dispenser.unwrap().flow_volume, 0.5);
}

#[tokio::test]
async fn api_tab() {
    let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
    let mut dispenser = DispenserDTO{ jwt_secret: String::from("some secret"), flow_volume:0.5, id:None};   
    let mut response: LocalResponse = client.post(uri!(super::dispenser))
        .header(ContentType::JSON)
        .body(json!(dispenser).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let returned_dispenser : Option<Dispenser> = response.into_json().await;
    assert!(returned_dispenser.is_some());
    let dispenser_id = returned_dispenser.unwrap()._id;
    dispenser.id = Some(dispenser_id.unwrap().to_string());

    //Claims is the data that we are going to unwrap in JWT tokens.
    let claims = Claims{exp: 0};
    let token = encode(&jsonwebtoken::Header::default(), &claims, &EncodingKey::from_secret("some secret".as_ref())).unwrap();
    assert!(!token.is_empty());
    //If we don't send it the right DTO, it will fail
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(dispenser).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::UnprocessableEntity);


    //If we try to close a closed tab, it will return http:: conflict
    let tz = Tz::UTC;
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "close".to_string(),
        updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
    };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Conflict);

    //If we try to put anything in status that is no "open or close" ..> error 400 bad request.
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "this is neither open nor closed".to_string(),
        updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
    };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::BadRequest);

    //Without Auth token, error 401
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "open".to_string(),
        updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
    };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);


    //If we try to close a non existent tab, it will return http:: not found
    let url = format!("/dispenser/{}/status", "507f191e810c19729de860ea".to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "close".to_string(),
        updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
    };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::NotFound);


    //Finally, we try to create a new tab with the proper req
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "open".to_string(),
         updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
     };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Accepted);

    //We close the tab
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "close".to_string(),
         updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
     };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Accepted);

    //We get the spendings, there should be only one spending.
    let url = format!("/dispenser/{}/spending", dispenser_id.unwrap().to_string()); 
    let mut response: LocalResponse = client.get(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let spendings : Option<SpendingDTO> = response.into_json().await;
    assert!(spendings.is_some());
    assert_eq!(spendings.unwrap().usages.len(), 1); // <-- here

    //We create a new tab
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "open".to_string(),
         updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
     };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Accepted);

    //We close the new tab
    let url = format!("/dispenser/{}/status", dispenser_id.unwrap().to_string()); 
    let status_dto:TabDTO = TabDTO{
        status: "close".to_string(),
         updated_at :  Utc::now().with_timezone(&tz).to_rfc3339()
     };
    let mut response: LocalResponse = client.put(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .body(json!(status_dto).to_string())
        .dispatch().await;
    assert_eq!(response.status(), Status::Accepted);

     //two spendings.
    let url = format!("/dispenser/{}/spending", dispenser_id.unwrap().to_string()); 
    let mut response: LocalResponse = client.get(url)
        .header(ContentType::JSON)
        .header(rocket::http::Header::new("Authorization", token.to_owned()))
        .dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let spendings : Option<SpendingDTO> = response.into_json().await;
    assert!(spendings.is_some());
    assert_eq!(spendings.unwrap().usages.len(), 2); // <-- here

}