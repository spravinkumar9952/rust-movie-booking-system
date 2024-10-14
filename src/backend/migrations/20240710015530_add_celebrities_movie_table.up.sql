-- Add up migration script here
ALTER TABLE movie 
ADD COLUMN director_id INT REFERENCES celebrity(id),
ADD COLUMN actors_id INT REFERENCES celebrity(id),
ADD COLUMN singers_id INT REFERENCES celebrity(id),
ADD COLUMN music_director_id INT REFERENCES celebrity(id);
