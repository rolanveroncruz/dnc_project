
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
