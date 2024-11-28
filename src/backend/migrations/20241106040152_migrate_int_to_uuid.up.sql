
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE show_booking DROP COLUMN id;
ALTER TABLE show_booking ADD COLUMN id UUID PRIMARY KEY DEFAULT uuid_generate_v4();

ALTER TABLE theatre_seat DROP COLUMN theatre_id;
ALTER TABLE theatre_show DROP COLUMN theatre_id;

ALTER TABLE theatre DROP COLUMN id;
ALTER TABLE theatre ADD COLUMN id UUID PRIMARY KEY DEFAULT uuid_generate_v4();

ALTER TABLE show_booking DROP COLUMN show_id;

ALTER TABLE theatre_seat ADD COLUMN theatre_id UUID;
ALTER TABLE theatre_seat ADD CONSTRAINT fk_theatre_id FOREIGN KEY (theatre_id) REFERENCES theatre(id);

ALTER TABLE theatre_seat DROP COLUMN id;
ALTER TABLE theatre_seat ADD COLUMN id UUID PRIMARY KEY DEFAULT uuid_generate_v4();

ALTER TABLE theatre_show DROP COLUMN id;
ALTER TABLE theatre_show ADD COLUMN id UUID PRIMARY KEY DEFAULT uuid_generate_v4();

ALTER TABLE theatre_show ADD COLUMN theatre_id UUID;
ALTER TABLE theatre_show ADD CONSTRAINT fk_theatre_id FOREIGN KEY (theatre_id) REFERENCES theatre(id);

ALTER TABLE show_booking ADD COLUMN show_id UUID;
ALTER TABLE show_booking ADD CONSTRAINT fk_show_id FOREIGN KEY (show_id) REFERENCES theatre_show(id);

ALTER TABLE show_booking DROP COLUMN user_id;

ALTER TABLE users DROP COLUMN id;
ALTER TABLE users ADD COLUMN id UUID PRIMARY KEY DEFAULT uuid_generate_v4();

ALTER TABLE show_booking ADD COLUMN user_id UUID;
ALTER TABLE show_booking ADD CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id);

