
mod common;
use common::test_api;


#[tokio::test]
async fn get_clinic_capabilities_admin(){
    test_api("admin@dnc.com.ph", "password", "clinic_capabilities", true).await;
}
#[tokio::test]
async fn get_clinic_capabilities_noperms(){
    test_api("noperms@dnc.com.ph", "noperms", "clinic_capabilities", false).await;
}
