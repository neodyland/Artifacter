use poise::Command;
mod build;
mod link;
mod profile;
mod unlink;
use crate::state::{Error, State};

pub fn load() -> Vec<Command<State, Error>> {
    vec![
        profile::profile(),
        build::build(),
        link::link(),
        unlink::unlink(),
    ]
}
