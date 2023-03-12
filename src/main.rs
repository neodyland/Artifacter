use std::{borrow::Cow, collections::HashMap, env, io::Cursor, sync::Arc};

use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::ScoreCounter;
use image::ImageOutputFormat;
use poise::serenity_prelude::{
    self as serenity, Activity, AttachmentType, CreateComponents, CreateEmbed, Interaction,
    InteractionResponseType,
};
use tokio::sync::Mutex;

use crate::util::create_components;

mod gen;
mod util;
struct Data {
    pub api: EnkaNetwork,
    pub icons: IconData,
    pub cache: HashMap<u64, (ScoreCounter, u32)>,
    pub channel: u64,
    pub looping: bool,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Arc<Mutex<Data>>, Error>;

/// UIDからデータを取得します
#[poise::command(slash_command)]
async fn build(ctx: Context<'_>, #[description = "ユーザーID"] uid: i32) -> Result<(), Error> {
    if uid.to_string().len() != 9 {
        ctx.send(|b| b.content("ユーザーIDは9桁の数字でなければなりません。"))
            .await?;
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
        ctx.send(|b| b.content("キャラクターが登録されていません。"))
            .await?;
        return Ok(());
    }
    ctx.send(|b| {
        b.components(|c| {
            create_components(c, characters, api, "ja", &uid);
            c
        })
        .embed(|e| {
            let rgb = [0x00, 0xff, 0x00];
            e.color(convert_rgb(rgb));
            e.title("キャラクターを選択してください").footer(|f| {
                f.text(format!("{}", uid));
                f
            });
            e
        })
    })
    .await?;
    Ok(())
}

fn convert_rgb(rgb: [u8; 3]) -> u32 {
    let [r, g, b] = rgb;
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

async fn event_event_handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Arc<Mutex<Data>>, Error>,
    data: &Arc<Mutex<Data>>,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name);
            let ctx = Arc::new(ctx.to_owned());
            if data.lock().await.looping == false {
                data.lock().await.looping = true;
                tokio::spawn(async move {
                    loop {
                        let activity =
                            Activity::playing(&format!("{} guilds", ctx.cache.guild_count()));
                        ctx.set_activity(activity).await;
                        tokio::time::sleep(std::time::Duration::from_secs(20)).await;
                    }
                });
            }
        }
        poise::Event::InteractionCreate { interaction } => {
            if let Interaction::MessageComponent(select_menu) = interaction {
                if select_menu.data.custom_id == "character"
                    || select_menu.data.custom_id == "score"
                {
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
                    let characters = ids.iter().map(|id| user.character(*id)).collect::<Vec<_>>();
                    let characters = characters
                        .iter()
                        .filter_map(|c| c.as_ref())
                        .collect::<Vec<_>>();
                    let characters = characters
                        .iter()
                        .map(|c| c.to_owned().to_owned())
                        .collect::<Vec<_>>();
                    let lang = "ja";
                    let value = select_menu.data.values.first();
                    let mut current = data
                        .cache
                        .remove(&select_menu.message.id.0)
                        .unwrap_or((ScoreCounter::Normal, 114514));
                    if select_menu.data.custom_id == "score" {
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
                            .insert(select_menu.message.id.0, current.to_owned());
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
                            .insert(select_menu.message.id.0, current.to_owned());
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
                    img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)?;
                    let msg = ctx
                        .http
                        .send_files(
                            data.channel,
                            vec![AttachmentType::Bytes {
                                data: Cow::Owned(image_data),
                                filename: "image.png".to_string(),
                            }],
                            &serde_json::Map::new(),
                        )
                        .await?;
                    let _ = select_menu
                        .edit_original_interaction_response(&ctx.http, |f| {
                            f.embed(|f| {
                                f.title("キャラクターを選択してください");
                                f.image(&msg.attachments[0].url);
                                f.footer(|f| {
                                    f.text(format!("{}", uid));
                                    f
                                });
                                f.color(convert_rgb(character.element.color_rgb()));
                                f
                            })
                            .components(|b| {
                                create_components(b, characters, &data.api, "ja", &uid);
                                b
                            })
                        })
                        .await;
                } else if &select_menu.data.custom_id == "end" {
                    let e = select_menu.message.embeds.first();
                    if e.is_none() {
                        return Ok(());
                    }
                    let e = e.unwrap();
                    let mut e = CreateEmbed::from(e.to_owned());
                    e.title("終了しました");
                    select_menu
                        .create_interaction_response(&ctx.http, |f| {
                            f.kind(InteractionResponseType::UpdateMessage)
                                .interaction_response_data(|d| {
                                    d.add_embed(e).set_components(CreateComponents(vec![]))
                                })
                        })
                        .await?;
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
        channel: env::var("CHANNEL")
            .expect("Expected a channel id in the environment")
            .parse()
            .unwrap(),
        looping: false,
    };
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![build()],
            event_handler: |ctx, event, framework, user_data| {
                Box::pin(event_event_handler(ctx, event, framework, user_data))
            },
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::GUILDS)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(&ctx.http, &framework.options().commands)
                    .await?;
                Ok(Arc::new(Mutex::new(data)))
            })
        });

    framework.run().await.unwrap();
    Ok(())
}
