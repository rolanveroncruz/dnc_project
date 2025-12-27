use axum::http::StatusCode;
use axum::response::Html;
use serde::{ Serialize, Deserialize};

pub async fn hello_world()->Html<&'static str>{
    Html("<h1> Hello from Axum v0.8.7 </h1>!")
}

pub async fn healthcheck()->StatusCode{
    StatusCode::OK
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestJsonRequest{
    pub name:String,
    pub message:String
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestJsonResponse{
    pub message:String
}
use axum::extract::{Json, State};
use crate::AppState;
use crate::handlers::structs::AuthUser;

pub async fn test_posting_json(State(_state): State<AppState>, Json(payload): Json<TestJsonRequest>) ->Json<TestJsonResponse>{
    let combined_message = format!("Hi, {}! {}!", payload.name, payload.message);
    Json(TestJsonResponse{message: combined_message})
}
pub async fn whoami(user:  AuthUser)->String{
    format!("Hello, email={}, role={}!", user.claims.email, user.claims.role_id)
}