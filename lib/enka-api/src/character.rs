use std::{borrow::Cow, collections::HashMap, fmt::Display, str::FromStr};

use image::DynamicImage;
use serde_json::{Number, Value};

use crate::element::Element;
use crate::util::{ascension_level_map, get_or_null};
use crate::{api::Api, fight_prop::FightProp};

#[derive(Clone, Debug)]
pub struct Character {
    pub id: CharacterId,
    pub element: Element,
    name: u32,
    talents: Vec<CharacterTalent>,
    skills: Vec<CharacterSkill>,
    reliquarys: Vec<Reliquary>,
    weapon: Weapon,
    costumes: HashMap<u32, Costume>,
    current_costume: Option<u64>,
    gachaslice: Option<String>,
    gachasplash: Option<String>,
    icon: Option<String>,
    iconcard: Option<String>,
    fight_prop: FightProp,
    friendship: u8,
    pub level: u8,
    pub ascension: u8,
    pub xp: u32,
}
impl Character {
    pub fn talents(&self) -> &Vec<CharacterTalent> {
        &self.talents
    }
    pub fn name<'a>(&self, api: &'a Api, language: impl AsRef<str>) -> Result<&'a str, String> {
        api.get_store()
            .locale(language, &format!("{}", self.name))
            .ok_or_else(|| String::from("no locale data"))
    }
    pub fn costumes(&self) -> Vec<&Costume> {
        self.costumes.values().collect()
    }
    pub fn current_costume(&self) -> Option<&Costume> {
        self.costumes.get(&(self.current_costume? as u32))
    }
    pub fn name_gacha_slice(&self) -> Option<&String> {
        self.gachaslice.as_ref()
    }
    pub fn name_gacha_splash(&self) -> Option<&String> {
        self.gachasplash.as_ref()
    }
    pub fn name_icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }
    pub fn name_iconcard(&self) -> Option<&String> {
        self.iconcard.as_ref()
    }
    pub fn skills(&self) -> &Vec<CharacterSkill> {
        &self.skills
    }
    pub async fn image_icon(&self, api: &Api) -> Option<DynamicImage> {
        api.ui_image(self.name_icon()?).await.ok()
    }
    pub async fn image_iconcard(&self, api: &Api) -> Option<DynamicImage> {
        api.ui_image(self.name_iconcard()?).await.ok()
    }
    pub async fn image_gacha_slice(&self, api: &Api) -> Option<DynamicImage> {
        let name = match self.name_gacha_slice() {
            Some(p) => Cow::Borrowed(p),
            None => Cow::Owned(self.icon.as_ref()?.replace("AvatarIcon", "Gacha_AvatarImg")),
        };
        api.ui_image(name.as_ref()).await.ok()
    }
    pub async fn image_gacha_splash(&self, api: &Api) -> Option<DynamicImage> {
        let name = match self.name_gacha_splash() {
            Some(p) => Cow::Borrowed(p),
            None => Cow::Owned(self.icon.as_ref()?.replace("AvatarIcon", "Gacha_AvatarImg")),
        };
        api.ui_image(name.as_ref()).await.ok()
    }
    pub fn friendship(&self) -> u8 {
        match self.id.0 {
            // Player
            10000005 => 0,
            10000007 => 0,
            _ => self.friendship,
        }
    }
    pub fn fight_prop(&self) -> &FightProp {
        &self.fight_prop
    }
    pub fn reliquarys(&self) -> &Vec<Reliquary> {
        &self.reliquarys
    }
    pub fn weapon(&self) -> &Weapon {
        &self.weapon
    }
    pub fn ascension_level(&self) -> u8 {
        ascension_level_map(self.ascension)
    }
}
#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub struct CharacterId(pub u32);
pub(crate) fn parse_character(api: &Api, player_character: &Value) -> Option<Character> {
    let avatar_id = player_character.get("avatarId")?.as_u64()? as u32;
    let character_id_str = if avatar_id == 10000005 || avatar_id == 10000007 {
        let depot_id = match player_character.get("skillDepotId") {
            Some(depot_id) => depot_id.to_string(),
            None => format!("{}01", avatar_id.to_string().get(7..7).unwrap()),
        };
        format!("{}-{}", avatar_id, depot_id)
    } else {
        format!("{}", avatar_id)
    };
    let store = api.get_store();
    let characters = store.characters.get(character_id_str)?.as_object()?;
    let mut talents = vec![];
    if let Some(consts) = characters.get("Consts") {
        if let Some(consts) = consts.as_array() {
            let talent_count = {
                match player_character.get("talentIdList") {
                    Some(consts) => match consts.as_array() {
                        Some(arr) => arr.len(),
                        None => 0,
                    },
                    None => 0,
                }
            };
            let mut index = 0;
            for name in consts {
                let talent = CharacterTalent {
                    image: name.as_str()?.to_owned(),
                    unlock: index < talent_count,
                };
                index += 1;
                talents.push(talent);
            }
        }
    }
    fn find(map: &serde_json::Map<String, Value>, key: &str) -> Option<String> {
        Some(String::from(map.get(key)?.as_str()?))
    }
    let mut skills: Vec<CharacterSkill> = vec![];
    {
        let extra_map = get_or_null(player_character, "proudSkillExtraLevelMap");
        let skill_level_map = get_or_null(player_character, "skillLevelMap");
        let skill_level_map = skill_level_map.as_object();
        if let Some(skill_level_map) = skill_level_map {
            let skill_image = characters.get("Skills")?;
            let mut skill_map = HashMap::with_capacity(4);
            for (id, level) in skill_level_map {
                let image = match skill_image.get(id) {
                    Some(v) => match v.as_str() {
                        Some(str) => Some(str.to_owned()),
                        None => None,
                    },
                    None => None,
                };
                let extra_level = match characters.get("ProudMap") {
                    Some(map) => match map.get(id) {
                        Some(id) => match extra_map.get(id.to_string()) {
                            Some(v) => match v.as_u64() {
                                Some(v) => v as u8,
                                None => 0,
                            },
                            None => 0,
                        },
                        None => 0,
                    },
                    None => 0,
                };
                let id = u64::from_str(id).ok()? as u32;
                let s = CharacterSkill {
                    extra_level,
                    id,
                    level: level.as_u64()? as u8,
                    image,
                };
                skill_map.insert(id, s);
            }
            let skill_order = characters.get("SkillOrder");
            if let Some(skill_order) = skill_order {
                if let Some(skill_order) = skill_order.as_array() {
                    for skill_id in skill_order {
                        if let Some(skill_id) = skill_id.as_u64() {
                            if let Some(s) = skill_map.remove(&(skill_id as u32)) {
                                skills.push(s);
                            }
                        }
                    }
                }
            }
        }
    }
    let current_costume = match player_character.get("costumeId") {
        Some(json) => json.as_u64(),
        None => None,
    };
    let mut costumes = HashMap::new();
    if let Some(costumes_list) = characters.get("Costumes") {
        if let Some(costumes_list) = costumes_list.as_object() {
            for (id, costume_json) in costumes_list {
                fn load_costume(id: &String, useing: bool, json: &Value) -> Option<Costume> {
                    fn str_or(json: &Value, key: &str) -> Option<String> {
                        Some(json.get(key)?.as_str()?.to_owned())
                    }
                    Some(Costume {
                        id: u32::from_str(id).ok()?,
                        useing,
                        art: str_or(json, "art"),
                        icon: str_or(json, "icon"),
                        side_icon: str_or(json, "sideIconName"),
                    })
                }
                let is_useing = match current_costume {
                    Some(costume_id) => costume_id.to_string().as_str() == id,
                    None => false,
                };
                if let Some(c) = load_costume(id, is_useing, costume_json) {
                    costumes.insert(c.id, c);
                }
            }
        }
    }
    let friendship = match player_character.get("fetterInfo") {
        Some(fetter_info) => match fetter_info.get("expLevel") {
            Some(exp) => match exp.as_u64() {
                Some(friendship) => friendship as u8,
                None => 1u8,
            },
            None => 1u8,
        },
        None => 1u8,
    };
    let prop_map = player_character.get("propMap")?;
    fn prop(prop_map: &Value, key: &str) -> Option<u64> {
        let s = prop_map.get(key)?.get("val")?.as_str()?;
        u64::from_str(s).ok()
    }
    let (weapon, reliquarys) = parse_equip_list(player_character.get("equipList")?.as_array()?);
    Some(Character {
        id: CharacterId(avatar_id),
        fight_prop: FightProp::from_json(player_character.get("fightPropMap")?),
        element: Element::from_str(characters.get("Element")?.as_str()?).ok()?,
        name: characters.get("NameTextMapHash")?.as_u64()? as u32,
        talents,
        skills,
        reliquarys,
        weapon: weapon?,
        costumes,
        current_costume,
        gachaslice: find(characters, "namegachaslice").or(find(characters, "filename_gachaslice")),
        gachasplash: find(characters, "namegachasplash")
            .or(find(characters, "filename_gachasplash")),
        icon: find(characters, "nameicon").or(find(characters, "filename_icon")),
        iconcard: find(characters, "nameiconcard").or(find(characters, "filename_iconcard")),
        friendship,
        xp: prop(prop_map, "1001").unwrap_or(0) as u32,
        ascension: prop(prop_map, "1002").unwrap_or(0) as u8,
        level: prop(prop_map, "4001").unwrap_or(0) as u8,
    })
}
//命の星座
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CharacterTalent {
    image: String,
    unlock: bool,
}
impl CharacterTalent {
    pub fn is_unlock(&self) -> bool {
        self.unlock
    }
    pub fn path(&self) -> &String {
        &self.image
    }
    pub async fn image(&self, api: &Api) -> Result<DynamicImage, String> {
        if self.image.is_empty() {
            return Err(String::from("None"));
        }
        api.ui_image(&self.image).await
    }
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Costume {
    id: u32,
    useing: bool,
    art: Option<String>,
    icon: Option<String>,
    side_icon: Option<String>,
}
impl Costume {
    pub async fn image_art(&self, api: &Api) -> Option<DynamicImage> {
        api.ui_image(self.art.as_ref()?).await.ok()
    }
    pub async fn image_icon(&self, api: &Api) -> Option<DynamicImage> {
        api.ui_image(self.icon.as_ref()?).await.ok()
    }
    pub async fn image_side_icon(&self, api: &Api) -> Option<DynamicImage> {
        api.ui_image(self.side_icon.as_ref()?).await.ok()
    }
    pub fn is_useing(&self) -> bool {
        self.useing
    }
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CharacterSkill {
    id: u32,
    level: u8,
    extra_level: u8,
    image: Option<String>,
}
impl CharacterSkill {
    pub fn level(&self) -> u8 {
        self.level
    }
    pub fn extra_level(&self) -> u8 {
        self.extra_level
    }
    pub async fn image(&self, api: &Api) -> Result<DynamicImage, String> {
        match &self.image {
            Some(path) => api.ui_image(path).await,
            None => Err("no image".to_string()),
        }
    }
}
#[derive(Hash, Copy, Clone, Debug)]
pub enum Stats {
    Hp,
    Attack,
    Defense,
    HpPercent,
    AttackPercent,
    DefensePercent,
    Critical,
    CriticalHurt,
    ChargeEfficiency,
    Heal,
    ElementMastery,
    ElementAddHurt(Element),
    None,
}
impl Stats {
    fn parse(t: impl AsRef<str>) -> Self {
        match t.as_ref() {
            "FIGHT_PROP_HP" => Self::Hp,
            "FIGHT_PROP_ATTACK" => Self::Attack,
            "FIGHT_PROP_DEFENSE" => Self::Defense,
            "FIGHT_PROP_HP_PERCENT" => Self::HpPercent,
            "FIGHT_PROP_ATTACK_PERCENT" => Self::AttackPercent,
            "FIGHT_PROP_DEFENSE_PERCENT" => Self::DefensePercent,
            "FIGHT_PROP_CRITICAL" => Self::Critical,
            "FIGHT_PROP_CRITICAL_HURT" => Self::CriticalHurt,
            "FIGHT_PROP_CHARGE_EFFICIENCY" => Self::ChargeEfficiency,
            "FIGHT_PROP_HEAL_ADD" => Self::Heal,
            "FIGHT_PROP_ELEMENT_MASTERY" => Self::ElementMastery,
            "FIGHT_PROP_PHYSICAL_ADD_HURT" => Self::ElementAddHurt(Element::None),
            "FIGHT_PROP_FIRE_ADD_HURT" => Self::ElementAddHurt(Element::Fire),
            "FIGHT_PROP_ELEC_ADD_HURT" => Self::ElementAddHurt(Element::Electric),
            "FIGHT_PROP_WATER_ADD_HURT" => Self::ElementAddHurt(Element::Water),
            "FIGHT_PROP_WIND_ADD_HURT" => Self::ElementAddHurt(Element::Wind),
            "FIGHT_PROP_ICE_ADD_HURT" => Self::ElementAddHurt(Element::Ice),
            "FIGHT_PROP_ROCK_ADD_HURT" => Self::ElementAddHurt(Element::Rock),
            "FIGHT_PROP_GRASS_ADD_HURT" => Self::ElementAddHurt(Element::Grass),
            _ => Self::None,
        }
    }
    pub fn id(&self) -> Cow<str> {
        match self {
            Self::Hp => Cow::Borrowed("FIGHT_PROP_HP"),
            Self::Attack => Cow::Borrowed("FIGHT_PROP_ATTACK"),
            Self::Defense => Cow::Borrowed("FIGHT_PROP_DEFENSE"),
            Self::HpPercent => Cow::Borrowed("FIGHT_PROP_HP_PERCENT"),
            Self::AttackPercent => Cow::Borrowed("FIGHT_PROP_ATTACK_PERCENT"),
            Self::DefensePercent => Cow::Borrowed("FIGHT_PROP_DEFENSE_PERCENT"),
            Self::Critical => Cow::Borrowed("FIGHT_PROP_CRITICAL"),
            Self::CriticalHurt => Cow::Borrowed("FIGHT_PROP_CRITICAL_HURT"),
            Self::ChargeEfficiency => Cow::Borrowed("FIGHT_PROP_CHARGE_EFFICIENCY"),
            Self::Heal => Cow::Borrowed("FIGHT_PROP_HEAL_ADD"),
            Self::ElementMastery => Cow::Borrowed("FIGHT_PROP_ELEMENT_MASTERY"),
            Self::ElementAddHurt(e) => {
                Cow::Owned(format!("FIGHT_PROP_{}_ADD_HURT", e.fight_prop_name()))
            }
            _ => Cow::Borrowed(""),
        }
    }
    pub fn name<'a>(&self, api: &'a Api, language: impl AsRef<str>) -> Option<&'a str> {
        api.get_store().locale(language, self.id())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct StatsValue(pub Stats, pub f64);
impl Display for StatsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Stats::Hp | Stats::Attack | Stats::Defense | Stats::ElementMastery | Stats::None => {
                write!(f, "{}", self.1.round() as u64)
            }
            _ => write!(f, "{:.*}%", 1, self.1),
        }
    }
}
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReliquaryType {
    Flower,
    Feather,
    Sands,
    Goblet,
    Circlet,
}
impl ReliquaryType {
    fn parse(t: impl AsRef<str>) -> Result<Self, ()> {
        Ok(match t.as_ref() {
            "EQUIP_BRACER" => Self::Flower,
            "EQUIP_NECKLACE" => Self::Feather,
            "EQUIP_SHOES" => Self::Sands,
            "EQUIP_RING" => Self::Goblet,
            "EQUIP_DRESS" => Self::Circlet,
            _ => return Err(()),
        })
    }
}
#[derive(Clone, Debug)]
pub struct Reliquary {
    name: u64,
    set_name: u32,
    icon: String,
    pub position: ReliquaryType,
    pub id: u32,
    pub level: u8,
    pub rarity: u8,
    pub main_stats: StatsValue,
    pub sub_stats: [Option<StatsValue>; 4],
}
impl Reliquary {
    pub fn set_name_hash(&self) -> u32 {
        self.set_name
    }
    pub fn set_name<'a>(&self, api: &'a Api, language: impl AsRef<str>) -> Option<&'a str> {
        api.get_store().locale(language, self.set_name.to_string())
    }
    pub fn name_hash(&self) -> u64 {
        self.name
    }
    pub fn name<'a>(&self, api: &'a Api, language: impl AsRef<str>) -> Option<&'a str> {
        api.get_store().locale(language, self.name.to_string())
    }
    pub fn name_icon(&self) -> &String {
        &self.icon
    }
    pub async fn image_icon(&self, api: &Api) -> Result<DynamicImage, String> {
        api.ui_image(self.name_icon()).await
    }
}
#[derive(Clone, Debug)]
pub struct Weapon {
    name: u64,
    icon: String,
    pub id: u32,
    pub level: u8,
    pub ascension: u8,
    pub refinement: u8,
    pub rarity: u8,
    pub base_attack: i32,
    pub stats: Option<StatsValue>,
}
impl Weapon {
    pub fn name_hash(&self) -> u64 {
        self.name
    }
    pub fn name<'a>(&self, api: &'a Api, language: impl AsRef<str>) -> Option<&'a str> {
        api.get_store().locale(language, self.name.to_string())
    }
    pub fn name_icon(&self) -> &String {
        &self.icon
    }
    pub async fn image_icon(&self, api: &Api) -> Result<DynamicImage, String> {
        api.ui_image(self.name_icon()).await
    }
    pub fn ascension_level(&self) -> u8 {
        ascension_level_map(self.ascension)
    }
}
fn parse_equip_list(list: &Vec<Value>) -> (Option<Weapon>, Vec<Reliquary>) {
    let mut weapon = None;
    let mut reliquarys = vec![];
    for entry in list {
        if entry.get("reliquary").is_some() {
            if let Some(r) = parse_equip_reliquary(entry) {
                println!("{:?}", r);
                reliquarys.push(r);
            }
        } else if entry.get("weapon").is_some() {
            if let Some(w) = parse_equip_weapon(entry) {
                weapon = Some(w);
            }
        }
    }
    (weapon, reliquarys)
}

