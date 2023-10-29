use localization::t;
use poise::CreateReply;

use crate::db::find_hsr;
use crate::db::hsr_unlink as db_unlink;
use crate::state::{Context, Error};

/// unlink discord account from hsr account
#[poise::command(
    slash_command,
    description_localized("ja", "データのリンクを解除します")
)]
pub async fn hsr_unlink(ctx: Context<'_>) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    ctx.defer().await?;
    let db = &ctx.data().db;
    let discord_id = ctx.author().id.get();
    let current = find_hsr(&db, discord_id).await.ok().flatten();
    if current.is_none() {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.notLinked")))
            .await?;
        return Ok(());
    }
    if let Err(err) = db_unlink(&db, discord_id).await {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.failedToUnlink", err)))
            .await?;
        return Ok(());
    }
    ctx.send(CreateReply::new().content(t!(locale, "main:general.unlinked")))
        .await?;
    Ok(())
}
