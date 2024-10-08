-- Add up migration script here
ALTER TABLE theatre ADD COLUMN "row" INT NOT NULL;
ALTER TABLE theatre ADD COLUMN "column" INT NOT NULL;