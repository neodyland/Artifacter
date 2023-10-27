use std::env;

use sqlx::{migrate, query};
pub use sqlx::{Error as SqlxError, PgPool};

pub async fn connect() -> PgPool {
    migrate!();
    PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to connect to Postgres.")
}

pub async fn find_genshin(p: &PgPool, id: i64) -> Result<Option<String>, SqlxError> {
    Ok(query!(
        "SELECT genshin_id FROM linker WHERE discord_id = $1",
        id.to_string()
    )
    .fetch_optional(p)
    .await?
    .map(|u| u.genshin_id))
}

pub async fn link(p: &PgPool, discord_id: i64, genshin_id: &str) -> Result<(), SqlxError> {
    query!(
        "INSERT INTO linker (discord_id, genshin_id) VALUES ($1, $2) ON CONFLICT (discord_id) DO UPDATE SET genshin_id = $2",
        discord_id.to_string(),
        genshin_id
    )
    .execute(p)
    .await?;
    Ok(())
}

pub async fn unlink(p: &PgPool, discord_id: i64) -> Result<(), SqlxError> {
    query!(
        "DELETE FROM linker WHERE discord_id = $1",
        discord_id.to_string()
    )
    .execute(p)
    .await?;
    Ok(())
}
