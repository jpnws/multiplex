-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS replies CASCADE;

DROP TABLE IF EXISTS comments CASCADE;

DROP TABLE IF EXISTS posts CASCADE;

DROP TABLE IF EXISTS threads CASCADE;

DROP TRIGGER IF EXISTS update_replies_mod_time ON replies;

DROP TRIGGER IF EXISTS update_comments_mod_time ON comments;

DROP TRIGGER IF EXISTS update_posts_mod_time ON posts;

DROP TRIGGER IF EXISTS update_threads_mod_time ON threads;

DROP FUNCTION IF EXISTS update_modified_column();