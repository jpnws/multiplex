-- This file should undo anything in `up.sql`

ALTER TABLE boards RENAME COLUMN created_at TO creation_date;

ALTER TABLE boards RENAME COLUMN modified_at TO modified_date;

ALTER TABLE posts RENAME COLUMN id TO post_id;

ALTER TABLE posts RENAME COLUMN content TO post_content;

ALTER TABLE posts RENAME COLUMN created_at TO post_date;

ALTER TABLE posts RENAME COLUMN modified_at TO modified_date;

ALTER TABLE comments RENAME COLUMN id TO comment_id;

ALTER TABLE comments RENAME COLUMN content TO comment_content;

ALTER TABLE comments RENAME COLUMN created_at TO comment_date;

ALTER TABLE comments RENAME COLUMN modified_at TO modified_date;

ALTER TABLE replies RENAME COLUMN id TO reply_id;

ALTER TABLE replies RENAME COLUMN content TO reply_content;

ALTER TABLE replies RENAME COLUMN created_at TO reply_date;

ALTER TABLE replies RENAME COLUMN modified_at TO modified_date;