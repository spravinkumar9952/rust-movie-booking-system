-- Add up migration script here
ALTER TABLE movie ADD COLUMN genre VARCHAR(255) NOT NULL;