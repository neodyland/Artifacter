-- Add migration script here
CREATE TABLE IF NOT EXISTS hsr_linker (
    discord_id TEXT NOT NULL,
    hsr_id TEXT NOT NULL
);
