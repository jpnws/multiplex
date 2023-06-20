use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct Parameters {
    subscription_token: String,
}

// Adding the query parameters for `confirm` instructs actix-web to only call
// the this handler if the extraction of the parameter value was successful.
// If the extraction fails, a 400 Bad Request is returned to the caller.
#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters))]
pub async fn confirm(parameters: web::Query<Parameters>) -> HttpResponse {
    dbg!("{}", &parameters.subscription_token);
    HttpResponse::Ok().finish()
}
