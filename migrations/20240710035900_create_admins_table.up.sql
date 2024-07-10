-- Add up migration script here
CREATE TABLE admins (
    id SERIAL PRIMARY KEY,
    phone_number VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    registration_token VARCHAR(255) UNIQUE,
    created_at TIMESTAMP
);