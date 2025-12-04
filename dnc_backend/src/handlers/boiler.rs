use axum::http::StatusCode;
use axum::response::Html;

pub async fn hello_world()->Html<&'static str>{
    Html("<h1> Hello from Axum v0.8.7 </h1>!")
}

pub async fn healthcheck()->StatusCode{
    StatusCode::OK
}