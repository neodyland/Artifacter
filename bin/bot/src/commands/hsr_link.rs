use localization::t;
use poise::CreateReply;

use crate::{
    db::{find_hsr, hsr_link as db_link},
    state::{Context, Error},
};

/// link discord account to hsr account
#[poise::command(slash_command, description_localized("ja", "データをリンクします"))]
pub async fn hsr_link(
    ctx: Context<'_>,
    #[description = "UID"]
    #[description_localized("ja", "ユーザーID")]
    uid: i32,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();
    let locale = ctx.locale().unwrap_or("ja");
    let db = &ctx.data().db;
    if uid.to_string().len() != 9 && uid.to_string().len() != 10 {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.userIdMustBeNineDigits")))
            .await?;
        return Ok(());
    }
    ctx.defer().await?;
    let current = find_hsr(db, discord_id).await.ok().flatten();
    if current.is_some() {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.alreadyLinked")))
            .await?;
        return Ok(());
    }
    if let Err(err) = db_link(db, discord_id, uid).await {
        ctx.send(CreateReply::new().content(t!(locale, "main:general.failedToLink", err)))
            .await?;
        return Ok(());
    }
    ctx.send(CreateReply::new().content(t!(locale, "main:general.linked")))
        .await?;
    Ok(())
}
