use std::{
    collections::HashMap,
    env,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use crate::util::{create_components, json, Locale};
use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::{ImageFormat, Lang, ScoreCounter};
use poise::serenity_prelude::{
    ComponentInteractionDataKind, CreateAttachment, CreateEmbed, CreateEmbedFooter,
    EditInteractionResponse, Interaction,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use serenity::{gateway::ActivityData, model::Timestamp};
use tokio::sync::Mutex;

mod commands;
mod entity;
mod logger;
mod util;

use util::{convert_rgb, get_score_calc};

async fn event_event_handler(
    event: &serenity::all::FullEvent,
    _framework: poise::FrameworkContext<'_, Arc<Mutex<Data>>, Error>,
    data: &Arc<Mutex<Data>>,
) -> Result<(), Error> {
    match event {
        serenity::all::FullEvent::InteractionCreate { ctx, interaction } => {
            if let Interaction::Component(select_menu) = interaction {
                let custom_id = select_menu.data.custom_id.clone();
                let lang = Lang::from(select_menu.locale.as_str());
                match select_menu.data.kind.clone() {
                    ComponentInteractionDataKind::StringSelect { values } => {
                        if &custom_id == "character"
                            || &custom_id == "score"
                            || &custom_id == "format"
                        {
                            let value = values.first();
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
                            let uid = uid.as_ref().unwrap();
                            let mut data = data.lock().await;
                            let (user, down) = data.api.simple(*uid).await?;
                            let characters = user.characters_vec();
                            let mut current = data
                                .cache
                                .remove(&select_menu.message.id.get())
                                .unwrap_or((ScoreCounter::Normal, 114514, ImageFormat::Png));
                            if custom_id == "score" {
                                let normal = &"normal".to_string();
                                let score = value.unwrap_or(normal);
                                if score == "normal" {
                                    current.0 = ScoreCounter::Normal;
                                } else if score == "hp" {
                                    current.0 = ScoreCounter::Hp;
                                } else if score == "def" {
                                    current.0 = ScoreCounter::Def;
                                } else if score == "mastery" {
                                    current.0 = ScoreCounter::ElementalMastery;
                                } else if score == "charge" {
                                    current.0 = ScoreCounter::ChargeEfficiency;
                                }
                                data.cache
                                    .insert(select_menu.message.id.get(), current.clone());
                            } else if custom_id == "character" {
                                let character = characters.iter().find(|c| {
                                    c.id.0
                                        == value
                                            .unwrap_or(&"114514".to_string())
                                            .parse()
                                            .unwrap_or(114514)
                                });
                                if character.is_none() {
                                    return Ok(());
                                }
                                let character = character.unwrap().to_owned().to_owned();
                                current.1 = character.id.0;
                                let def = gen::get_default(&character.id.0);
                                current.0 = def;
                                data.cache
                                    .insert(select_menu.message.id.get(), current.clone());
                            } else {
                                let d = &"png".to_string();
                                let f = value.unwrap_or(d);
                                if f == "png" {
                                    current.2 = ImageFormat::Png;
                                } else if f == "jpeg" {
                                    current.2 = ImageFormat::Jpeg;
                                }
                                data.cache
                                    .insert(select_menu.message.id.get(), current.clone());
                            }
                            let character = characters.iter().find(|c| c.id.0 == current.1);
                            if character.is_none() {
                                return Ok(());
                            }
                            let character = character.unwrap().to_owned().to_owned();
                            let now = std::time::Instant::now();
                            let img = gen::generate(
                                character.to_owned(),
                                &data.api,
                                &lang,
                                &data.icons,
                                current.0,
                                current.2.clone(),
                            )
                            .await;
                            if img.is_none() {
                                log::error!(
                                    "Failed to generate image for uid: {} and cid: {}",
                                    uid,
                                    character.id.0
                                );
                                return Ok(());
                            }
                            log::info!("Generated image in {}ms", now.elapsed().as_millis());
                            let img = img.unwrap();
                            let footer = CreateEmbedFooter::new(format!("{}", uid));
                            let filename = format!("image.{}", current.2.to_string());
                            let e = CreateEmbed::default()
                                .title(
                                    Locale::from(json!({"ja":format!(
                                            "{}のステータス",
                                            character.name(&data.api, &lang.to_string())?
                                        ),"en": format!(
                                        "Status: {}",
                                        character.name(&data.api, &lang.to_string())?
                                    )}))
                                    .get(&lang),
                                )
                                .description(format!("{}{}",Locale::from(get_score_calc(&current.0)).get(&lang),if down {
                                    let cached = Locale::from(json!({"ja":"\nキャッシュから取得","en": "\nCached data"}));
                                    cached.get(&lang).to_string()
                                } else {"".to_string()}))
                                .attachment(&filename)
                                .footer(footer)
                                .timestamp(Timestamp::from_unix_timestamp(user.lastupdate().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64).unwrap())
                                .color(convert_rgb(character.element.color_rgb()));
                            let at = CreateAttachment::bytes(img, filename);
                            let res = EditInteractionResponse::new()
                                .new_attachment(at)
                                .components(create_components(characters, &data.api, &lang, uid))
                                .embed(e);
                            select_menu.edit_response(&ctx.http, res).await?;
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
                            let uid = uid.as_ref().unwrap();
                            let data = data.lock().await;
                            let (user, down) = data.api.simple(*uid).await?;
                            let characters = user.characters_vec();
                            if characters.is_empty() {
                                let msg = EditInteractionResponse::new().components(vec![]).embeds(vec![]).content(Locale::from(
                                    json!({"ja":"キャラクターが登録されていません。(もしくは非公開になっています)","en": "No character is set(Or it may be private)"})).get(&lang));
                                select_menu.edit_response(&ctx.http, msg).await?;
                                return Ok(());
                            }
                            let footer = CreateEmbedFooter::new(format!("{}", uid));
                            let embed = CreateEmbed::default()
                                .title(format!(
                                    "{}({},{})",
                                    user.profile().nickname(),
                                    user.profile().level(),
                                    user.profile().world_level()
                                ))
                                .footer(footer)
                                .color(convert_rgb([0x00, 0xff, 0x00]))
                                .description(format!("{}{}",user.profile().signature(),if down {
                                    let cached = Locale::from(json!({"ja":"\nキャッシュから取得","en": "\nCached data"}));
                                    cached.get(&lang).to_string()
                                } else {"".to_string()}))
                                .image("attachment://name_card.png")
                                .timestamp(Timestamp::from_unix_timestamp(user.lastupdate().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64).unwrap())
                                .fields(vec![
                                    (
                                        Locale::from(json!(
                                            {"ja": "アチーブメント", "en": "Achievements"}
                                        ))
                                        .get(&lang),
                                        user.profile().achievement().to_string(),
                                        true,
                                    ),
                                    (
                                        Locale::from(json!(
                                            {"ja": "螺旋", "en": "Spiral Abyss"}
                                        ))
                                        .get(&lang),
                                        format!(
                                            "{}{}{}{}",
                                            user.profile().tower_floor_index(),
                                            Locale::from(json!(
                                                {"ja": "階", "en": "F"}
                                            ))
                                            .get(&lang),
                                            user.profile().tower_level_index(),
                                            Locale::from(json!(
                                                {"ja": "層", "en": "L"}
                                            ))
                                            .get(&lang)
                                        ),
                                        true,
                                    ),
                                ]);
                            let card = match user.profile().name_card_image(&data.api).await {
                                Some(card) => gen::convert(card, ImageFormat::Png),
                                None => None,
                            };
                            let attachment = if card.is_some() {
                                Some(CreateAttachment::bytes(card.unwrap(), "name_card.png"))
                            } else {
                                None
                            };
                            let mut builder = EditInteractionResponse::default()
                                .components(create_components(characters, &data.api, &lang, uid))
                                .embed(embed);
                            if attachment.is_some() {
                                builder = builder.new_attachment(attachment.unwrap());
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
        }
        _ => {}
    }

    Ok(())
}