-- Add up migration script here
ALTER TABLE clients ADD syncwall TEXT;

CREATE TRIGGER IF NOT EXISTS nullify_syncwall_on_wallpaper_delete
AFTER DELETE ON wallpapers
FOR EACH ROW
BEGIN
    UPDATE clients
    SET syncwall = NULL
    WHERE syncwall = OLD.code;
END;
