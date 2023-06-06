use crate::models::Thread;
use axum::extract::Path;
use axum::response::Json;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde_json::{json, Value};
use std::env;

pub mod models;
pub mod schema;

pub async fn get_thread_by_id(Path(thread_id): Path<i32>) -> Json<Value> {
    use self::schema::threads::dsl::threads;

    let conn = &mut establish_connection();
    let result = threads.find(thread_id).first::<Thread>(conn);

    match result {
        Ok(thread) => Json(json!({
            "thread": thread,
        })),
        Err(diesel::NotFound) => Json(json!({
            "error": "Thread not found",
        })),
        Err(err) => Json(json!({
            "error": format!("Database error: {}", err),
        })),
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL_BBS").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
