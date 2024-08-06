use poise::Command;
mod about;
mod build;
mod genshin_profile;
mod hsr;
mod hsr_link;
mod hsr_profile;
mod hsr_unlink;
mod link;
mod unlink;
mod zzz_link;
mod zzz_unlink;
use crate::state::{Error, State};

pub fn load() -> Vec<Command<State, Error>> {
    vec![
        genshin_profile::genshin_profile(),
        build::build(),
        link::link(),
        unlink::unlink(),
        hsr_link::hsr_link(),
        hsr_unlink::hsr_unlink(),
        hsr_profile::hsr_profile(),
        hsr::hsr(),
        about::about(),
        zzz_link::zzz_link(),
        zzz_unlink::zzz_unlink(),
    ]
}
