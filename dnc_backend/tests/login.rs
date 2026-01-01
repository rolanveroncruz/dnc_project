mod common;

use dnc_backend::{LoginRequest, LoginResponse, Claims};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
#[tokio::test]
async fn test_login_admin(){
    // 1. Get the running server address (starts if needed)
    let addr = common::setup_server().await;

    //2. make a request using a client (reqwest is standard for this)
    let client = reqwest::Client::new();

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
    assert_eq!(response.status(), 200);

    // 3. Parse json response
    let login:LoginResponse = response.json().await.unwrap();

    // 4. Validate token signature + claims
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET missing");
    let validation = Validation::new(Algorithm::HS512);

    let token_data = decode::<Claims>(
        &login.token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ).expect("Invalid JWT token");

    // 5.  Assert specific claims values
    assert_eq!( token_data.claims.email, request.email);
}

use dnc_backend::handlers::WhoAmIResponse;
#[tokio::test]
async fn test_login_and_whoami(){
    // 1. Get the running server address (starts if needed)
    let addr = common::setup_server().await;

    //2. make a request using a client (reqwest is standard for this)
    let client = reqwest::Client::new();

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
    assert_eq!(response.status(), 200);

    // 3. Parse json response
    let login:LoginResponse = response.json().await.unwrap();

    // 4. Validate token signature + claims
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET missing");
    let validation = Validation::new(Algorithm::HS512);

    let token_data = decode::<Claims>(
        &login.token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ).expect("Invalid JWT token");
    assert_eq!( token_data.claims.email, request.email);


    let whoami_response = client
        .get(format!("http://{}/api/whoami", addr))
        .bearer_auth(&login.token)
        .send()
        .await
        .unwrap();
    assert_eq!(whoami_response.status(), 200);

    let whoami_response:WhoAmIResponse = whoami_response.json().await.unwrap();
    assert_eq!(whoami_response.email, request.email);
    assert_eq!(whoami_response.role_id, 1);
}

#[tokio::test]
async fn test_login_noperms(){
    // 1. Get the running server address (starts if needed)
    let addr = common::setup_server().await;

    //2. make a request using a client (reqwest is standard for this)
    let client = reqwest::Client::new();

    let request = LoginRequest{
        email: "noperms@dnc.com.ph".to_string(),
        password: "noperms".to_string()
    };

    let response = client
        .post(format!("http://{}/login", addr))
        .json(&request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // 3. Parse json response
    let login:LoginResponse = response.json().await.unwrap();

    // 4. Validate token signature + claims
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET missing");
    let validation = Validation::new(Algorithm::HS512);

    let token_data = decode::<Claims>(
        &login.token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ).expect("Invalid JWT token");

    // 5.  Assert specific claims values
    assert_eq!( token_data.claims.email, request.email);
}
