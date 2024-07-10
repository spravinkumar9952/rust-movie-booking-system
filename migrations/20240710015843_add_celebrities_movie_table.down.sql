-- Add down migration script here
ALTER TABLE celebrity
DROP COLUMN movies_id,
DROP COLUMN designations;