use serde::{Deserialize, Serialize};
use jsonwebtoken::{DecodingKey, Validation};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims{
    pub sub:i32, // subject: user id
    pub email:String,
    pub role_id: i32,
    pub exp:usize // expiration timestamp
}

#[derive(Clone)]
pub struct JwtConfig {
    pub decoding_key: DecodingKey,
    pub validation: Validation,
}
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub claims: Claims,
}

use serde_with::{ serde_as, DisplayFromStr};
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub q: Option<String>,
    pub active: Option<bool>,
    pub sort: Option<String>,
    pub order: Option<String>,
    #[serde_as(as="Option<DisplayFromStr>")]
    pub page: Option<u64>,

    #[serde(rename = "pageSize")]
    #[serde_as(as="Option<DisplayFromStr>")]
    pub page_size: Option<u64>,
}
#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub page: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

use axum::http::StatusCode;
pub struct AppError(StatusCode, String);
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status: {}  Message: {}", self.0, self.1)
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}