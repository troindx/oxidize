use std::io;

use rocket::{routes, Route};
use rocket::{get, http::Status, response::status, serde::json::Json, State};
use rocket_db_pools::mongodb::bson::oid::ObjectId;

use crate::framework::app::App;
use crate::modules::user::guard::OxidizeSession;

use super::dto::EmailVerification;

#[get("/mail/verifications/start-verification", format = "application/json")]
pub async fn start_verification(app: &State<App>, session: OxidizeSession) -> status::Custom<Json<Option<bool>>> {
    let verification = app.mail.start_verification(&session.user).await;
    match verification {
        Some(_) => status::Custom(Status::Ok, Json::from(Some(true))),
        None => status::Custom(Status::Conflict, Json::from(None))
    }
}

#[get("/mail/verifications/<id>/verify/<secret>", format = "application/json")]
pub async fn finish_verification(app: &State<App>, id:&str, secret: String, session: OxidizeSession
) -> status::Custom<Json<Option<EmailVerification>>> {
    let verification_id = ObjectId::parse_str(id);
    if verification_id.is_err() {
        return status::Custom(Status::BadRequest, Json::from(None));
    }
    match app.mail.finish_verification(&session.user._id.unwrap(), &verification_id.unwrap(), secret.as_str()).await {
        Ok(verif) => status::Custom(Status::Ok, Json::from(Some(verif))),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return status::Custom(Status::NotFound, Json::from(None));
            } else if err.kind() == io::ErrorKind::PermissionDenied {
                return status::Custom(Status::Unauthorized, Json::from(None));
            } else if err.kind() == io::ErrorKind::InvalidData{
                return status::Custom(Status::Conflict, Json::from(None));
            } else {
                return status::Custom(Status::InternalServerError, Json::from(None));}
        }
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![start_verification, finish_verification]
}