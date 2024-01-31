use localization::t;
use poise::serenity_prelude as serenity;
use serenity::CreateInteractionResponse;
use serenity::{
    ComponentInteractionDataKind, Context, CreateInteractionResponseMessage,
    EditInteractionResponse, Embed, Interaction,
};

use crate::component::generate_components;
use crate::hsr_components::{hsr_generate_components, hsr_profile_components};
use crate::{
    component::profile_components,
    state::{Error, State},
};

use super::Framework;

pub fn message(embeds: &[Embed]) -> Option<i32> {
    if let Some(embed) = embeds.first() {
        if let Some(footer) = &embed.footer {
            if let Ok(f) = footer.text.parse::<i32>() {
                return Some(f);
            } else {
                return None;
            }
        }
    }
    None
}

pub async fn handler(
    _frame: Framework<'_>,
    state: &State,
    ctx: &Context,
    interaction: &Interaction,
) -> Result<(), Error> {
    if let Interaction::Component(select_menu) = interaction {
        let custom_id = select_menu.data.custom_id.clone();
        let lang = select_menu.locale.to_string();
        if let ComponentInteractionDataKind::StringSelect { values } = &select_menu.data.kind {
            if &custom_id == "character" || &custom_id == "score" || &custom_id == "format" {
                select_menu.defer(&ctx.http).await?;
                let uid = match message(&select_menu.message.embeds) {
                    Some(uid) => uid,
                    None => {
                        select_menu
                            .create_response(
                                ctx,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content(t!(lang, "main:general.parseFailed")),
                                ),
                            )
                            .await?;
                        return Ok(());
                    }
                };
                let user = state
                    .api
                    .profile(uid.to_string(), Some(lang.clone()))
                    .await?;
                let mut cache = state.cache.lock().await;
                if let Some(value) = values.first() {
                    cache.update(uid, custom_id, value.to_string());
                };
                if let Some(value) = cache.get_or_default(uid) {
                    if let Some((embed, components, attachment)) =
                        generate_components(lang.clone(), uid.to_string(), user, value, &state.api)
                            .await
                    {
                        let mut builder = EditInteractionResponse::new()
                            .components(components)
                            .embed(embed);
                        if let Some(attachment) = attachment {
                            builder = builder.new_attachment(attachment);
                        }
                        select_menu.edit_response(&ctx.http, builder).await?;
                    };
                } else {
                    let (embed, components, attachment) =
                        profile_components(lang, uid.to_string(), user);
                    let mut builder = EditInteractionResponse::new()
                        .components(components)
                        .embed(embed);
                    if let Some(attachment) = attachment {
                        builder = builder.new_attachment(attachment);
                    }
                    select_menu.edit_response(&ctx.http, builder).await?;
                }
            } else if &custom_id == "hsr_character"
                || &custom_id == "hsr_score"
                || &custom_id == "hsr_format"
                || &custom_id == "hsr_base_img"
            {
                select_menu.defer(&ctx.http).await?;
                let uid = match message(&select_menu.message.embeds) {
                    Some(uid) => uid,
                    None => {
                        select_menu
                            .create_response(
                                ctx,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content(t!(lang, "main:general.parseFailed")),
                                ),
                            )
                            .await?;
                        return Ok(());
                    }
                };
                let user = state
                    .api
                    .hsr_profile(uid.to_string(), Some(lang.clone()))
                    .await?;
                let mut cache = state.hsr_cache.lock().await;
                if let Some(value) = values.first() {
                    cache.update(uid, custom_id, value.to_string());
                };
                if let Some(value) = cache.get_or_default(uid) {
                    if let Some((embed, components, attachment)) =
                        hsr_generate_components(lang, uid.to_string(), user, value, &state.api)
                            .await
                    {
                        let mut builder = EditInteractionResponse::new()
                            .components(components)
                            .embed(embed);
                        if let Some(attachment) = attachment {
                            builder = builder.new_attachment(attachment);
                        }
                        select_menu.edit_response(&ctx.http, builder).await?;
                    };
                } else {
                    let (embed, components, attachment) =
                        hsr_profile_components(lang, uid.to_string(), user);
                    let mut builder = EditInteractionResponse::new()
                        .components(components)
                        .embed(embed);
                    if let Some(attachment) = attachment {
                        builder = builder.new_attachment(attachment);
                    }
                    select_menu.edit_response(&ctx.http, builder).await?;
                }
            }
        }
    }
    Ok(())
}
