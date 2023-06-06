// @generated automatically by Diesel CLI.

diesel::table! {
    comments (comment_id) {
        comment_id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        comment_content -> Text,
        comment_date -> Timestamp,
        modified_date -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Int4,
        thread_id -> Int4,
        user_id -> Int4,
        post_content -> Text,
        post_date -> Timestamp,
        modified_date -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::table! {
    replies (reply_id) {
        reply_id -> Int4,
        comment_id -> Int4,
        parent_reply_id -> Nullable<Int4>,
        user_id -> Int4,
        reply_content -> Text,
        reply_date -> Timestamp,
        modified_date -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::table! {
    threads (thread_id) {
        thread_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        user_id -> Int4,
        creation_date -> Timestamp,
        modified_date -> Timestamp,
        #[max_length = 255]
        user_ip -> Varchar,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> threads (thread_id));
diesel::joinable!(replies -> comments (comment_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    replies,
    threads,
);
