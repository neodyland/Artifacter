use localization::t;
use poise::serenity_prelude as serenity;
use serenity::CreateInteractionResponse;
use serenity::{
    ComponentInteractionDataKind, Context, CreateInteractionResponseMessage,
    EditInteractionResponse, Embed, Interaction,
};

use crate::component::generate_components;
use crate::{
    component::profile_components,
    state::{Error, State},
};

use super::Framework;

pub fn message(embeds: &Vec<Embed>) -> Option<i32> {
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
        match select_menu.data.kind.clone() {
            ComponentInteractionDataKind::StringSelect { values } => {
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
                        if let Some((embed, components, attachment)) = generate_components(
                            lang.clone(),
                            uid.to_string(),
                            user,
                            value,
                            &state.api,
                        )
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
                }
            }
            ComponentInteractionDataKind::Button => {
                if custom_id == "build" {
                    select_menu.defer(&ctx.http).await?;
                    let uid = select_menu.message.embeds.first();
                    if uid.is_none() {
                        return Ok(());
                    }
                    let uid = &uid.unwrap().footer;
                    if uid.is_none() {
                        return Ok(());
                    }
                    let uid = &uid.as_ref().unwrap().text.parse::<i32>();
                    if uid.is_err() {
                        return Ok(());
                    }
                    let uid = uid.as_ref().unwrap().to_string();
                    let user = state.api.profile(uid.clone(), Some(lang.clone())).await?;
                    if user.characters.is_empty() {
                        let msg = EditInteractionResponse::new()
                            .components(vec![])
                            .embeds(vec![])
                            .content(t!(lang, "main:general.noCharacters"));
                        select_menu.edit_response(&ctx.http, msg).await?;
                        return Ok(());
                    }
                    let (embed, components, attachment) = profile_components(lang, uid, user);
                    let mut builder = EditInteractionResponse::new()
                        .components(components)
                        .embed(embed);
                    if let Some(attachment) = attachment {
                        builder = builder.new_attachment(attachment);
                    }
                    select_menu.edit_response(&ctx.http, builder).await?;
                }
                /* if custom_id == "end" {
                    let e = select_menu.message.embeds.first();
                    if e.is_none() {
                        return Ok(());
                    }
                    let e = e.unwrap();
                    let e = CreateEmbed::from(e.to_owned());
                    let res = CreateInteractionResponseMessage::new()
                        .embed(e)
                        .components(vec![])
                        .files(vec![]);
                    let res = CreateInteractionResponse::UpdateMessage(res);
                    select_menu.create_response(&ctx.http, res).await?;
                }*/
            }
            _ => {}
        }
    }
    Ok(())
}
