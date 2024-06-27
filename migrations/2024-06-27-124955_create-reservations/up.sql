CREATE TABLE reservations (
    id SERIAL PRIMARY KEY,
    reservation_date DATE NOT NULL,
    party_size INTEGER NOT NULL,
    table_id INTEGER NOT NULL REFERENCES tables(id),
    customer_id INTEGER NOT NULL REFERENCES users(id)
);