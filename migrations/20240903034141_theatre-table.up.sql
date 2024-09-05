-- Add up migration script here
CREATE TABLE theatre (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    no_of_screens INT NOT NULL
);