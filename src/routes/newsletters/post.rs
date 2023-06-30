use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use reqwest::header;
use sqlx::PgPool;

use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::routes::error_chain_fmt;
use crate::utils::see_other;

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BodyData {
    pub title: String,
    pub content: Content,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

impl Content {
    pub fn new(html: &str, text: &str) -> Content {
        Content {
            html: html.to_string(),
            text: text.to_string(),
        }
    }
}

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PublishError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PublishError::UnexpectedError(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            PublishError::AuthError(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
        }
    }
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip(pool, body, email_client)
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn publish_newsletter(
    pool: web::Data<PgPool>,
    body: web::Form<BodyData>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, PublishError> {
    if body.title.is_empty() && body.content.html.is_empty() && body.content.text.is_empty() {
        FlashMessage::error("Newsletter title and its content must not be empty.").send();
        return Ok(see_other("/admin/newsletters"));
    }

    if body.title.is_empty() {
        FlashMessage::error("Newsletter title must not be empty.").send();
        return Ok(see_other("/admin/newsletters"));
    }

    if body.content.html.is_empty() && body.content.text.is_empty() {
        FlashMessage::error("Newsletter content must not be empty.").send();
        return Ok(see_other("/admin/newsletters"));
    }

    let subscribers = get_confirmed_subscribers(&pool).await?;

    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(
                        &subscriber.email,
                        &body.title,
                        &body.content.html,
                        &body.content.text,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {}", subscriber.email)
                    })?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    "Skipping a confirmed subscriber. Their stored contact details are invalid",
                );
            }
        };
    }

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
    // We are returning a `Vec` of `Result`s in the happy case. This allows the
    // caller to bubble up errors due to network issues or other transient
    // failures using the `?` operator, while the compiler forces them to handle
    // the subtle mapping error. See http://sled.rs/errors.html for a deep-dive
    // about this technique.
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers = sqlx::query!(
        r#"
            SELECT
                email
            FROM
                subscriptions
            WHERE
                status = 'confirmed'
        "#,
    )
    .fetch_all(pool)
    .await?
    // Map into the domain type.
    .into_iter()
    .map(|r| match SubscriberEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();

    Ok(confirmed_subscribers)
}
