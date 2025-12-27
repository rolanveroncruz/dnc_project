use http::StatusCode;
use dnc_backend::{LoginRequest, LoginResponse};
use serde_json::Value;
mod common;

#[tokio::test]
async fn get_all_dental_services(){
    // 1. Get the running server address (starts if needed). Also, prepare a client.
    let addr = common::setup_server().await;
    let client = reqwest::Client::new();

    // 2. Prepare login request and login.
    let request = LoginRequest{
        email: "admin@dnc.com.ph".to_string(),
        password: "password".to_string()
    };

    let response = client
        .post(format!("http://{}/login", addr))
        .json(&request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Parse json response
    let login:LoginResponse = response.json().await.unwrap();


    // 4. Get all dental services
    let response = client
        .get(format!("http://{}/api/dental_services", addr))
        .bearer_auth(&login.token)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let dental_services_list:Vec<Value> = response.json().await.unwrap();
    assert!(dental_services_list.len() > 0);

}
