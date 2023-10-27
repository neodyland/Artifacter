use serde_json::Value;
use std::borrow::Cow;

pub fn get_or_null<'a>(v: &'a Value, key: &str) -> std::borrow::Cow<'a, Value> {
    match v.get(key) {
        Some(v) => Cow::Borrowed(v),
        None => Cow::Owned(Value::Null),
    }
}

pub fn ascension_level_map(al: u8) -> u8 {
    match al {
        0 => 20,
        1 => 40,
        2 => 50,
        3 => 60,
        4 => 70,
        5 => 80,
        6 => 90,
        _ => 0,
    }
}
