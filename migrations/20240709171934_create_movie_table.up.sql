-- Add up migration script here
CREATE TYPE genre_type AS ENUM ('action', 'comedy', 'drama', 'horror', 'romance', 'sci-fi', 'thriller');


CREATE TABLE movie (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    release_date DATE ,
    duration INT,
    rating DECIMAL(1, 1) CONSTRAINT rating_check CHECK (rating >= 0 AND rating <= 5),
    genre genre_type,
    created_at TIMESTAMP
);