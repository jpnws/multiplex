use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_publish_newsletter_form() {
    let app = spawn_app().await;
    let response = app.get_publish_newsletter().await;
    assert_is_redirect_to(&response, "/login");
}
