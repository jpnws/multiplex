use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Parameters {
    _subscription_token: String,
}

// Adding the query parameters for `confirm` instruct actix-web to only call
// the this handler if the extraction of the parameter value was successful.
// If the extraction fails, a 400 Bad Request is returned to the caller.
#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
