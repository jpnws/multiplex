use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=boards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Board {
    id: i32,
    name: String,
    user_id: i32,
    created_at: NaiveDateTime,
    modified_at: NaiveDateTime,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    id: i32,
    thread_id: i32,
    user_id: i32,
    content: String,
    created_at: NaiveDateTime,
    modified_at: NaiveDateTime,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    id: i32,
    post_id: i32,
    user_id: i32,
    content: String,
    created_at: NaiveDateTime,
    modified_at: NaiveDateTime,
    user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=replies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reply {
    id: i32,
    comment_id: i32,
    parent_reply_id: Option<i32>,
    user_id: i32,
    content: String,
    created_at: NaiveDateTime,
    modified_at: NaiveDateTime,
    user_ip: String,
}

#[derive(Insertable)]
#[diesel(table_name=boards)]
pub struct NewThread<'a> {
    pub name: &'a str,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name=posts)]
pub struct NewPost<'a> {
    pub thread_id: i32,
    pub user_id: i32,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name=comments)]
pub struct NewComment<'a> {
    pub post_id: i32,
    pub user_id: i32,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name=replies)]
pub struct NewReply<'a> {
    pub comment_id: i32,
    pub parent_reply_id: Option<i32>,
    pub user_id: i32,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: &'a str,
}
