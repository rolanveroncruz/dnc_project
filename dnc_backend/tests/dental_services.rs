mod common;
use common::test_api;

#[tokio::test]
async fn get_dental_services_admin(){
    test_api("admin@dnc.com.ph", "password", "dental_services", true).await;
}

#[tokio::test]
async fn get_dental_services_noperms() {
    test_api("noperms@dnc.com.ph", "noperms", "dental_services", false).await;
}
