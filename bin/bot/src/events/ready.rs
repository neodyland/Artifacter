use std::sync::Arc;

use poise::serenity_prelude as serenity;
use serenity::{ActivityData, Context, Ready};

use crate::state::{Error, State};

use super::Framework;

pub async fn handler(
    _frame: Framework<'_>,
    state: &State,
    ctx: &Context,
    ready: &Ready,
) -> Result<(), Error> {
    log::info!("{} is connected!", ready.user.name);
    let ctx = Arc::new(ctx.to_owned());
    let started = {
        let mut started = state.started.lock().await;
        if let Some(s) = ready.shard {
            started.insert(s.id.0)
        } else {
            false
        }
    };
    if started {
        tokio::spawn(async move {
            loop {
                let activity =
                    ActivityData::playing(&format!("{} guilds", ctx.cache.guild_count()));
                ctx.set_activity(Some(activity));
                tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            }
        });
    }
    Ok(())
}
