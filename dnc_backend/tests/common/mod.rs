// tests/common/mod.rs
use std::net::SocketAddr;
use http::StatusCode;
use tokio::net::TcpListener;
use dnc_backend::{LoginRequest, LoginResponse};

pub async fn setup_server() -> SocketAddr {
    let app = dnc_backend::build_app(dnc_backend::AppState::new().await);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    addr
}

#[allow(dead_code)]
pub async fn test_api(email: &str, password: &str, api: &str, expected_contents:bool) {

    // 1. Get the running server address (starts if needed). Also, prepare a client.
    let addr = setup_server().await;
    let client = reqwest::Client::new();

    // 2. Prepare login request and login.
    let request = LoginRequest {
        email: email.to_string(),
        password: password.to_string()
    };

    let response = client
        .post(format!("http://{}/login", addr))
        .json(&request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Parse json response
    let login: LoginResponse = response.json().await.unwrap();


    // 4. Get all dental services
    let page_size = 5;
    let response = client
        .get(format!("http://{}/api/{api}?page=1&pageSize={page_size}", addr))
        .bearer_auth(&login.token)
        .send()
        .await
        .unwrap();
    let status = response.status();
    if expected_contents {
        let bytes = response.bytes().await.unwrap();
        let raw = String::from_utf8_lossy(&bytes);
        println!("status= {status}");
        println!("raw =  {raw}");
        assert!(status.is_success(), "request failed: {status} body={raw}");

        let v: serde_json::Value = serde_json::from_slice(&bytes).expect("response is not valid JSON");
        let items = v.get("items")
            .and_then(|x| x.as_array())
            .expect("response does not contain items array");

        assert!(!items.is_empty(), "expected at least one item in the list");
    } else {
        assert_eq!(status, StatusCode::FORBIDDEN, "request failed: {status}");
    }
}

