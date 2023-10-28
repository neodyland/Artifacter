-- Add migration script here
CREATE TABLE IF NOT EXISTS linker (
    discord_id TEXT,
    genshin_id TEXT NOT NULL,
    old_discord_id DOUBLE PRECISION
);
