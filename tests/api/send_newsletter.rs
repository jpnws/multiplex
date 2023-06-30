use rstest::*;

use crate::helpers::{assert_is_redirect_to, spawn_app};
use multiplex::routes::{BodyData, Content};

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
    let body_data = BodyData {
        title: String::from("abc"),
        content: Content::new("def", "hijk"),
    };
    let response = app.post_newsletter(&body_data).await;
    assert_is_redirect_to(&response, "/login");
}

#[rstest]
// #[case((
//     serde_json::json!({
//         "title": "",
//         "content": Content::new("", ""),
//     }),
//     "Newsletter title and its content must not be empty."
// ))]
// #[case((
//     serde_json::json!({
//         "title": "",
//         "content": Content::new("abc", "def"),
//     }),
//     "Newsletter title must not be empty."
// ))]
#[case((
    BodyData {
        title: String::from("abc"),
        content: Content::new("", ""),
    },
    "Newsletter content must not be empty."
))]
#[tokio::test]
async fn newsletter_title_and_content_must_not_be_empty(#[case] test_case: (BodyData, &str)) {
    let app = spawn_app().await;

    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    let response = app.post_newsletter(&test_case.0).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains(test_case.1));
}
