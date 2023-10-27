use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileQuery {
    pub uid: i32,
    pub lang: Option<String>,
    pub image_format: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub world_level: u8,
    pub level: u8,
    pub tower_floor_index: u8,
    pub tower_level_index: u8,
    pub uid: i32,
    pub achievement: u32,
    pub name: String,
    pub description: String,
    pub name_card: Option<String>,
    pub from_cache: bool,
    pub characters: Vec<UserCharacter>,
    pub lastupdate: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCharacter {
    pub ascension: u8,
    pub level: u8,
    pub element: String,
    pub xp: u32,
    pub name: String,
    pub icon: String,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateQuery {
    pub uid: i32,
    pub lang: Option<String>,
    pub image_format: Option<String>,
    pub cid: u32,
    pub counter: Option<String>,
}
