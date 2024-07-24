-- Add migration script here
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL, 
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)
