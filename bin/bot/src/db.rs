use std::env;

use sqlx::{migrate, query};
pub use sqlx::{Error as SqlxError, PgPool};

pub async fn connect() -> PgPool {
    migrate!();
    PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to connect to Postgres.")
}

pub async fn find_genshin(p: &PgPool, id: u64) -> Result<Option<String>, SqlxError> {
    // migration
    if let Some(genshin_id) = query!(
        "SELECT genshin_id FROM linker WHERE old_discord_id = $1",
        id as f64
    )
    .fetch_optional(p)
    .await?
    {
        query!(
            "UPDATE linker SET discord_id = $1, old_discord_id = NULL WHERE genshin_id = $2",
            id.to_string(),
            genshin_id.genshin_id
        )
        .execute(p)
        .await?;
        return Ok(Some(genshin_id.genshin_id));
    };
    Ok(query!(
        "SELECT genshin_id FROM linker WHERE discord_id = $1 OR old_discord_id = $2",
        id.to_string(),
        id as f64
    )
    .fetch_optional(p)
    .await?
    .map(|u| u.genshin_id))
}

pub async fn link(p: &PgPool, discord_id: u64, genshin_id: i32) -> Result<(), SqlxError> {
    query!(
        "INSERT INTO linker (discord_id, genshin_id) VALUES ($1, $2) ON CONFLICT (discord_id) DO UPDATE SET genshin_id = $2",
        discord_id.to_string(),
        genshin_id.to_string()
    )
    .execute(p)
    .await?;
    Ok(())
}

pub async fn unlink(p: &PgPool, discord_id: u64) -> Result<(), SqlxError> {
    query!(
        "DELETE FROM linker WHERE discord_id = $1 OR old_discord_id = $2",
        discord_id.to_string(),
        discord_id as f64
    )
    .execute(p)
    .await?;
    Ok(())
}