fn to_u64(v: &Value) -> Option<u64> {
    v.as_u64()
        .or_else(|| v.as_str().and_then(|s| u64::from_str(s).ok()))
}

fn parse_equip_reliquary(entry: &Value) -> Option<Reliquary> {
    let id = entry.get("itemId")?.as_u64()? as u32;
    let reliquary = entry.get("reliquary")?;
    let flat = entry.get("flat")?;
    let reliquary_substats = flat.get("reliquarySubstats");
    let mut sub_stats = [None, None, None, None];
    if let Some(reliquary_substats) = reliquary_substats {
        if let Some(reliquary_substats) = reliquary_substats.as_array() {
            let mut index = 0;
            for v in reliquary_substats {
                sub_stats[index] = parse_reliquary_stat(v, "appendPropId");
                index += 1;
            }
        }
    }
    Some(Reliquary {
        id,
        position: ReliquaryType::parse(flat.get("equipType")?.as_str()?).ok()?,
        icon: flat.get("icon")?.as_str()?.to_owned(),
        name: to_u64(flat.get("nameTextMapHash")?)?,
        set_name: to_u64(flat.get("setNameTextMapHash")?)? as u32,
        rarity: flat.get("rankLevel")?.as_u64()? as u8,
        level: (reliquary.get("level")?.as_u64()? - 1) as u8,
        main_stats: parse_reliquary_stat(flat.get("reliquaryMainstat")?, "mainPropId")?,
        sub_stats,
    })
}
fn parse_reliquary_stat(v: &Value, id_key: &str) -> Option<StatsValue> {
    Some(StatsValue(
        Stats::parse(v.get(id_key)?.as_str()?),
        v.get("statValue")?.as_f64()?,
    ))
}
fn parse_equip_weapon(entry: &Value) -> Option<Weapon> {
    let id = entry.get("itemId")?.as_u64()? as u32;
    let weapon = entry.get("weapon")?;
    let flat = entry.get("flat")?;
    let stats_list = flat.get("weaponStats")?.as_array()?;
    let mut base_attack = 0;
    let mut stats = None;
    for v in stats_list {
        let prop_name = v.get("appendPropId")?.as_str()?;
        let value = v.get("statValue")?.as_f64()?;
        match prop_name {
            "FIGHT_PROP_BASE_ATTACK" => base_attack = value as i32,
            a => stats = Some(StatsValue(Stats::parse(a), value)),
        }
    }
    Some(Weapon {
        id,
        level: weapon
            .get("level")
            .unwrap_or(&Value::Number(Number::from_str("0").ok()?))
            .as_u64()? as u8,
        ascension: weapon
            .get("promoteLevel")
            .unwrap_or(&Value::Number(Number::from_str("0").ok()?))
            .as_u64()? as u8,
        refinement: weapon.get("affixMap")?.get(format!("1{}", id))?.as_u64()? as u8,
        rarity: flat.get("rankLevel")?.as_u64()? as u8,
        icon: flat.get("icon")?.as_str()?.to_owned(),
        name: to_u64(flat.get("nameTextMapHash")?)?,
        base_attack,
        stats,
    })
}
