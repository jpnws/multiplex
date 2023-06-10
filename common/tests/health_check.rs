// The test covers the full range of properties we are interested in.
// - The health check is exposed at /health_check.
// - The health check is behind a GET method.
// - The health check always returns a 200 OK HTTP status code.
// - The health check always returns an empty body.
#[tokio::test]
async fn check_api_health() {
    // ====================================
    // Arrange
    // ====================================
    let socket_addr = common::spawn_app();
    let client = reqwest::Client::new();
    // ====================================
    // Act
    // ====================================
    let response = client
        .get(&format!("http://{}/health_check", &socket_addr))
        .send()
        .await
        .expect("Failed to execute request.");
    // ====================================
    // Assert
    // ====================================
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
