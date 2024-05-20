mod auth;
use jsonwebtoken::{decode, Validation, Algorithm, DecodingKey};
use rocket::{request::{FromRequest, Outcome}, Data, Request, http::{Status, RawStr}, serde::json::Json, data::{FromData, self}};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use crate::super::user;

#[derive(Debug)]
pub enum GuardError {
    NotFound,
    Invalid,
    MissingToken
}
