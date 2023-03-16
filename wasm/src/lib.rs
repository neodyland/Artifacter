use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use enkanetwork_rs::{CharacterId, EnkaNetwork, IconData};
use gen::{generate as rs_generate, ImageFormat, Lang, ScoreCounter};
use once_cell::sync::OnceCell;
use std::str::FromStr;
use itertools::join;
use base64::{Engine as _, engine::general_purpose};

static ENKA: OnceCell<EnkaNetwork> = OnceCell::new();

static ICON_DATA: OnceCell<IconData> = OnceCell::new();

/// Load
#[wasm_bindgen]
pub async fn w_load() -> Result<(),String> {
    let enka = EnkaNetwork::new().map_err(|e| e.to_string())?;
    let icon_data = enka.icon_data().await;
    let e = ENKA.set(enka);
    if e.is_err() {
        return Err("EnkaNetwork already loaded".to_string());
    }
    let e = ICON_DATA.set(icon_data);
    if e.is_err() {
        return Err("IconData already loaded".to_string());
    }
    Ok(())
}

/// get characters
#[wasm_bindgen]
pub async fn get_characters(uid: i32) -> Result<String,String> {
    let enka = ENKA
        .get()
        .ok_or("EnkaNetwork not loaded")?;
    let user = enka
        .simple(uid)
        .await?;
    Ok(join(user
        .profile()
        .show_character_list()
        .to_owned()
        .iter()
        .map(|x| x.0)
        .collect::<Vec<_>>(),","))
}

/// generater
#[wasm_bindgen]
pub async fn generate(
    uid: i32,
    cid: u32,
    lang: String,
    format: String,
    counter: String,
) -> Result<String,String> {
    let lang = Lang::from(lang.as_str());
    let enka = ENKA
        .get()
        .ok_or("EnkaNetwork not loaded")?;
    let icon_data = ICON_DATA
        .get()
        .ok_or("IconData not loaded")?;
    let user = enka
        .simple(uid)
        .await?;
    let character = user.character(CharacterId(cid));
    if character.is_none() {
        return Err("Character not found".to_string());
    }
    let character = character.unwrap();
    let counter = ScoreCounter::from_str(counter.as_str())?;
    let format = ImageFormat::from_str(format.as_str())?;
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
        return Err("Generate failed".to_string());
    }
    Ok(general_purpose::STANDARD_NO_PAD.encode(result.unwrap()))
}
