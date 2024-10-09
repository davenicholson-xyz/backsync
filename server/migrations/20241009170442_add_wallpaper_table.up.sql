-- Add up migration script here
CREATE TABLE IF NOT EXISTS wallpapers (
    id TEXT PRIMARY KEY,
    filename TEXT NOT NULL UNIQUE,
    extension TEXT NOT NULL
);
