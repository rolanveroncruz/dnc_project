mod common;
use serde::Serialize;

#[derive(Serialize)]
struct CreateUserRequest {
    email: String,
    password: String,
}
#[tokio::test]
async fn test_login(){
    // 1. Get the running server address (starts if needed)
    let addr = common::setup_server().await;

    //2. make a request using a client (reqwest is standard for this)
    let client = reqwest::Client::new();

    let body = CreateUserRequest {
        email: "admin@dnc.com.ph".to_string(),
        password: "password".to_string()
    };

    let response = client
        .post(format!("http://{}/login", addr))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

}
