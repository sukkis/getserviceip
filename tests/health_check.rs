mod common;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = common::spawn();

    // Act
    let response = reqwest::get(&format!("{}/health_check", &address))
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.text().await.unwrap(), "OK");
}
