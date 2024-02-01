use poise::{serenity_prelude as serenity, FrameworkContext};
use serenity::{Command, FullEvent};

use crate::state::{Error, State};
mod interaction;
mod ready;

type Framework<'a> = FrameworkContext<'a, State, Error>;

pub async fn handler(
    event: &FullEvent,
    frame: FrameworkContext<'_, State, Error>,
) -> Result<(), Error> {
    let state = frame.user_data();
    let ctx = frame.serenity_context;
    match event {
        FullEvent::Ready { data_about_bot } => {
            let cmd =
                poise::builtins::create_application_commands(frame.options().commands.as_slice());
            Command::set_global_commands(&ctx.http, cmd.as_slice()).await?;
            ready::handler(frame, &state, ctx, data_about_bot).await?;
        }
        FullEvent::InteractionCreate { interaction } => {
            interaction::handler(frame, &state, ctx, interaction).await?;
        }
        _ => {}
    }
    Ok(())
}
