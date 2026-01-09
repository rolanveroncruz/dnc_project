mod common;
use common::test_api;
#[tokio::test]
async fn get_data_objects_admin(){
    test_api("admin@dnc.com.ph", "password", "data_objects", true).await;
}
#[tokio::test]
async fn get_data_objects_noperms(){
    test_api("noperms@dnc.com.ph", "noperms", "data_objects", false).await;
}
