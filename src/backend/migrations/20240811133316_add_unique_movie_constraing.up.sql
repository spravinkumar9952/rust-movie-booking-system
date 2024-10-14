-- Add up migration script here
ALTER TABLE movie ADD CONSTRAINT movie_id_unique UNIQUE (title, genre);