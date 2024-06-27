CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    content VARCHAR NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    post_id INTEGER NOT NULL REFERENCES posts(id)
);