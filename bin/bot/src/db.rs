use std::env;

use sqlx::{migrate, query};
pub use sqlx::{Error as SqlxError, PgPool};

pub async fn connect() -> PgPool {
    let pool = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to connect to Postgres.");
    migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate database.");
    pool
}

pub async fn find_hsr(p: &PgPool, id: u64) -> Result<Option<String>, SqlxError> {
    Ok(query!(
        "SELECT hsr_id FROM hsr_linker WHERE discord_id = $1",
        id.to_string(),
    )
    .fetch_optional(p)
    .await?
    .map(|u| u.hsr_id))
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

pub async fn find_zzz(p: &PgPool, id: u64) -> Result<Option<String>, SqlxError> {
    Ok(query!(
        "SELECT zzz_id FROM zzz_linker WHERE discord_id = $1",
        id.to_string(),
    )
    .fetch_optional(p)
    .await?
    .map(|u| u.zzz_id))
}

pub async fn link(p: &PgPool, discord_id: u64, genshin_id: i32) -> Result<(), SqlxError> {
    query!(
        "INSERT INTO linker (discord_id, genshin_id) VALUES ($1, $2)",
        discord_id.to_string(),
        genshin_id.to_string()
    )
    .execute(p)
    .await?;
    Ok(())
}

pub async fn hsr_link(p: &PgPool, discord_id: u64, hsr_id: i32) -> Result<(), SqlxError> {
    query!(
        "INSERT INTO hsr_linker (discord_id, hsr_id) VALUES ($1, $2)",
        discord_id.to_string(),
        hsr_id.to_string()
    )
    .execute(p)
    .await?;
    Ok(())
}

pub async fn zzz_link(p: &PgPool, discord_id: u64, zzz_id: i32) -> Result<(), SqlxError> {
    query!(
        "INSERT INTO zzz_linker (discord_id, zzz_id) VALUES ($1, $2)",
        discord_id.to_string(),
        zzz_id.to_string()
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

pub async fn hsr_unlink(p: &PgPool, discord_id: u64) -> Result<(), SqlxError> {
    query!(
        "DELETE FROM hsr_linker WHERE discord_id = $1",
        discord_id.to_string(),
    )
    .execute(p)
    .await?;
    Ok(())
}

pub async fn zzz_unlink(p: &PgPool, discord_id: u64) -> Result<(), SqlxError> {
    query!(
        "DELETE FROM zzz_linker WHERE discord_id = $1",
        discord_id.to_string(),
    )
    .execute(p)
    .await?;
    Ok(())
}
