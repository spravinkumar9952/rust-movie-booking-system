-- Add down migration script here
ALTER TABLE users DROP CONSTRAINT users_phone_number_unique;