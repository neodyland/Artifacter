use std::collections::BTreeMap;

pub struct Cache {
    inner: BTreeMap<i32, CacheValue>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
    pub fn update(&mut self, uid: i32, k: String, v: String) {
        if !self.inner.contains_key(&uid) {
            self.inner.insert(
                uid,
                CacheValue {
                    format: "png".to_string(),
                    score: None,
                    character: None,
                },
            );
        }
        if let Some(value) = self.inner.get_mut(&uid) {
            match k.as_str() {
                "format" => value.format = v,
                "score" => value.score = Some(v),
                "character" => {
                    value.character = Some(v);
                    value.score = None;
                }
                _ => {}
            };
        }
    }

    pub fn get_or_default(&self, uid: i32) -> Option<&CacheValue> {
        if let Some(value) = self.inner.get(&uid) {
            return Some(value);
        }
        None
    }
}

pub struct CacheValue {
    pub format: String,
    pub score: Option<String>,
    pub character: Option<String>,
}
