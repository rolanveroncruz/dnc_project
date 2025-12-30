mod common;
use common::test_api;

#[tokio::test]
async fn get_users_admin(){
    test_api("admin@dnc.com.ph", "password", "roles", true).await;
}

#[tokio::test]
async fn get_users_noperms(){
    test_api("noperms@dnc.com.ph", "noperms", "roles", false).await;
}
