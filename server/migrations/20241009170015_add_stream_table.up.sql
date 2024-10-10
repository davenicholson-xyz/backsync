CREATE TABLE IF NOT EXISTS streams (
    id INTEGER PRIMARY KEY,
    addr TEXT NOT NULL UNIQUE,
    hostname TEXT NOT NULL,
    connected_at TEXT,
    wallpaper TEXT FOREIGN KEY(wallpaper) REFERENCES wallpapers(id)
);
