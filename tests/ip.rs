use actix_web::{test, App};
use getserviceip::ip; // Adjust the import path according to your project structure

#[tokio::test]
async fn test_ip() {
    let app = test::init_service(App::new().service(ip)).await;
    let req = test::TestRequest::post()
        .uri("/ip")
        .set_json(serde_json::json!({
            "hostname": "example.com",
            "IPv6": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
            "IPv4": "192.168.0.1"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result["hostname"], "example.com");
    assert_eq!(result["IPv6"], "2001:0db8:85a3:0000:0000:8a2e:0370:7334");
    assert_eq!(result["IPv4"], "192.168.0.1");
}
