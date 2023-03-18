use std::{collections::HashMap, env, sync::Arc};

extern crate line_bot_sdk_rust as line;
#[macro_use]
extern crate rocket;

use crate::util::{create_components, json, Locale};
use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::{ImageFormat, Lang, ScoreCounter};
use line::bot::LineBot;
use line::events::messages::MessageType as EventMessageType;
use line::events::{EventType, Events};
use line::messages::{SendMessageType, TextMessage};
use line::support::rocket_support::{Body, Signature};
use rocket::State;
use rocket::http::Status;
use serde_json::Value;
use tokio::sync::Mutex;

struct Environment {
    pub line_channel_secret: str,
    pub line_access_token: str,
}
//mod util;
struct Data {
    pub api: EnkaNetwork,
    pub icons: IconData,
    pub cache: HashMap<u64, (ScoreCounter, u32, ImageFormat)>,
    pub looping: bool,
}
type Error = Box<dyn std::error::Error + Send + Sync>;

fn get_score_calc(s: &ScoreCounter) -> Value {
    match s {
        ScoreCounter::Normal => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + 攻撃力(%)", "en": "Critical Rate × 2 + Critical Damage + Attack(%)"})
        }
        ScoreCounter::Hp => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + HP(%)", "en": "Critical Rate × 2 + Critical Damage + HP(%)"})
        }
        ScoreCounter::Def => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + 防御(%)", "en": "Critical Rate × 2 + Critical Damage + Defense(%)"})
        }
        ScoreCounter::ElementalMastery => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + (熟知 ÷ 4)", "en": "Critical Rate × 2 + Critical Damage + (Elemental Mastery ÷ 4)"})
        }
        ScoreCounter::ChargeEfficiency => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + 元素チャージ効率", "en": "Critical Rate × 2 + Critical Damage + Elemental Charge Efficiency"})
        }
    }
}

#[post("/", data = "<body>")]
fn callback(signature: Signature, body: Body, environment: State<Environment>, data: State<Data>) -> Status {
    let bot = LineBot::new(&environment.line_channel_secret, &environment.line_access_token);
    let result: Result<Events, &'static str> =
        bot.parse_event_request(&signature.key, &body.string);
    
    match result {
        Ok(res) => {
            for event in res.events {
                if let EventType::MessageEvent(message_event) = event.r#type {
                    // TextMessageEvent only
                    if let EventMessageType::TextMessage(text_message) = message_event.message.r#type {
                        // Create TextMessage
                        let message = SendMessageType::TextMessage(TextMessage {
                            text: text_message.text,
                            emojis: None,
                        });
                        // Reply message with reply_token
                        let _res = bot.reply_message(&message_event.reply_token, vec![message]);
                    }
                }
            }
            return Status::new(200, "OK");
        },
        Err(msg) => {
            return Status::new(500, msg);
        },
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let line_channel_secret =
        env::var("LINE_CHANNEL_SECRET").expect("Failed getting LINE_CHANNEL_SECRET");
    let line_access_token =
        env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed getting LINE_CHANNEL_ACCESS_TOKEN");
    let environment = Environment {
        line_channel_secret,
        line_access_token,
    };

    let api = EnkaNetwork::new()?;
    let icons = IconData::load(&api).await;
    let data = Data {
        api,
        icons,
        cache: HashMap::new(),
        looping: false,
    };

    rocket::ignite()
        .mount("/", routes![callback])
        .manage(environment)
        .manage(data)
        .launch();

    Ok(())
}
