use multiplex::routes::{BodyData, Content};
use rstest::*;
use uuid::Uuid;
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::{spawn_app, ConfirmationLinks, TestApp};

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange

    let app = spawn_app().await;

    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        // We assert that no request is fired at Postmark.
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = BodyData {
        title: String::from("Newsletter title"),
        content: Content::new(
            "<p>Newsletter body as HTML</p>",
            "Newsletter body as plain text",
        ),
    };

    let response = app.post_newsletter(&newsletter_request_body).await;

    // Assert

    assert_eq!(200, response.status().as_u16());

    // Mock verifies on Drop that we have not sent the newsletter email.
}

/// Use the public API of the app under test to create unconfirmed subscriber.
async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=penguin&email=penguin%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    // We now inspect the requests received by the mock Postmark server to
    // retrieve the confirmation link and return it.
    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    // We can then reuse the same helper and just add an extra step to actually
    // call the confirmation link.
    let confirmation_link = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // Arrange

    let app = spawn_app().await;

    create_confirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = BodyData {
        title: String::from("Newsletter title"),
        content: Content::new(
            "<p>Newsletter body as HTML</p>",
            "Newsletter body as plain text",
        ),
    };

    let response = app.post_newsletter(&newsletter_request_body).await;

    // Assert

    assert_eq!(200, response.status().as_u16());

    // Mock verifies on Drop that we have sent the newsletter email.
}

#[rstest]
#[case((
    BodyData {
        title: String::from(""),
        content: Content::new("<p>Newsletter body as HTML</p>", "Newsletter body as plain text"),
    },
    "Newsletter content must not be empty."
))]
#[case((
    BodyData {
        title: String::from(""),
        content: Content::new("", ""),
    },
    "Newsletter content must not be empty."
))]
#[tokio::test]
async fn newsletters_return_400_for_invalid_data(#[case] test_case: (BodyData, &str)) {
    let app = spawn_app().await;
    let (invalid_body, error_message) = test_case;
    let response = app.post_newsletter(&invalid_body).await;
    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message
    );
}

#[tokio::test]
async fn requests_missing_authorization_are_rejected() {
    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .post(&format!("{}/newsletters", &app.address))
        .json(&serde_json::json!({
            "title": "Newsletter title",
            "content": {
                "text": "Newsletter body as plaint text",
                "html": "<p>Newsletter body as HTML</p>",
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());

    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}

#[tokio::test]
async fn non_existing_user_is_rejected() {
    // Arrange
    let app = spawn_app().await;

    // Random credentials
    let username = Uuid::new_v4().to_string();
    let password = Uuid::new_v4().to_string();

    let response = reqwest::Client::new()
        .post(&format!("{}/newsletters", &app.address))
        .basic_auth(username, Some(password))
        .json(&serde_json::json!({
            "title": "Newsletter title",
            "content": {
                "text": "Newsletter body as plain text",
                "html": "<p>Newsletter body as HTML</p>",
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert

    assert_eq!(401, response.status().as_u16());

    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}

#[tokio::test]
async fn invalid_password_is_rejected() {
    // Arrange

    let app = spawn_app().await;

    // Existing username.
    let username = &app.test_user.username;

    // Random password.
    let password = Uuid::new_v4().to_string();

    assert_ne!(app.test_user.password, password);

    // Act

    let response = reqwest::Client::new()
        .post(&format!("{}/newsletters", &app.address))
        .basic_auth(username, Some(password))
        .json(&serde_json::json!({
            "title": "Newsletter title",
            "content": {
                "text": "Newsletter body as plain text",
                "html": "<p>Newsletter body as HTML</p>",
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert

    assert_eq!(401, response.status().as_u16());

    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}
