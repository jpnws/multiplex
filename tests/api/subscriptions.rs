use crate::helpers::spawn_app;
use rstest::*;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // ====================================
    // Arrange
    // ====================================
    let app = spawn_app().await;
    let body = "name=penguin&email=penguin%40gmail.com";
    // ====================================
    // Act
    // ====================================
    let response = app.post_subscriptions(body.into()).await;
    // ====================================
    // Assert
    // ====================================
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "penguin@gmail.com");
    assert_eq!(saved.name, "penguin");
}

#[rstest]
#[case("name=le%20guin", "missing the email")]
#[case("email=ursula_le_guin%40gmail.com", "missing the name")]
#[case("", "missing both name and email")]
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(
    #[case] body: String,
    #[case] err_msg: String,
) {
    // ====================================
    // Arrange
    // ====================================
    let app = spawn_app().await;
    // ====================================
    // Act
    // ====================================
    let response = app.post_subscriptions(body).await;
    // ====================================
    // Assert
    // ====================================
    assert_eq!(
        400,
        response.status().as_u16(),
        // Additional customised error message on test failure
        "The API did not fail with 400 Bad Request when the payload was {}.",
        err_msg
    );
}

#[rstest]
#[case("name=&email=ursula_le_guin%40gmail.com", "empty name")]
#[case("name=Ursula&email=", "empty email")]
#[case("name=Ursula&email=definitely-not-an-email", "invalid email")]
#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid(
    #[case] body: String,
    #[case] err_msg: String,
) {
    // ====================================
    // Arrange
    // ====================================
    let app = spawn_app().await;
    // ====================================
    // Act
    // ====================================
    let response = app.post_subscriptions(body).await;
    // ====================================
    // Assert
    // ====================================
    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not return a 400 Bad Request when the payload was {}",
        err_msg
    );
}
