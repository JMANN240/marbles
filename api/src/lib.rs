use serde::{Deserialize, Serialize};

pub mod marble;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
}
