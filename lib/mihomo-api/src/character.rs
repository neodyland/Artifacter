use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Path {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub color: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Attribute {
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    pub percent: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Properties {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    pub percent: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LightCone {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub rank: u8,
    pub level: u64,
    pub promotion: u64,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: Path,
    pub attributes: Vec<Attribute>,
    pub properties: Vec<Properties>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RelicSet {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub num: u8,
    pub desc: String,
    pub properties: Vec<Properties>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubAffix {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    pub percent: bool,
    pub count: u8,
    pub step: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Relic {
    pub main_affix: Properties,
    pub sub_affix: Vec<SubAffix>,
    pub id: String,
    pub name: String,
    pub set_id: String,
    pub set_name: String,
    pub rarity: u8,
    pub level: u8,
    pub icon: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub max_level: u8,
    pub icon: String,
    pub r#type: String,
    pub type_text: String,
    pub effect: String,
    pub simple_desc: String,
    pub desc: String,
    pub element: Option<Element>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SkillTree {
    pub id: String,
    pub level: u8,
    pub anchor: String,
    pub max_level: u8,
    pub icon: String,
    pub parent: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub rank: u8,
    pub level: u8,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: Path,
    pub element: Element,
    pub rank_icons: Vec<String>,
    pub promotion: u64,
    pub light_cone: Option<LightCone>,
    pub properties: Vec<Properties>,
    pub attributes: Vec<Attribute>,
    pub additions: Vec<Attribute>,
    pub relic_sets: Vec<RelicSet>,
    pub skills: Vec<Skill>,
    pub skill_trees: Vec<SkillTree>,
    pub relics: Vec<Relic>,
}
