use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::{Request, request::{self, FromRequest, Outcome}};
use rocket::http::Status;
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use crate::{framework::{app::App, auth::Claims}, modules::CRUDMongo};
use super::dto::User;

pub struct UpdateAuthGuard{
    pub user_before_update : User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UpdateAuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user_id = request.param::<String>(1).and_then(|param| param.ok()).expect("No User ID in request");
        let result = ObjectId::parse_str(&user_id);
        let id = if result.is_ok() {
            result.unwrap()
        } else {
            return Outcome::Error((Status::BadRequest, ()));
        };
        let app = request.rocket().state::<App>().expect("Error retrieving app");
        let user = if let Some(user) = app.users.read(id).await {
            user
        } else {
            return Outcome::Error((Status::NotFound, ()));
        };

        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_value) = auth_header {
            if auth_value.starts_with("Bearer ") {
                let token = &auth_value[7..];

                // Retrieve the user's public key for token verification
                let user_public_key = user.public_key.clone();
                let decoding_key = DecodingKey::from_rsa_pem(user_public_key.as_bytes()).expect("Invalid public key");

                match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::RS512)) {
                    Ok(token_data) => {
                        if user_id != token_data.claims.user_id {
                            return Outcome::Error((Status::BadRequest, ()));
                        }
                        return Outcome::Success(UpdateAuthGuard {user_before_update:user.to_owned()});
                    }
                    Err(_) => {
                        return Outcome::Error((Status::Unauthorized, ()));
                    }
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
