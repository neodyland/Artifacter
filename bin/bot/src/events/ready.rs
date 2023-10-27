use std::sync::{atomic::Ordering, Arc};

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
    if !state.started.load(Ordering::Relaxed) {
        state.started.store(true, Ordering::Relaxed);
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
