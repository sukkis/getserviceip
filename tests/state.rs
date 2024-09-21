mod common;

#[tokio::test]
async fn ip_endpoint_updates_state() {
    // Arrange
    let (address, state) = common::spawn();
    let client = reqwest::Client::new();
    let ip_info = serde_json::json!({
        "hostname": "example.com",
        "ip_v6": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
        "ip_v4": "192.168.0.1"
    });

    // Act
    let response = client
        .post(format!("{}/ip", &address))
        .json(&ip_info)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    // Verify the state
    let state_guard = state.lock().unwrap();
    assert_eq!(state_guard.len(), 1);
    assert_eq!(state_guard[0].hostname, "example.com");
    assert_eq!(
        state_guard[0].ip_v6,
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334"
    );
    assert_eq!(state_guard[0].ip_v4, "192.168.0.1");
}
