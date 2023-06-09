mod common;

#[tokio::test]
async fn test_greeting() {
    common::spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/greeting/World")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    let body = response.text().await.expect("Failed to retrieve body.");
    assert_eq!("Hello World!", body);
}
