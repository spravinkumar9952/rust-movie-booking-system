-- Add up migration script here
DROP TABLE celebrity_movies;
DROP TABLE movie;
CREATE TABLE movie (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title VARCHAR(255) NOT NULL
)