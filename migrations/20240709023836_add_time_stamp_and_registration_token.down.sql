-- Add down migration script here
ALTER TABLE users
DROP COLUMN registration_token,
DROP COLUMN created_at;