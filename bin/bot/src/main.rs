use std::env;

use poise::serenity_prelude as serenity;
use state::State;

mod api;
mod cache;
mod commands;
mod component;
mod db;
mod events;
mod hsr_components;
mod logger;
mod state;
mod util;

#[tokio::main]
async fn main() {
    logger::logger_init();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");
    let state = State::new().await;
    let frame = poise::Framework::new(
        poise::FrameworkOptions {
            commands: commands::load(),
            event_handler: |event, framework, user_data| {
                Box::pin(events::handler(event, framework, user_data))
            },
            ..Default::default()
        },
        |ctx, _ready, framework| {
            Box::pin(async move {
                let cmd = poise::builtins::create_application_commands(
                    framework.options().commands.as_slice(),
                );
                serenity::all::Command::set_global_commands(&ctx.http, cmd).await?;
                Ok(state)
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
        .await
        .unwrap();
    client.start_autosharded().await.unwrap();
}
