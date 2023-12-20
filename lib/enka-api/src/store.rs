use std::str::FromStr;

use serde_json::Value;

use crate::{
    constants::{LOC, NAMECARDS, UI_ICON},
    fight_prop::FightPropLocale,
    playerinfo::NameCard,
};

pub struct Store {
    loc: Value,
    pub(crate) namecards: Value,
    pub(crate) characters: Value,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            loc: Value::from_str(LOC).unwrap(),
            namecards: Value::from_str(NAMECARDS).unwrap(),
            characters: UI_ICON.clone(),
        }
    }
    pub fn locale(&self, language: impl AsRef<str>, key: impl AsRef<str>) -> Option<&str> {
        let map = self.loc.as_object()?;
        let lang = map.get(language.as_ref())?.as_object()?;
        lang.get(key.as_ref())?.as_str()
    }
    pub fn is_locale_available(&self, loc: impl AsRef<str>) -> bool {
        match self.loc.as_object() {
            Some(v) => v.contains_key(loc.as_ref()),
            None => false,
        }
    }
    pub fn locale_list(&self) -> Vec<&String> {
        match self.loc.as_object() {
            Some(v) => {
                let keys = v.keys();
                let mut list = Vec::with_capacity(keys.len());
                for x in keys {
                    list.push(x);
                }
                list
            }
            None => vec![],
        }
    }
    pub fn namecard_path(&self, id: NameCard) -> Result<&str, String> {
        let namecard_map = self
            .namecards
            .as_object()
            .ok_or_else(|| String::from("no namecards"))?;
        let json_value = namecard_map[&format!("{}", id.0)]
            .as_object()
            .ok_or_else(|| String::from("not found in map"))?;
        json_value["icon"]
            .as_str()
            .ok_or_else(|| String::from("not string"))
    }
    pub fn get_or_empty(&self, language: impl AsRef<str>, key: impl AsRef<str>) -> &str {
        self.locale(language, key).unwrap_or("")
    }
    pub fn fight_prop_locale(&self, language: impl AsRef<str>) -> Result<FightPropLocale, String> {
        if self.is_locale_available(&language) {
            Ok(FightPropLocale::parse(self, language))
        } else {
            Err("not available".to_owned())
        }
    }
}
