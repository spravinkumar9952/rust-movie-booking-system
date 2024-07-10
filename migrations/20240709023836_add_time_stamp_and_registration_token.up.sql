-- Add up migration script here
ALTER TABLE users
ADD COLUMN registration_token VARCHAR(255),
ADD COLUMN created_at TIMESTAMP;