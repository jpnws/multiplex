-- Your SQL goes here

ALTER TABLE boards RENAME COLUMN creation_date TO created_at;

ALTER TABLE boards RENAME COLUMN modified_date TO modified_at;

ALTER TABLE posts RENAME COLUMN post_id TO id;

ALTER TABLE posts RENAME COLUMN post_content TO content;

ALTER TABLE posts RENAME COLUMN post_date TO created_at;

ALTER TABLE posts RENAME COLUMN modified_date TO modified_at;

ALTER TABLE comments RENAME COLUMN comment_id TO id;

ALTER TABLE comments RENAME COLUMN comment_content TO content;

ALTER TABLE comments RENAME COLUMN comment_date TO created_at;

ALTER TABLE comments RENAME COLUMN modified_date TO modified_at;

ALTER TABLE replies RENAME COLUMN reply_id TO id;

ALTER TABLE replies RENAME COLUMN reply_content TO content;

ALTER TABLE replies RENAME COLUMN reply_date TO created_at;

ALTER TABLE replies RENAME COLUMN modified_date TO modified_at;