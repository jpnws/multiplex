use newsletter::cfg::Settings;
use rstest::*;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // ====================================
    // Arrange
    // ====================================
    let socket_addr = multiplexer::spawn_app();
    let cfg = Settings::new().expect("Failed to read configuration.");
    let db_connection = PgConnection::connect(&cfg.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    // ====================================
    // Act
    // ====================================
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("http://{}/subscriptions", &socket_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // ====================================
    // Assert
    // ====================================
    assert_eq!(200, response.status().as_u16(), "{}", socket_addr);
}

// Table-driven test. Parametrized test. Using rstest crate.
#[rstest]
#[case("name=le%20guin", "missing the email")]
#[case("email=ursula_le_guin%40gmail.com", "missing the name")]
#[case("", "missing both name and email")]
#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing(
    #[case] invalid_body: String,
    #[case] error_message: String,
) {
    // ====================================
    // Arrange
    // ====================================
    let socket_addr = multiplexer::spawn_app();
    let cfg = Settings::new().expect("Failed to read configuration.");
    let db_connection = PgConnection::connect(&cfg.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    // ====================================
    // Act
    // ====================================
    let response = client
        .post(&format!("http://{}/subscriptions", &socket_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");
    // ====================================
    // Assert
    // ====================================
    assert_eq!(
        400,
        response.status().as_u16(),
        // Additional customised error message on test failure
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message
    );
}
