// The test covers the full range of properties we are interested in.
// - The health check is exposed at /health_check.
// - The health check is behind a GET method.
// - The health check always returns a 200 OK HTTP status code.
// - The health check always returns an empty body.
#[tokio::test]
async fn test_check_health() {
    // ====================================
    // Arrange
    // ====================================
    let socket_addr = multiplexer::spawn_app();
    let client = reqwest::Client::new();
    // ====================================
    // Act
    // ====================================
    let response = client
        .get(&format!("http://{}/check_health", &socket_addr))
        .send()
        .await
        .expect("Failed to execute request.");
    // ====================================
    // Assert
    // ====================================
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
