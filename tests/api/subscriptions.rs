use crate::helpers::spawn_app;
use rstest::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        // We are not setting an expectation here anymore.
        // The test is focused on another aspect of app behavior.
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body.into()).await;

    // Assert
    // Get the first intercepted request.
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    // Parse the body as JSON, starting from the raw bytes.
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
    // Extract the link from one of the request fields.
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let html_link = get_link(body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(body["TextBody"].as_str().unwrap());

    // The two links should be identical.
    assert_eq!(html_link, text_link);
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body.into()).await;

    // Assert
    // Mock asserts on drop.
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=penguin&email=penguin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let response = app.post_subscriptions(body.into()).await;

    // Assert
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
