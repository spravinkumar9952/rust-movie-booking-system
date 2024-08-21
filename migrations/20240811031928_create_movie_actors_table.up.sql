-- Add up migration script here

ALTER TABLE movie ALTER COLUMN id DROP DEFAULT;
ALTER TABLE movie ALTER COLUMN id TYPE VARCHAR(255);

ALTER TABLE celebrity ALTER COLUMN id DROP DEFAULT;
ALTER TABLE celebrity ALTER COLUMN id TYPE VARCHAR(255);

CREATE TABLE movie_celebrities (
    movie_id VARCHAR(255) REFERENCES movie(id) ON DELETE CASCADE,
    celebrity_id VARCHAR(255) REFERENCES celebrity(id) ON DELETE CASCADE,
    PRIMARY KEY (movie_id, celebrity_id)
);


