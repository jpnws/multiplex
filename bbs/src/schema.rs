// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        thread_id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::table! {
    replies (id) {
        id -> Int4,
        comment_id -> Int4,
        parent_reply_id -> Nullable<Int4>,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> boards (thread_id));
diesel::joinable!(replies -> comments (comment_id));

diesel::allow_tables_to_appear_in_same_query!(boards, comments, posts, replies,);
