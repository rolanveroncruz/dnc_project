mod common;
use common::test_api;

#[tokio::test]
async fn get_role_permissions_admin(){
    test_api("admin@dnc.com.ph", "password", "role_permissions", true).await;
}

#[tokio::test]
async fn get_role_permissions_noperms(){
    test_api("noperms@dnc.com.ph", "noperms", "role_permissions", false).await;
}
