-- Add up migration script here
CREATE TABLE IF NOT EXISTS wallpapers (
    id INTEGER PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    extension TEXT NOT NULL
);
