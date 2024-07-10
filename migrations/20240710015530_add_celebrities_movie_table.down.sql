-- Add down migration script here
ALTER TABLE movie
DROP COLUMN director_id,
DROP COLUMN actors_id,
DROP COLUMN singers_id,
DROP COLUMN music_director_id;
