use enkanetwork_rs::{Element, ReliquaryType};
use image::{
    imageops::FilterType::{self, Triangle},
    DynamicImage,
};

const BASE_ELECTRIC: &[u8; 1238638] = include_bytes!("../../assets/base/electric.png");
const BASE_FIRE: &[u8; 1395828] = include_bytes!("../../assets/base/fire.png");
const BASE_GRASS: &[u8; 1338229] = include_bytes!("../../assets/base/grass.png");
const BASE_WATER: &[u8; 617017] = include_bytes!("../../assets/base/water.png");
const BASE_ICE: &[u8; 1343022] = include_bytes!("../../assets/base/ice.png");
const BASE_ROCK: &[u8; 1443299] = include_bytes!("../../assets/base/rock.png");
const BASE_WIND: &[u8; 534511] = include_bytes!("../../assets/base/wind.png");

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

struct Scores {
    ss: f64,
    s: f64,
    a: f64,
}

fn get_scores_for_part(part: Option<ReliquaryType>) -> Scores {
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

const GRADES_B: &[u8; 45056] = include_bytes!("../../assets/grades/B.png");
const GRADES_A: &[u8; 39600] = include_bytes!("../../assets/grades/A.png");
const GRADES_S: &[u8; 58084] = include_bytes!("../../assets/grades/S.png");
const GRADES_SS: &[u8; 82990] = include_bytes!("../../assets/grades/SS.png");

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

const RARITY_1: &[u8; 2342] = include_bytes!("../../assets/rarity/1.png");
const RARITY_2: &[u8; 2945] = include_bytes!("../../assets/rarity/2.png");
const RARITY_3: &[u8; 3166] = include_bytes!("../../assets/rarity/3.png");
const RARITY_4: &[u8; 3400] = include_bytes!("../../assets/rarity/4.png");
const RARITY_5: &[u8; 3449] = include_bytes!("../../assets/rarity/5.png");

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
