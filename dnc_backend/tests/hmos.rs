mod common;
use common::test_api;


#[tokio::test]
async fn get_hmos_admin(){
    test_api("admin@dnc.com.ph", "password", "hmos", true).await;
}
#[tokio::test]
async fn get_hmos_noperms(){
    test_api("noperms@dnc.com.ph", "noperms", "hmos", false).await;
}
