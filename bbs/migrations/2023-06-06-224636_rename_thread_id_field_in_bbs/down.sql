-- This file should undo anything in `up.sql`

ALTER TABLE boards RENAME COLUMN thread_id TO board_id;