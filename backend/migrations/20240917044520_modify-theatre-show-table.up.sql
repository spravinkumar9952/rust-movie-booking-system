-- Add up migration script here
ALTER TABLE theatre_show RENAME COLUMN show_time TO start_time;
ALTER TABLE theatre_show ADD COLUMN end_time TIMESTAMP WITHOUT TIME ZONE;
ALTER TABLE theatre_show ADD COLUMN movie_id VARCHAR(255);
ALTER TABLE theatre_show ADD CONSTRAINT fk_movie_id FOREIGN KEY (movie_id) REFERENCES movie(id);
