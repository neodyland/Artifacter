use enka_api::{character::ReliquaryType, element::Element};
use image::{
    imageops::FilterType::{self, Triangle},
    DynamicImage,
};

pub const FONT: &[u8; 11451544] = include_bytes!("../../../assets/font.ttf");
const BASE_ELECTRIC: &[u8; 1238638] = include_bytes!("../../../assets/base/electric.png");
const BASE_FIRE: &[u8; 1395828] = include_bytes!("../../../assets/base/fire.png");
const BASE_GRASS: &[u8; 1338229] = include_bytes!("../../../assets/base/grass.png");
const BASE_WATER: &[u8; 617017] = include_bytes!("../../../assets/base/water.png");
const BASE_ICE: &[u8; 1343022] = include_bytes!("../../../assets/base/ice.png");
const BASE_ROCK: &[u8; 1443299] = include_bytes!("../../../assets/base/rock.png");
const BASE_WIND: &[u8; 534511] = include_bytes!("../../../assets/base/wind.png");
const GRADES_B: &[u8; 45056] = include_bytes!("../../../assets/grades/B.png");
const GRADES_A: &[u8; 39600] = include_bytes!("../../../assets/grades/A.png");
const GRADES_S: &[u8; 58084] = include_bytes!("../../../assets/grades/S.png");
const GRADES_SS: &[u8; 82990] = include_bytes!("../../../assets/grades/SS.png");
const RARITY_1: &[u8; 2342] = include_bytes!("../../../assets/rarity/1.png");
const RARITY_2: &[u8; 2945] = include_bytes!("../../../assets/rarity/2.png");
const RARITY_3: &[u8; 3166] = include_bytes!("../../../assets/rarity/3.png");
const RARITY_4: &[u8; 3400] = include_bytes!("../../../assets/rarity/4.png");
const RARITY_5: &[u8; 3449] = include_bytes!("../../../assets/rarity/5.png");
pub const DUPE: &str = include_str!("../../../assets/dupe.json");
pub const SUBOP: &str = include_str!("../../../assets/subop.json");
pub fn get_base_image(kind: &Element) -> Option<DynamicImage> {
    match kind {
        Element::Electric => image::load_from_memory(BASE_ELECTRIC).ok(),
        Element::Fire => image::load_from_memory(BASE_FIRE).ok(),
        Element::Grass => image::load_from_memory(BASE_GRASS).ok(),
        Element::Water => image::load_from_memory(BASE_WATER).ok(),
        Element::Ice => image::load_from_memory(BASE_ICE).ok(),
        Element::Rock => image::load_from_memory(BASE_ROCK).ok(),
        Element::Wind => image::load_from_memory(BASE_WIND).ok(),
        Element::None => None,
    }
}

pub fn get_grade_image(score: f64, part: Option<ReliquaryType>) -> Option<DynamicImage> {
    let scores = get_scores_for_part(part);
    let grade: &[u8] = if score >= scores.ss {
        GRADES_SS
    } else if score >= scores.s {
        GRADES_S
    } else if score >= scores.a {
        GRADES_A
    } else {
        GRADES_B
    };
    image::load_from_memory(grade)
        .ok()
        .map(|i| i.resize(45, 45, FilterType::Nearest))
}

pub fn get_rarity_image(rarity: u8) -> Option<DynamicImage> {
    let rarity: &[u8] = match rarity {
        1 => RARITY_1,
        2 => RARITY_2,
        3 => RARITY_3,
        4 => RARITY_4,
        5 => RARITY_5,
        _ => return None,
    };
    image::load_from_memory(rarity).ok().map(|i| {
        i.resize_exact(
            (i.width() as f32 * 0.9).round() as u32,
            (i.height() as f32 * 0.9).round() as u32,
            Triangle,
        )
    })
}

pub struct Scores {
    ss: f64,
    s: f64,
    a: f64,
}

pub fn get_scores_for_part(part: Option<ReliquaryType>) -> Scores {
    if part.is_none() {
        return Scores {
            ss: 220.0,
            s: 200.0,
            a: 180.0,
        };
    }
    let part = part.unwrap();
    match part {
        ReliquaryType::Circlet => Scores {
            ss: 40.0,
            s: 35.0,
            a: 30.0,
        },
        ReliquaryType::Flower | ReliquaryType::Feather => Scores {
            ss: 50.0,
            s: 45.0,
            a: 40.0,
        },
        ReliquaryType::Sands => Scores {
            ss: 45.0,
            s: 40.0,
            a: 35.0,
        },
        ReliquaryType::Goblet => Scores {
            ss: 45.0,
            s: 40.0,
            a: 37.0,
        },
    }
}
const ELECTRIC: &[u8; 6968] = include_bytes!("../../../assets/clock/electric.png");
const ELECTRIC_LOCKED: &[u8; 7797] = include_bytes!("../../../assets/clock/electric_locked.png");
const FIRE: &[u8; 6599] = include_bytes!("../../../assets/clock/fire.png");
const FIRE_LOCKED: &[u8; 7447] = include_bytes!("../../../assets/clock/fire_locked.png");
const GRASS: &[u8; 6941] = include_bytes!("../../../assets/clock/grass.png");
const GRASS_LOCKED: &[u8; 7591] = include_bytes!("../../../assets/clock/grass_locked.png");
const ICE: &[u8; 6953] = include_bytes!("../../../assets/clock/ice.png");
const ICE_LOCKED: &[u8; 7761] = include_bytes!("../../../assets/clock/ice_locked.png");
const ROCK: &[u8; 6898] = include_bytes!("../../../assets/clock/rock.png");
const ROCK_LOCKED: &[u8; 7627] = include_bytes!("../../../assets/clock/rock_locked.png");
const WATER: &[u8; 6726] = include_bytes!("../../../assets/clock/water.png");
const WATER_LOCKED: &[u8; 7544] = include_bytes!("../../../assets/clock/water_locked.png");
const WIND: &[u8; 6071] = include_bytes!("../../../assets/clock/wind.png");
const WIND_LOCKED: &[u8; 7737] = include_bytes!("../../../assets/clock/wind_locked.png");

pub fn get_clock_image(f: impl AsRef<str>, locked: bool) -> Option<DynamicImage> {
    image::load_from_memory(match (f.as_ref().to_lowercase(), locked) {
        (f, true) if f == "electric" || f == "elec" => ELECTRIC_LOCKED,
        (f, true) if f == "fire" => FIRE_LOCKED,
        (f, true) if f == "grass" => GRASS_LOCKED,
        (f, true) if f == "ice" => ICE_LOCKED,
        (f, true) if f == "rock" => ROCK_LOCKED,
        (f, true) if f == "water" => WATER_LOCKED,
        (f, true) if f == "wind" => WIND_LOCKED,
        (f, false) if f == "electric" || f == "elec" => ELECTRIC,
        (f, false) if f == "fire" => FIRE,
        (f, false) if f == "grass" => GRASS,
        (f, false) if f == "ice" => ICE,
        (f, false) if f == "rock" => ROCK,
        (f, false) if f == "water" => WATER,
        (f, false) if f == "wind" => WIND,
        _ => return None,
    })
    .ok()
}
