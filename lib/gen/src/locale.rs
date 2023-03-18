use serde_json::Value;

use crate::Lang;

pub use serde_json::json;

pub struct Locale {
    pub ja: String,
    pub en: String,
}

impl Locale {
    pub fn get(&self, lang: &Lang) -> &str {
        match lang {
            Lang::Ja => &self.ja,
            Lang::En => &self.en,
        }
    }
}

impl From<Value> for Locale {
    fn from(value: Value) -> Self {
        let ja = value["ja"].as_str().unwrap_or("空白").to_owned();
        let en = value["en"].as_str().unwrap_or("Empty").to_owned();
        Self { ja, en }
    }
}
