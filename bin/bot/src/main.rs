use std::{env, sync::Arc};

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
mod tips;
mod util;

#[tokio::main]
async fn main() {
    logger::logger_init();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");
    let state = State::new().await;
    let frame = poise::Framework::new(poise::FrameworkOptions {
        commands: commands::load(),
        event_handler: |framework, event| Box::pin(events::handler(event, framework)),
        ..Default::default()
    });
    let mut cache_settings = serenity::Settings::default();
    cache_settings.cache_users = false;
    cache_settings.max_messages = 0;
    let mut client = serenity::Client::builder(&token, serenity::all::GatewayIntents::GUILDS)
        .framework(frame)
        .cache_settings(cache_settings)
        .data(Arc::new(state))
        .await
        .unwrap();
    client.start_autosharded().await.unwrap();
}
