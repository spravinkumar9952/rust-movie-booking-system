-- Add up migration script here
CREATE TABLE theatre_seat (
    id SERIAL PRIMARY KEY,
    theatre_id INT NOT NULL,
    seat_number INT NOT NULL,
    "row" INT NOT NULL,   -- "row" is a reserved keyword, so quotes are needed.
    "column" INT NOT NULL,
    is_booked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (theatre_id) REFERENCES theatre(id)
);
