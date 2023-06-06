use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::threads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Thread {
    thread_id: i32,
    title: String,
    user_id: i32,
    creation_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    post_id: i32,
    thread_id: i32,
    user_id: i32,
    post_content: String,
    post_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    comment_id: i32,
    post_id: i32,
    user_id: i32,
    comment_content: String,
    comment_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::replies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reply {
    reply_id: i32,
    comment_id: i32,
    parent_reply_id: Option<i32>,
    user_id: i32,
    reply_content: String,
    reply_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    user_ip: String,
}
