use crate::models::Board;
use actix_web::{get, web, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde_json::json;
use std::env;

pub mod models;
pub mod schema;

#[get("/boards/{id}")]
pub async fn get_board_by_id(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    use self::schema::boards::dsl::boards;
    let conn = &mut establish_connection();
    let result = boards.find(id).first::<Board>(conn);
    match result {
        Ok(board) => web::Json(json!({
            "board": board,
        })),
        Err(diesel::NotFound) => web::Json(json!({
            "error": "Board not found",
        })),
        Err(err) => web::Json(json!({
            "error": format!("Database error: {}", err),
        })),
    }
}

fn establish_connection() -> PgConnection {
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
