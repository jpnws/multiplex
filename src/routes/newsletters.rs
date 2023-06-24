use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::ResponseError;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use base64::Engine;
use reqwest::header;
use secrecy::Secret;
use sqlx::PgPool;

use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::routes::error_chain_fmt;

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
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
                    // actix_web::http::header provides a collection of constants for
                    // the names of several well-known and standard HTTP headers.
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
        }
    }
    // `status_code` is invoked by the default `error_response` implementation.
    // We are providing a bespoke `error_response` implementation; therefore
    // there is no need to maintain a `status_code` implementation anymore.
    // fn status_code(&self) -> StatusCode {
    //     match self {
    //         PublishError::AuthError(_) => StatusCode::UNAUTHORIZED,
    //         PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    //     }
    // }
}

// We are prefixing `body` with a `_` to avoid a compiler warning about unused
// arguments.
pub async fn publish_newsletter(
    pool: web::Data<PgPool>,
    body: web::Json<BodyData>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, PublishError> {
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
                    // We record the error chain as a structured field on the log
                    // record.
                    error.cause_chain = ?error,
                    // Using `\` to split a long string literal over two lines,
                    // without creating a `\n` character.
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid",
                );
            }
        };
    }

    Ok(HttpResponse::Ok().finish())
}

struct Credentials {
    username: String,
    password: Secret<String>,
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
