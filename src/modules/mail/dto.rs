use chrono::{DateTime, Utc};
use rocket_db_pools::mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct EmailVerification {
    pub email:String,
    pub secret:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub created: DateTime<Utc>,
    pub verified: bool
}