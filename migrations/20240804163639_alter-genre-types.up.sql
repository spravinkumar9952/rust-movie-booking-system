-- Add up migration script here
CREATE TYPE genre_type_2 AS ENUM ('Action', 'Comedy', 'Drama', 'Horror', 'Romance');
ALTER TABLE movie ALTER COLUMN genre TYPE genre_type_2 USING genre::text::genre_type_2;
DROP TYPE genre_type;
ALTER TYPE genre_type_2 RENAME TO genre_type;