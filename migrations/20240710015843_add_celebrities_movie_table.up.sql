-- Add up migration script here
CREATE TYPE designation_type AS ENUM ('director', 'actor', 'singer', 'music_director');

ALTER TABLE celebrity
ADD COLUMN designations designation_type [];