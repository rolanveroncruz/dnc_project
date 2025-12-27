use dnc_backend::handlers::boiler::{TestJsonRequest, TestJsonResponse};

mod common;

#[tokio::test]
async fn test_health_check(){
    // 1. Get the running server address (starts if needed)
    let addr = common::setup_server().await;

    //2. make a request using a client (reqwest is standard for this)
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/healthcheck", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

}


#[tokio::test]
async fn test_hello(){
    // 1. Get the running server address (starts if needed)
    let addr = common::setup_server().await;

    //2. make a request using a client (reqwest is standard for this)
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/hello", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

}
#[tokio::test]
async fn test_test_posting(){
    let addr = common::setup_server().await;

    //2. make a request using a client (reqwest is standard for this)
    let client = reqwest::Client::new();
    let payload = TestJsonRequest{
        name: "Rolan".to_string(),
        message: "Hello, World".to_string(),
    };

    let response = client
        .post(format!("http://{}/test_post", addr))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let response_body:TestJsonResponse = response.json().await.expect("Failed to parse response body as JSON");
    println!("\n**********{:?}**********\n", response_body.message);
    assert_eq!(response_body.message, "Hi, Rolan! Hello, World!");

}