-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    phone_number VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);