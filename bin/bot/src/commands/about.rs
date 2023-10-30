use localization::t;
use poise::serenity_prelude as serenity;
use poise::CreateReply;
use serenity::{CreateActionRow, CreateButton, CreateEmbed};

use crate::state::{Context, Error};

/// About this bot
#[poise::command(slash_command, description_localized("ja", "このbotについて"))]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let locale = ctx.locale().unwrap_or("ja");
    let embed = CreateEmbed::new()
        .color((160, 124, 240))
        .title(t!(locale, "main:about.title"))
        .description(t!(locale, "main:about.description"))
        .image("https://static.neody.org/example_artifacter_image.png")
        .thumbnail(
            ctx.http()
                .get_current_user()
                .await?
                .avatar_url()
                .unwrap_or_default(),
        );
    let components = CreateActionRow::Buttons([
        (t!(locale, "main:about.inviteThisBot"), "https://discord.com/api/oauth2/authorize?client_id=821033562555809824&permissions=0&scope=applications.commands%20bot"),
        (t!(locale, "main:about.website"), "https://artifacter.neody.land/"),
        (t!(locale, "main:about.supportServer"), "https://discord.com/invite/5JKEWnYZHj"),
    ].map(|(k, v)| {
        CreateButton::new_link(v).label(k)
    }).into_iter().collect());
    let reply = CreateReply::new().embed(embed).components(vec![components]);
    ctx.send(reply).await?;
    Ok(())
}
