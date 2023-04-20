// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use base64::{engine::general_purpose, Engine as _};
use enkanetwork_rs::{CharacterId, EnkaNetwork, IconData, StoreValue};
use gen::{convert, generate as rs_generate, ImageFormat, Lang, ScoreCounter};
use once_cell::sync::OnceCell;
use serde::Serialize;

static ENKA: OnceCell<EnkaNetwork> = OnceCell::new();

static ICON_DATA: OnceCell<IconData> = OnceCell::new();

static STORE: OnceCell<StoreValue> = OnceCell::new();

#[derive(Debug, Clone)]
enum Td {
    String(String),
    Int(u32),
    U8(u8),
}

impl Serialize for Td {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Td::String(s) => serializer.serialize_str(s),
            Td::Int(i) => serializer.serialize_u32(*i),
            Td::U8(i) => serializer.serialize_u8(*i),
        }
    }
}

impl From<String> for Td {
    fn from(s: String) -> Self {
        Td::String(s)
    }
}

impl From<u32> for Td {
    fn from(i: u32) -> Self {
        Td::Int(i)
    }
}

impl From<u8> for Td {
    fn from(i: u8) -> Self {
        Td::U8(i)
    }
}

pub async fn load(enka: EnkaNetwork) -> Option<()> {
    let icon_data = enka.icon_data().await;
    let store = enka.store().await.ok()?;
    ENKA.set(enka).ok()?;
    ICON_DATA.set(icon_data).ok()?;
    STORE.set(store).ok()?;
    Some(())
}

// get profile
#[tauri::command]
async fn get_profile(uid: i32) -> Result<Vec<Td>, String> {
    let enka = ENKA.get().ok_or("EnkaNetwork not loaded".to_string())?;
    let user = enka.simple(uid).await?;
    let namecard = user
        .profile()
        .name_card_image(enka)
        .await
        .map(|f| convert(f, ImageFormat::Png));
    let v = (
        user.profile().nickname(),
        user.profile().signature(),
        user.profile().achievement(),
        user.profile().level(),
        user.profile().world_level(),
        user.profile().tower_floor_index(),
        user.profile().tower_level_index(),
        namecard.flatten().unwrap_or(vec![]),
    );
    let mut array = Vec::new();
    array.push(Td::from(v.0.to_owned()));
    array.push(Td::from(v.1.to_owned()));
    array.push(Td::from(v.2.to_owned()));
    array.push(Td::from(v.3.to_owned()));
    array.push(Td::from(v.4.to_owned()));
    array.push(Td::from(v.5.to_owned()));
    array.push(Td::from(v.6.to_owned()));
    array.push(Td::from(
        general_purpose::STANDARD_NO_PAD.encode(v.7.as_slice()),
    ));
    Ok(array)
}

/// get characters
#[tauri::command]
async fn get_characters(uid: i32, lang: String) -> Result<Vec<Vec<Td>>, String> {
    let lang = Lang::from(lang.as_str());
    let enka = ENKA.get().ok_or("EnkaNetwork not loaded".to_string())?;
    let user = enka.simple(uid).await?;
    let cv = user.characters_vec();
    let mut v = vec![];
    for x in cv {
        let ic = x.image_icon(&enka).await;
        let ic = ic
            .map(|f| convert(f, ImageFormat::Png))
            .flatten()
            .unwrap_or(vec![]);
        v.push((
            x.id.0.clone(),
            x.name(&enka, &lang.to_string()).unwrap_or("Unknown"),
            x.level.clone(),
            x.element.fight_prop_name(),
            general_purpose::STANDARD_NO_PAD.encode(ic.as_slice()),
        ))
    }
    let mut array = Vec::new();
    for x in v {
        array.push(
            [
                Td::from(x.0.to_owned()),
                Td::from(x.1.to_owned()),
                Td::from(x.2.to_owned()),
                Td::from(x.3.to_owned()),
                Td::from(x.4.to_owned()),
            ]
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
        );
    }
    Ok(array)
}

// generate
#[tauri::command]
async fn generate(
    uid: i32,
    cid: u32,
    lang: String,
    format: String,
    counter: String,
) -> Result<String, String> {
    let lang = Lang::from(lang.as_str());
    let enka = ENKA.get().ok_or("EnkaNetwork not loaded".to_string())?;
    let icon_data = ICON_DATA.get().ok_or("IconData not loaded".to_string())?;
    let user = enka.simple(uid).await?;
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
    let result = result.unwrap();
    let str = general_purpose::STANDARD_NO_PAD.encode(result.as_slice());
    Ok(str)
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(_main(EnkaNetwork::new().unwrap()));
}

async fn _main(enka: EnkaNetwork) {
    load(enka).await;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_profile,
            get_characters,
            generate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
