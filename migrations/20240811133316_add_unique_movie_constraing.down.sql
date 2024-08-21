-- Add down migration script here
ALTER TABLE movie DROP CONSTRAINT movie_id_unique;