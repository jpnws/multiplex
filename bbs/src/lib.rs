use crate::models::Board;
use axum::extract::Path;
use axum::response::Json;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde_json::{json, Value};
use std::env;

pub mod models;
pub mod schema;

pub async fn get_board_by_id(Path(id): Path<i32>) -> Json<Value> {
    use self::schema::boards::dsl::boards;
    let conn = &mut establish_connection();
    let result = boards.find(id).first::<Board>(conn);
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
    load_env();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn load_env() {
    let current_crate_dir = env!("CARGO_MANIFEST_DIR");
    let env_file_path = format!("{}/.env", current_crate_dir);
    dotenvy::from_filename(env_file_path).ok();
}
