mod common;

#[tokio::test]
async fn ip_endpoint_works() {
    // Arrange
    let address = common::spawn();
    let client = reqwest::Client::new();
    let ip_info = serde_json::json!({
        "hostname": "example.com",
        "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        "ip_v4": "192.168.0.1"
    });

    // Act
    let response = client
        .post(&format!("{}/ip", &address))
        .json(&ip_info)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(response_body, ip_info);
}
