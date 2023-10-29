use std::time::{Duration, SystemTime};

use serde::Deserialize;
use serde_json::Value;

use crate::character::Character;

pub struct ApiRawUser {
    contents: Vec<u8>,
    uid: i32,
    modtime: SystemTime,
}

impl ApiRawUser {
    pub fn from_raw(buf: Vec<u8>, uid: i32, modtime: SystemTime) -> Self {
        Self {
            contents: buf,
            uid,
            modtime,
        }
    }
    pub fn uid(&self) -> i32 {
        self.uid
    }
    pub fn contents(&self) -> &[u8] {
        &self.contents
    }
    pub fn resolve(&self) -> Result<ApiUser, String> {
        ApiUser::from_raw(&self.contents, self.uid, self.modtime).ok_or("invalid json".to_string())
    }
}

#[derive(Debug)]
pub struct ApiUser {
    pub characters: Vec<Character>,
    pub uid: i32,
    pub name: String,
    pub level: u8,
    pub world_level: u8,
    pub friend_count: u8,
    pub description: String,
    pub lastupdate: SystemTime,
    pub avatar_id: String,
    pub avatar_name: String,
    pub avatar_icon: String,
    pub advancements: u64,
    pub avatar_count: u64,
}

impl ApiUser {
    pub fn uid(&self) -> i32 {
        self.uid
    }
    pub fn reload_time(&self) -> SystemTime {
        self.lastupdate + Duration::new(30, 0)
    }
    pub fn from_raw(buf: &Vec<u8>, uid: i32, modtime: SystemTime) -> Option<Self> {
        let value: Value = serde_json::from_slice(buf).ok()?;
        let value = value.as_object()?;
        let profile = value.get("player")?.as_object()?;
        let characters = value.get("characters")?.as_array()?;
        let name = profile.get("nickname")?.as_str()?.to_string();
        let description = profile.get("signature")?.as_str().unwrap_or("").to_string();
        let world_level = profile.get("world_level")?.as_i64()? as u8;
        let level = profile.get("level")?.as_i64()? as u8;
        let friend_count = profile.get("friend_count")?.as_i64()? as u8;
        let avatar = profile.get("avatar")?.as_object()?;
        let space = profile.get("space_info")?.as_object()?;
        let mut cs = Vec::with_capacity(characters.len());
        for c in characters {
            cs.push(match Character::deserialize(c) {
                Ok(c) => c,
                Err(e) => {
                    println!("failed to deserialize character: {}", e);
                    continue;
                }
            });
        }
        Some(Self {
            characters: cs,
            uid,
            name,
            description,
            world_level,
            level,
            friend_count,
            lastupdate: modtime,
            avatar_id: avatar.get("id")?.as_str()?.to_string(),
            avatar_name: avatar.get("name")?.as_str()?.to_string(),
            avatar_icon: avatar.get("icon")?.as_str()?.to_string(),
            advancements: space.get("achievement_count")?.as_u64()?,
            avatar_count: space.get("avatar_count")?.as_u64()?,
        })
    }
}
