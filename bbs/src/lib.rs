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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::testing::prelude::*;
    use axum::{routing::get, Router};

    use crate::models::*;

    // Helper function to create a test board
    fn create_test_board(conn: &PgConnection) -> Board {
        // Insert a test board into the database
        let new_board = models::NewBoard {
            name: todo!(),
            user_id: todo!(),
            created_at: todo!(),
            modified_at: todo!(),
            user_ip: todo!(),
        };

        diesel::insert_into(schema::boards::table)
            .values(&new_board)
            .get_result(conn)
            .expect("Failed to create test board")
    }

    #[tokio::test]
    async fn test_get_board_by_id() {
        // Set up the test environment
        load_env();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let conn =
            PgConnection::establish(&database_url).expect("Failed to connect to the database");

        // Create a test board
        let test_board = create_test_board(&conn);

        // Create the app with the test board as an extension
        let app = Router::new()
            .route(
                "/boards/:id",
                get(
                    extract::Extension::<PgConnection>::from_shared(conn.clone())
                        .and_then(get_board_by_id),
                ),
            )
            .layer(AddExtensionLayer::new(conn));

        // Send a request to get the test board by ID
        let response = request::get("/boards/1")
            .header("Content-Type", "application/json")
            .reply(&app)
            .await;

        // Assert that the response is successful
        assert_eq!(response.status(), StatusCode::OK);

        // Extract the response body
        let body = extract::json::<Value>(response.into_response().into_body())
            .await
            .unwrap();

        // Assert the expected response body based on the test board
        assert_eq!(body["thread"]["id"], test_board.id);
        // ... Assert other properties of the test board
    }
}
