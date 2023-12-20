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
        self.inner.entry(uid).or_insert_with(|| CacheValue {
            format: "png".to_string(),
            score: None,
            character: None,
        });
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

pub struct HsrCache {
    inner: BTreeMap<i32, HsrCacheValue>,
}

impl HsrCache {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
    pub fn update(&mut self, uid: i32, k: String, v: String) {
        self.inner.entry(uid).or_insert_with(|| HsrCacheValue {
            format: "png".to_string(),
            score: None,
            character: None,
            base_img: None,
        });
        if let Some(value) = self.inner.get_mut(&uid) {
            match k.as_str() {
                "hsr_format" => value.format = v,
                "hsr_score" => value.score = Some(v),
                "hsr_character" => {
                    value.character = Some(v);
                    value.score = None;
                }
                "hsr_base_img" => value.base_img = Some(v),
                _ => {}
            };
        }
    }

    pub fn get_or_default(&self, uid: i32) -> Option<&HsrCacheValue> {
        if let Some(value) = self.inner.get(&uid) {
            return Some(value);
        }
        None
    }
}

pub struct HsrCacheValue {
    pub format: String,
    pub score: Option<String>,
    pub character: Option<String>,
    pub base_img: Option<String>,
}
