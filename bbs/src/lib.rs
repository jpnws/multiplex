use axum::extract::Path;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Thread {
    thread_id: u32,
    title: String,
    user_id: u32,
    creation_date: String,
    modified_date: String,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    post_id: u32,
    thread_id: u32,
    user_id: u32,
    post_content: String,
    post_date: String,
    modified_date: String,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    comment_id: u32,
    post_id: u32,
    user_id: u32,
    comment_content: String,
    comment_date: String,
    modified_date: String,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reply {
    reply_id: u32,
    comment_id: u32,
    parent_reply_id: Option<u32>,
    user_id: u32,
    reply_content: String,
    reply_date: String,
    modified_date: String,
    user_ip: String,
}

pub async fn thread_handler(Path(thread_id): Path<u32>) -> Json<Value> {
    // Perform logic to fetch the thread by ID from a database or other data source
    // let thread = fetch_thread_by_id(thread_id);
    println!("thread_id {}", thread_id);
    let thread = Some(Thread {
        thread_id: 1,
        title: "Example Thread".to_owned(),
        user_id: 1,
        creation_date: "2023-06-04".to_owned(),
        modified_date: "2023-06-04".to_owned(),
        user_ip: "127.0.0.1".to_owned(),
    });
    // Return a JSON response with the thread data
    match thread {
        Some(thread) => Json(json!({
            "thread": thread,
        })),
        None => Json(json!({
            "error": "Thread not found",
        })),
    }
}
