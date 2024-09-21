mod common;

#[tokio::test]
async fn list_all_endpoint_works() {
    // Arrange
    let (address, _state) = common::spawn();
    let client = reqwest::Client::new();

    let ip_info_1 = serde_json::json!({
        "hostname": "example1.com",
        "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        "ip_v4": "192.168.0.1"
    });

    let ip_info_2 = serde_json::json!({
        "hostname": "example2.com",
        "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7335",
        "ip_v4": "192.168.0.2"
    });

    // Act
    let response_1 = client
        .post(format!("{}/ip", &address))
        .json(&ip_info_1)
        .send()
        .await
        .expect("Failed to execute request.");

    let response_2 = client
        .post(format!("{}/ip", &address))
        .json(&ip_info_2)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response_1.status().is_success());
    assert!(response_2.status().is_success());

    // Request the list_all endpoint
    let list_response = client
        .get(format!("{}/list_all", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(list_response.status().is_success());

    let list_response_body: serde_json::Value = list_response
        .json()
        .await
        .expect("Failed to parse response body.");

    // Compare the results
    let expected_response = serde_json::json!([ip_info_1, ip_info_2]);
    assert_eq!(list_response_body, expected_response);
}
