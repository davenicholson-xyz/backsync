-- Add up migration script here
CREATE TABLE IF NOT EXISTS streams (
    id INTEGER PRIMARY KEY,
    addr TEXT NOT NULL UNIQUE,
    hostname TEXT NOT NULL,
    connected_at TEXT
);
