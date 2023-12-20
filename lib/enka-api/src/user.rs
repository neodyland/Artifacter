use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use serde_json::Value;

use crate::{
    api::Api,
    character::{parse_character, Character, CharacterId},
    playerinfo::PlayerInfo,
};

pub struct ApiRawUser {
    contents: Vec<u8>,
    uid: i32,
    lastupdate: SystemTime,
}

impl ApiRawUser {
    pub fn from_raw(contents: Vec<u8>, uid: i32, lastupdate: SystemTime) -> Self {
        Self {
            contents,
            uid,
            lastupdate,
        }
    }
    pub fn uid(&self) -> i32 {
        self.uid
    }
    pub fn lastupdate(&self) -> SystemTime {
        self.lastupdate
    }
    pub fn contents(&self) -> &[u8] {
        &self.contents
    }
    pub fn resolve(&self, api: &Api) -> Result<ApiUser, String> {
        let res: Result<Value, serde_json::Error> = serde_json::from_slice(&self.contents);
        match res {
            Ok(val) => match val.as_object() {
                Some(root) => {
                    let profile = PlayerInfo::from(
                        root.get("playerInfo")
                            .ok_or_else(|| String::from("no player info"))?,
                    );
                    let mut characters = HashMap::new();
                    if let Some(avatar_info_list) = root.get("avatarInfoList") {
                        if let Some(map) = avatar_info_list.as_array() {
                            for id in map {
                                if let Some(c) = parse_character(api, id) {
                                    characters.insert(c.id, c);
                                }
                            }
                        }
                    }
                    let ttl = match root.get("ttl") {
                        Some(ttl) => ttl.as_u64().unwrap_or(5 * 60) as u32,
                        None => 5 * 60,
                    };
                    Ok(ApiUser {
                        uid: self.uid,
                        ttl,
                        lastupdate: self.lastupdate,
                        profile,
                        characters,
                    })
                }
                None => Err(String::from("NoRootValue")),
            },
            Err(e) => Err(format!("{}", e)),
        }
    }
}

pub struct ApiUser {
    pub uid: i32,
    pub ttl: u32,
    pub lastupdate: SystemTime,
    pub profile: PlayerInfo,
    pub characters: HashMap<CharacterId, Character>,
}
impl ApiUser {
    pub fn reload_time(&self) -> SystemTime {
        self.lastupdate + Duration::new(self.ttl as u64, 0)
    }
    pub fn lastupdate(&self) -> SystemTime {
        self.lastupdate
    }
    pub fn profile(&self) -> &PlayerInfo {
        &self.profile
    }
    pub fn characters(&self) -> Vec<Character> {
        self.characters.values().cloned().collect()
    }
    pub fn character(&self, id: CharacterId) -> Option<&Character> {
        self.characters.get(&id)
    }
    pub fn uid(&self) -> i32 {
        self.uid
    }
    pub fn ttl_sec(&self) -> u32 {
        self.ttl
    }
}
