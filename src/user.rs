use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUser {
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}
