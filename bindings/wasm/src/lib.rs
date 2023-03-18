use js_sys::{Array, JsString, Number};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use base64::{engine::general_purpose, Engine as _};
use enkanetwork_rs::{CharacterId, EnkaNetwork, IconData};
use gen::{generate as rs_generate, ImageFormat, Lang, ScoreCounter};
use once_cell::sync::OnceCell;
use std::str::FromStr;

static ENKA: OnceCell<EnkaNetwork> = OnceCell::new();

static ICON_DATA: OnceCell<IconData> = OnceCell::new();

/// Load
#[wasm_bindgen]
pub async fn w_load() -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let enka = EnkaNetwork::new_wasm()
        .await
        .map_err(|e| JsError::new(&e.to_string()))?;
    let icon_data = enka.icon_data().await;
    let e = ENKA.set(enka);
    if e.is_err() {
        return Err(JsError::new("EnkaNetwork already loaded").into());
    }
    let e = ICON_DATA.set(icon_data);
    if e.is_err() {
        return Err(JsError::new("IconData already loaded").into());
    }
    Ok(JsValue::UNDEFINED)
}

/// get characters
#[wasm_bindgen]
pub async fn get_characters(uid: i32, lang: String) -> Result<JsValue, JsValue> {
    let lang = Lang::from(lang.as_str());
    let enka = ENKA.get().ok_or(JsError::new("EnkaNetwork not loaded"))?;
    let user = enka.simple(uid).await.map_err(|e| JsError::new(&e))?;
    let v = &user
        .profile()
        .show_character_list()
        .to_owned()
        .iter()
        .map(|x| {
            (
                x.0,
                user.character(x.to_owned())
                    .unwrap()
                    .name(&enka, &lang.to_string())
                    .unwrap_or("Unknown"),
            )
        })
        .collect::<Vec<_>>();
    let array = Array::new();
    for x in v {
        array.push(
            &[
                &JsValue::from(Number::from(x.0.to_owned())),
                &JsValue::from(JsString::from(x.1.to_owned())),
            ]
            .iter()
            .cloned()
            .collect::<Array>(),
        );
    }
    Ok(array.into())
}
/// generater
#[wasm_bindgen]
pub async fn generate(
    uid: i32,
    cid: u32,
    lang: String,
    format: String,
    counter: String,
) -> Result<JsValue, JsValue> {
    let lang = Lang::from(lang.as_str());
    let enka = ENKA.get().ok_or(JsError::new("EnkaNetwork not loaded"))?;
    let icon_data = ICON_DATA.get().ok_or(JsError::new("IconData not loaded"))?;
    let user = enka.simple(uid).await.map_err(|e| JsError::new(&e))?;
    let character = user.character(CharacterId(cid));
    if character.is_none() {
        return Err(JsError::new("Character not found").into());
    }
    let character = character.unwrap();
    let counter = ScoreCounter::from_str(counter.as_str()).map_err(|e| JsError::new(&e))?;
    let format = ImageFormat::from_str(format.as_str()).map_err(|e| JsError::new(&e))?;
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
        return Err(JsError::new("Generate failed").into());
    }
    let result = result.unwrap();
    let str = general_purpose::STANDARD_NO_PAD.encode(result.as_slice());
    let res = JsString::from(str);
    Ok(res.into())
}
