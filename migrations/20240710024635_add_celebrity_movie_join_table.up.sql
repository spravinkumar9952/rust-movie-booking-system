-- Add up migration script here
CREATE TABLE celebrity_movies (
    celebrity_id INT NOT NULL,
    movie_id INT NOT NULL,
    PRIMARY KEY (celebrity_id, movie_id),
    CONSTRAINT fk_celebrity
        FOREIGN KEY (celebrity_id) 
        REFERENCES celebrity (id)
        ON DELETE CASCADE,
    CONSTRAINT fk_movie
        FOREIGN KEY (movie_id) 
        REFERENCES movie (id)
        ON DELETE CASCADE
);