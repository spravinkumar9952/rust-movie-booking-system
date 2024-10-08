-- Add up migration script here
CREATE TABLE celebrity (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
)