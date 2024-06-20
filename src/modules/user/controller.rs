use rocket::{delete, get, post, put};
use rocket::{routes, State};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use super::dto::{User, UserRoles};
use crate::modules::CRUDMongo;
use crate::framework::app::App;
use crate::modules::user::guard::UpdateAuthGuard;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use rocket::Route;

#[post("/user", format = "application/json", data = "<user>")]
pub async fn create_user(app: &State<App>, mut user: Json<User>) -> status::Custom<Json<Option<User>>> {
    user.0.role = UserRoles::USER as u8;
    let new_id = app.users.create(user.0.to_owned()).await;
    match new_id {
        Some(id) => {
            let mut created_user = user.0.to_owned();
            created_user._id = id.inserted_id.as_object_id();
            status::Custom(Status::Created, Json::from(Some(created_user)))
        }
        None => status::Custom(Status::Conflict, Json::from(None)) // Return 409 Conflict if the user already exists
    }
}

#[get("/user/<id>", format = "application/json")]
pub async fn read_user(app: &State<App>, id: String) -> status::Custom<Json<Option<User>>> {
    match ObjectId::parse_str(&id) {
        Ok(object_id) => {
            let user = app.users.read(object_id).await;
            match user {
                Some(user) => status::Custom(Status::Ok, Json(Some(user))),
                None => status::Custom(Status::NotFound, Json::from(None)),
            }
        }
        Err(_) => status::Custom(Status::BadRequest, Json::from(None)),
    }
}

#[get("/user/email/<email>", format = "application/json")]
pub async fn find_user_by_email(app: &State<App>, email: String) -> status::Custom<Option<Json<User>>> {
    let user = app.users.find_by_email(&email).await;
    match user {
        Some(user) => status::Custom(Status::Ok, Some(Json(user))),
        None => status::Custom(Status::NotFound, None),
    }
}

#[put("/user/<_id>", format = "application/json", data = "<user>")]
pub async fn update_user(app: &State<App>, _id:String,  user: Json<User>, _session: UpdateAuthGuard) -> status::Custom<Option<Json<User>>> {
    let updated_user = app.users.update(user.0.to_owned()).await;
    match updated_user {
        Some(_) => status::Custom(Status::Ok, Some(user)),
        None => status::Custom(Status::InternalServerError, None),
    }
}

#[delete("/user/<id>", format = "application/json")]
pub async fn delete_user(app: &State<App>, id: String ,_session: UpdateAuthGuard) -> status::Custom<Option<Json<ObjectId>>> {
    match ObjectId::parse_str(&id) {
        Ok(object_id) => {
            let delete_result = app.users.delete(object_id).await;
            match delete_result {
                Some(result) => {
                    if result.deleted_count > 0 {
                        status::Custom(Status::Ok, Some(Json(object_id)))
                    } else {
                        status::Custom(Status::NotFound, None)
                    }
                }
                None => status::Custom(Status::InternalServerError, None),
            }
        }
        Err(_) => status::Custom(Status::BadRequest, None),
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![create_user, delete_user, update_user, read_user, find_user_by_email]
}