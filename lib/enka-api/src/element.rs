use std::str::FromStr;

use crate::store::Store;

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Element {
    Fire,
    Water,
    Wind,
    Electric,
    Grass,
    Ice,
    Rock,
    None,
}
impl FromStr for Element {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fire" => Ok(Self::Fire),
            "Water" => Ok(Self::Water),
            "Wind" => Ok(Self::Wind),
            "Electric" => Ok(Self::Electric),
            "Grass" => Ok(Self::Grass),
            "Ice" => Ok(Self::Ice),
            "Rock" => Ok(Self::Rock),
            "Physical" => Ok(Self::None),
            "None" => Ok(Self::None),
            _ => Err(String::from("unknown element")),
        }
    }
}
impl ToString for Element {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl Element {
    pub fn color_rgb(&self) -> [u8; 3] {
        match self {
            Self::Fire => [255, 153, 85],
            Self::Water => [62, 153, 255],
            Self::Wind => [128, 255, 230],
            Self::Electric => [179, 128, 255],
            Self::Grass => [165, 200, 59],
            Self::Ice => [85, 221, 255],
            Self::Rock => [255, 204, 0],
            Self::None => [255, 255, 255],
        }
    }
    pub fn fight_prop_name(&self) -> &'static str {
        match self {
            Self::Fire => "FIRE",
            Self::Water => "WATER",
            Self::Wind => "WIND",
            Self::Electric => "ELEC",
            Self::Grass => "GRASS",
            Self::Ice => "ICE",
            Self::Rock => "ROCK",
            Self::None => "PHYSICAL",
        }
    }
    pub fn attack_name<'a>(&self, store: &'a Store, language: impl AsRef<str>) -> &'a str {
        let res = store.locale(
            language,
            format!("FIGHT_PROP_{}_ADD_HURT", self.fight_prop_name()),
        );
        match res {
            Some(v) => v,
            None => "",
        }
    }
    pub fn resist_name<'a>(&self, store: &'a Store, language: impl AsRef<str>) -> &'a str {
        let res = store.locale(
            language,
            format!("FIGHT_PROP_{}_SUB_HURT", self.fight_prop_name()),
        );
        match res {
            Some(v) => v,
            None => "",
        }
    }
}
