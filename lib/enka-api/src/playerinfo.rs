use image::DynamicImage;
use serde_json::Value;

use crate::{api::Api, character::CharacterId, util::get_or_null};
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PlayerInfo {
    nickname: String,
    signature: String,
    level: u8,
    world_level: u8,
    achievement: u32,
    tower_floor_index: u8,
    tower_level_index: u8,
    name_card: Option<NameCard>,
    profile_picture: CharacterId,
    avatar_info_list: Vec<CharacterId>,
    name_card_list: Vec<NameCard>,
}

impl From<&Value> for PlayerInfo {
    fn from(v: &Value) -> Self {
        let mut avatar_info_list = vec![];
        if let Some(arr) = get_or_null(v, "showAvatarInfoList").as_array() {
            for item in arr {
                if let Some(id) = get_or_null(item, "avatarId").as_u64() {
                    avatar_info_list.push(CharacterId(id as u32));
                }
            }
        }
        let mut name_card_list = vec![];
        if let Some(arr) = get_or_null(v, "showNameCardIdList").as_array() {
            for item in arr {
                if let Some(id) = item.as_u64() {
                    name_card_list.push(NameCard(id as u32));
                }
            }
        }
        Self {
            nickname: get_or_null(v, "nickname").as_str().unwrap_or("").to_owned(),
            signature: get_or_null(v, "signature")
                .as_str()
                .unwrap_or("")
                .to_owned(),
            level: get_or_null(v, "level").as_u64().unwrap_or(0) as u8,
            world_level: get_or_null(v, "worldLevel").as_u64().unwrap_or(0) as u8,
            achievement: get_or_null(v, "finishAchievementNum").as_u64().unwrap_or(0) as u32,
            name_card: get_or_null(v, "nameCardId")
                .as_u64()
                .map(|f| NameCard(f as u32)),
            tower_floor_index: get_or_null(v, "towerFloorIndex").as_u64().unwrap_or(0) as u8,
            tower_level_index: get_or_null(v, "towerLevelIndex").as_u64().unwrap_or(0) as u8,
            profile_picture: CharacterId(
                get_or_null(get_or_null(v, "profilePicture").as_ref(), "avatarId")
                    .as_i64()
                    .unwrap_or(0) as u32,
            ),
            avatar_info_list,
            name_card_list,
        }
    }
}

impl PlayerInfo {
    pub fn nickname(&self) -> &String {
        &self.nickname
    }
    pub fn signature(&self) -> &String {
        &self.signature
    }
    pub fn level(&self) -> u8 {
        self.level
    }
    pub fn world_level(&self) -> u8 {
        self.world_level
    }
    pub fn achievement(&self) -> u32 {
        self.achievement
    }
    pub fn name_card(&self) -> Option<NameCard> {
        self.name_card
    }
    pub async fn name_card_image(&self, api: &Api) -> Option<DynamicImage> {
        if let Some(name_card) = self.name_card {
            if name_card.has_value() {
                return name_card.image(api).await.ok();
            }
        }
        None
    }
    pub fn profile_picture(&self) -> CharacterId {
        self.profile_picture
    }
    pub fn show_character_list(&self) -> &Vec<CharacterId> {
        &self.avatar_info_list
    }
    pub fn show_name_card_list(&self) -> &Vec<NameCard> {
        &self.name_card_list
    }
    pub fn tower_floor_index(&self) -> u8 {
        self.tower_floor_index
    }
    pub fn tower_level_index(&self) -> u8 {
        self.tower_level_index
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NameCard(pub(crate) u32);
impl NameCard {
    pub fn has_value(&self) -> bool {
        self.0 > 0
    }
    pub async fn image(&self, api: &Api) -> Result<DynamicImage, String> {
        if !self.has_value() {
            return Err(String::from("None"));
        }
        let store = api.get_store();
        api.ui_image(store.namecard_path(*self)?).await
    }
}
