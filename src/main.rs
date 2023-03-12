use std::{collections::HashMap, env, io::Cursor, sync::Arc};

use crate::util::create_components;
use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::ScoreCounter;
use image::ImageOutputFormat;
use poise::{
    serenity_prelude::{
        ComponentInteractionDataKind, CreateAttachment, CreateEmbed, CreateEmbedFooter,
        CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
        Interaction,
    },
    CreateReply,
};
use serenity::gateway::ActivityData;
use tokio::sync::Mutex;

mod gen;
mod util;
struct Data {
    pub api: EnkaNetwork,
    pub icons: IconData,
    pub cache: HashMap<u64, (ScoreCounter, u32)>,
    pub looping: bool,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Arc<Mutex<Data>>, Error>;

/// UIDからデータを取得します
#[poise::command(slash_command)]
async fn build(ctx: Context<'_>, #[description = "ユーザーID"] uid: i32) -> Result<(), Error> {
    if uid.to_string().len() != 9 {
        let msg = CreateReply::new().content("ユーザーIDは9桁の数字でなければなりません。");
        ctx.send(msg).await?;
        return Ok(());
    }
    ctx.defer().await?;
    let data = ctx.data();
    let api = &data.lock().await.api;
    let user = api.simple(uid).await?;
    let ids = user.profile().show_character_list();
    let characters = ids.iter().map(|id| user.character(*id)).collect::<Vec<_>>();
    let characters = characters
        .iter()
        .filter_map(|c| c.as_ref())
        .collect::<Vec<_>>();
    let characters = characters
        .iter()
        .map(|c| c.to_owned().to_owned())
        .collect::<Vec<_>>();
    if characters.is_empty() {
        let msg = CreateReply::new().content("キャラクターが登録されていません。");
        ctx.send(msg).await?;
        return Ok(());
    }
    let footer = CreateEmbedFooter::new(format!("{}", uid));
    let embed = CreateEmbed::default()
        .title("キャラクターを選択してください")
        .footer(footer)
        .color(convert_rgb([0x00, 0xff, 0x00]));
    let builder = CreateReply::default()
        .components(create_components(characters, api, "ja", &uid))
        .embed(embed);
    ctx.send(builder).await?;
    Ok(())
}

fn convert_rgb(rgb: [u8; 3]) -> u32 {
    let [r, g, b] = rgb;
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

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
            println!("{} is connected!", data_about_bot.user.name);
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
                match select_menu.data.kind.clone() {
                    ComponentInteractionDataKind::StringSelect { values } => {
                        if &custom_id == "character" || &custom_id == "score" {
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
                            let lang = "ja";
                            let mut current = data
                                .cache
                                .remove(&select_menu.message.id.get())
                                .unwrap_or((ScoreCounter::Normal, 114514));
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
                                    .insert(select_menu.message.id.get(), current.to_owned());
                            } else {
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
                                data.cache
                                    .insert(select_menu.message.id.get(), current.to_owned());
                            }
                            let character = characters.iter().find(|c| c.id.0 == current.1);
                            if character.is_none() {
                                return Ok(());
                            }
                            let character = character.unwrap().to_owned().to_owned();
                            let img = gen::generate(
                                character.to_owned(),
                                &data.api,
                                &lang,
                                &data.icons,
                                current.0,
                            )
                            .await;
                            if img.is_none() {
                                return Ok(());
                            }
                            let img = img.unwrap();
                            let mut image_data: Vec<u8> = Vec::new();
                            img.write_to(
                                &mut Cursor::new(&mut image_data),
                                ImageOutputFormat::Png,
                            )?;
                            let footer = CreateEmbedFooter::new(format!("{}", uid));
                            let e = CreateEmbed::default()
                                .title(format!("{}のステータス", character.name(&data.api, lang)?))
                                .image("attachment://image.png")
                                .footer(footer)
                                .color(convert_rgb(character.element.color_rgb()));
                            let at = CreateAttachment::bytes(image_data, "image.png");
                            let res = EditInteractionResponse::new()
                                .new_attachment(at)
                                .components(create_components(characters, &data.api, "ja", &uid))
                                .embed(e);
                            select_menu.edit_response(&ctx.http, res).await?;
                        }
                    }
                    ComponentInteractionDataKind::Button => {
                        if &custom_id == "end" {
                            let e = select_menu.message.embeds.first();
                            if e.is_none() {
                                return Ok(());
                            }
                            let e = e.unwrap();
                            let e = CreateEmbed::from(e.to_owned()).title("終了しました");
                            let res = CreateInteractionResponseMessage::default()
                                .add_embed(e)
                                .components(vec![]);
                            let res = CreateInteractionResponse::UpdateMessage(res);
                            select_menu.create_response(&ctx.http, res).await?;
                        }
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
    dotenv::dotenv().ok();
    let api = EnkaNetwork::new()?;
    tokio::runtime::Builder::new_current_thread()
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
            commands: vec![build()],
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
