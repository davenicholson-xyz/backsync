-- Add up migration script here
ALTER TABLE clients ADD locked BOOLEAN DEFAULT 0;
