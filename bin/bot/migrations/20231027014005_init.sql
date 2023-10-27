-- Add migration script here
CREATE TABLE IF NOT EXISTS linker (
    discord_id TEXT PRIMARY KEY NOT NULL,
    genshin_id TEXT NOT NULL
);
