use std::{collections::HashMap, env, sync::Arc};

use crate::util::{create_components, json, Locale};
use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::{ImageFormat, Lang, ScoreCounter};
use poise::serenity_prelude::{
        ComponentInteractionDataKind, CreateAttachment, CreateEmbed, CreateEmbedFooter,
        EditInteractionResponse, Interaction,
};
use serenity::gateway::ActivityData;
use tokio::sync::Mutex;

mod logger;
mod util;
mod commands;

use util::{convert_rgb,get_score_calc};

pub struct Data {
    pub api: EnkaNetwork,
    pub icons: IconData,
    pub cache: HashMap<u64, (ScoreCounter, u32, ImageFormat)>,
    pub looping: bool,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Arc<Mutex<Data>>, Error>;

async fn event_event_handler(
    event: &serenity::all::FullEvent,
    _framework: poise::FrameworkContext<'_, Arc<Mutex<Data>>, Error>,
    data: &Arc<Mutex<Data>>,
) -> Result<(), Error> {
    match event {
        serenity::all::FullEvent::Ready {
            ctx,
            data_about_bot,
        } => {
            log::info!("{} is connected!", data_about_bot.user.name);
            let ctx = Arc::new(ctx.to_owned());
            if data.lock().await.looping == false {
                data.lock().await.looping = true;
                tokio::spawn(async move {
                    loop {
                        let activity =
                            ActivityData::playing(&format!("{} guilds", ctx.cache.guild_count()));
                        ctx.set_activity(Some(activity));
                        tokio::time::sleep(std::time::Duration::from_secs(20)).await;
                    }
                });
            }
        }
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
                            let user = data.api.simple(*uid).await?;
                            let ids = user.profile().show_character_list();
                            let characters =
                                ids.iter().map(|id| user.character(*id)).collect::<Vec<_>>();
                            let characters = characters
                                .iter()
                                .filter_map(|c| c.as_ref())
                                .collect::<Vec<_>>();
                            let characters = characters
                                .iter()
                                .map(|c| c.to_owned().to_owned())
                                .collect::<Vec<_>>();
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
                                current.1 = character.id.0.clone();
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
                                .description(Locale::from(get_score_calc(&current.0)).get(&lang))
                                .attachment(&filename)
                                .footer(footer)
                                .color(convert_rgb(character.element.color_rgb()));
                            let at = CreateAttachment::bytes(img, filename);
                            let res = EditInteractionResponse::new()
                                .new_attachment(at)
                                .components(create_components(characters, &data.api, &lang, &uid))
                                .embed(e);
                            select_menu.edit_response(&ctx.http, res).await?;
                        }
                    }
                    ComponentInteractionDataKind::Button => {
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

fn main() -> anyhow::Result<()> {
    logger::logger_init();
    dotenv::dotenv().ok();
    let api = EnkaNetwork::new()?;
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .enable_all()
        .build()?
        .block_on(_main(api))?;
    Ok(())
}

async fn _main(api: EnkaNetwork) -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let icons = IconData::load(&api).await;
    let data = Data {
        api,
        icons,
        cache: HashMap::new(),
        looping: false,
    };
    let frame = poise::Framework::new(
        poise::FrameworkOptions {
            commands: vec![commands::build()],
            listener: |event, framework, user_data| {
                Box::pin(event_event_handler(event, framework, user_data))
            },
            ..Default::default()
        },
        |ctx, _ready, framework| {
            Box::pin(async move {
                let cmd = poise::builtins::create_application_commands(
                    framework.options().commands.as_slice(),
                );
                serenity::all::Command::set_global_application_commands(&ctx.http, cmd).await?;
                Ok(Arc::new(Mutex::new(data)))
            })
        },
    );
    let mut client = serenity::Client::builder(token, serenity::all::GatewayIntents::GUILDS)
        .framework(frame)
        .cache_settings(|f| {
            f.cache_users = false;
            f.max_messages = 0;
            f
        })
        .await?;

    client.start_autosharded().await?;
    Ok(())
}
