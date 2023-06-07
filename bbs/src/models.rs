use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=boards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Board {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub thread_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=replies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reply {
    pub id: i32,
    pub comment_id: i32,
    pub parent_reply_id: Option<i32>,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
    pub user_ip: String,
}

#[derive(Insertable)]
#[diesel(table_name=boards)]
pub struct NewBoard<'a> {
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
