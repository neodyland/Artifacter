use std::{collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;
use serde_json::Value;

const CHARACTERS: &str = include_str!("../../../dynamic-assets/characters.json");
const IMAGE_CHARACTERS: &str = include_str!("../../../dynamic-assets/image-characters.json");
pub const LOC: &str = include_str!("../../../dynamic-assets/loc.json");
pub const NAMECARDS: &str = include_str!("../../../dynamic-assets/namecards.json");

fn mearge_character_db(mut base: Value, db: Value) -> Option<Value> {
    let mut db_map = HashMap::new();
    let dbroot = db.as_object()?;
    for (_, value) in dbroot {
        if let Some(side_icon) = value["filename_sideIcon"].as_str() {
            db_map.insert(side_icon, value);
        }
    }
    let baseroot = base.as_object_mut()?;
    for (_, value) in baseroot {
        if let Some(side_icon) = value["SideIconName"].as_str() {
            if let Some(db_value) = db_map.get(side_icon) {
                value["filename_icon"] = db_value["filename_icon"].clone();
                value["filename_iconcard"] = db_value["filename_iconCard"].clone();
                value["filename_gachasplash"] = db_value["filename_gachaSplash"].clone();
                value["filename_gachaslice"] = db_value["filename_gachaSlice"].clone();
            }
        }
    }
    Some(base)
}

pub const UI_ICON: Lazy<Value> = Lazy::new(|| {
    let ui_icon = mearge_character_db(
        Value::from_str(CHARACTERS).unwrap(),
        Value::from_str(IMAGE_CHARACTERS).unwrap(),
    );
    ui_icon.unwrap()
});
