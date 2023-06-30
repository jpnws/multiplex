use uuid::Uuid;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn verify_that_a_user_gets_redirected_to_login_page_if_not_logged_in_when_accessing_publish_newsletter_form(
) {
    let app = spawn_app().await;
    let response = app.get_publish_newsletter().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn verify_that_a_user_gets_redirected_to_login_page_if_not_logged_in_when_submitting_a_newsletter(
) {
    let app = spawn_app().await;
    let response = app
        .post_a_newsletter(&serde_json::json!({
            "title": Uuid::new_v4().to_string(),
            "content": Uuid::new_v4().to_string()
        }))
        .await;
    assert_is_redirect_to(&response, "/login");
}
