CREATE TABLE IF NOT EXISTS clients (
    id INTEGER PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    addr TEXT NOT NULL UNIQUE,
    hostname TEXT NOT NULL,
    connected_at TEXT,
    wallpaper INTEGER, FOREIGN KEY(wallpaper) REFERENCES wallpapers(id)
);
