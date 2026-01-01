use axum::http::StatusCode;
use axum::response::{Html, Extension, IntoResponse};
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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhoAmIResponse{
    pub email:String,
    pub role_id: i32,
}
pub async fn whoami(Extension(user):  Extension<AuthUser>)->impl IntoResponse{
    Json(WhoAmIResponse{
        email: user.claims.email,
        role_id: user.claims.role_id
    })
}