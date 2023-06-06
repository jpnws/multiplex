-- This file should undo anything in `up.sql`

ALTER TABLE boards RENAME COLUMN id TO board_id;

ALTER TABLE boards RENAME COLUMN name TO title;