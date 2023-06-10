mod common;

#[tokio::test]
async fn test_greeting() {
    // ====================================
    // Arrange
    // ====================================
    let socket_addr = common::spawn_app();
    let client = reqwest::Client::new();
    // ====================================
    // Act
    // ====================================
    let response = client
        .get(format!("http://{}/greeting/World", socket_addr))
        .send()
        .await
        .expect("Failed to execute request.");
    // ====================================
    // Assert
    // ====================================
    assert!(response.status().is_success());
    let body = response.text().await.expect("Failed to retrieve body.");
    assert_eq!("Hello World!", body);
}
