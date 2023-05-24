#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use enkanetwork_rs::{CharacterId, EnkaNetwork, IconData};
use gen::{generate as rs_generate, ImageFormat, Lang, ScoreCounter};
use napi::{bindgen_prelude::Buffer, Error, Result, Status};
use once_cell::sync::OnceCell;
use std::str::FromStr;

static ENKA: OnceCell<EnkaNetwork> = OnceCell::new();

static ICON_DATA: OnceCell<IconData> = OnceCell::new();

/// Load
#[napi]
pub async fn load() -> Result<()> {
    let enka = tokio::task::spawn_blocking(move || Some(EnkaNetwork::new().ok()))
        .await
        .map_err(|_| Error::new(Status::GenericFailure, "EnkaNetwork load failed"))?;
    let enka = enka.ok_or(Error::new(
        Status::GenericFailure,
        "EnkaNetwork load failed",
    ))?;
    let enka = enka.ok_or(Error::new(
        Status::GenericFailure,
        "EnkaNetwork load failed",
    ))?;
    let icon_data = enka.icon_data().await;
    let e = ENKA.set(enka);
    if e.is_err() {
        return Err(Error::new(
            Status::GenericFailure,
            "EnkaNetwork already loaded",
        ));
    }
    let e = ICON_DATA.set(icon_data);
    if e.is_err() {
        return Err(Error::new(
            Status::GenericFailure,
            "IconData already loaded",
        ));
    }
    Ok(())
}

/// get characters
#[napi]
pub async fn get_characters(uid: i32) -> Result<Vec<u32>> {
    let enka = ENKA
        .get()
        .ok_or(Error::new(Status::GenericFailure, "EnkaNetwork not loaded"))?;
    let (user, _) = enka
        .simple(uid)
        .await
        .map_err(|e| Error::new(Status::GenericFailure, e))?;
    Ok(user
        .profile()
        .show_character_list()
        .to_owned()
        .iter()
        .map(|x| x.0)
        .collect::<Vec<u32>>())
}

/// generater
#[napi]
pub async fn generate(
    uid: i32,
    cid: u32,
    lang: String,
    format: String,
    counter: String,
) -> Result<Buffer> {
    let lang = Lang::from(lang.as_str());
    let enka = ENKA
        .get()
        .ok_or(Error::new(Status::GenericFailure, "EnkaNetwork not loaded"))?;
    let icon_data = ICON_DATA
        .get()
        .ok_or(Error::new(Status::GenericFailure, "IconData not loaded"))?;
    let (user, _) = enka
        .simple(uid)
        .await
        .map_err(|e| Error::new(Status::GenericFailure, e))?;
    let character = user.character(CharacterId(cid));
    if character.is_none() {
        return Err(Error::new(Status::GenericFailure, "Character not found"));
    }
    let character = character.unwrap();
    let counter = ScoreCounter::from_str(counter.as_str())
        .map_err(|e| Error::new(Status::GenericFailure, e))?;
    let format = ImageFormat::from_str(format.as_str())
        .map_err(|e| Error::new(Status::GenericFailure, e))?;
    let result = rs_generate(
        character.to_owned(),
        enka,
        &lang,
        icon_data,
        counter,
        format,
    )
    .await;
    if result.is_none() {
        return Err(Error::new(Status::GenericFailure, "Generate failed"));
    }
    let result = result.unwrap();
    Ok(result.into())
}
