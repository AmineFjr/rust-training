CREATE TABLE tables (
    id SERIAL PRIMARY KEY,
    seating_capacity INTEGER NOT NULL,
    restaurant_id INTEGER NOT NULL REFERENCES restaurants(id)
);