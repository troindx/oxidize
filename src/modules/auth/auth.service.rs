mod auth;
use jsonwebtoken::{decode, Validation, Algorithm, DecodingKey};
use rocket::{request::{FromRequest, Outcome}, Data, Request, http::{Status, RawStr}, serde::json::Json, data::{FromData, self}};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use crate::bar::Bar;
use super::models::{TabDTO, Dispenser, DispenserDTO, Claims};

#[derive(Debug)]
pub enum GuardError {
    NotFound,
    Invalid,
    MissingToken
}



#[rocket::async_trait]
impl<'r> FromRequest<'r> for Dispenser {
    type Error = GuardError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = req.rocket().state::<Bar>().unwrap();
        let query = req.uri().path();
        let params:Vec<&RawStr> = query.split("/").collect();
        let id = params[2].as_str();
        let dispenser = state.mongo.get_dispenser(ObjectId::parse_str(String::from(id)).unwrap()).await;
        if dispenser.is_none(){
            return Outcome::Failure((Status::NotFound, GuardError::NotFound));
        }       

        let token_o = req.headers().get_one("Authorization");
        if token_o.is_none(){
            return Outcome::Failure((Status::Unauthorized, GuardError::MissingToken));
        }
        let mut val = Validation::new(Algorithm::HS256);
        val.validate_exp = false;
        let token_message = decode::<Claims>(token_o.unwrap(), &DecodingKey::from_secret(dispenser.to_owned().unwrap().jwt_secret.as_str().as_ref()), &val);
        if token_message.is_ok(){
            
            return Outcome::Success(dispenser.unwrap());
        }
        else{
            return Outcome::Failure((Status::Unauthorized, GuardError::Invalid));
        }
    }
}
