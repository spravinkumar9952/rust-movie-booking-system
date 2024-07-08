-- Add up migration script here
ALTER TABLE users
ADD CONSTRAINT users_phone_number_unique UNIQUE (phone_number);