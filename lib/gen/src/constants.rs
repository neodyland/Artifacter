use enka_api::{character::ReliquaryType, element::Element};
use gen_utils::get_grade_image as r_get_grade_image;
use gen_utils::get_rarity_image as r_get_rarity_image;
use image::imageops::FilterType::Triangle;
use image::load_from_memory;
use image::{imageops::FilterType, DynamicImage};
use once_cell::sync::Lazy;
use rusttype::Font;

pub static FONT: Lazy<Font> =
    Lazy::new(|| Font::try_from_bytes(include_bytes!("../../../assets/font.ttf")).unwrap());
static BASE_ELECTRIC: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/base/electric.png")).unwrap());
static BASE_FIRE: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/base/fire.png")).unwrap());
static BASE_GRASS: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/base/grass.png")).unwrap());
static BASE_WATER: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/base/water.png")).unwrap());
static BASE_ICE: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/base/ice.png")).unwrap());
static BASE_ROCK: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/base/rock.png")).unwrap());
static BASE_WIND: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/base/wind.png")).unwrap());
pub static DUPE: &str = include_str!("../../../assets/dupe.json");
pub static SUBOP: &str = include_str!("../../../assets/subop.json");
pub fn get_base_image(kind: &Element) -> Option<DynamicImage> {
    match kind {
        Element::Electric => Some(BASE_ELECTRIC.clone()),
        Element::Fire => Some(BASE_FIRE.clone()),
        Element::Grass => Some(BASE_GRASS.clone()),
        Element::Water => Some(BASE_WATER.clone()),
        Element::Ice => Some(BASE_ICE.clone()),
        Element::Rock => Some(BASE_ROCK.clone()),
        Element::Wind => Some(BASE_WIND.clone()),
        Element::None => None,
    }
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
static ELECTRIC: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/electric.png")).unwrap());
static ELECTRIC_LOCKED: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/clock/electric_locked.png")).unwrap()
});
static FIRE: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/fire.png")).unwrap());
static FIRE_LOCKED: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/clock/fire_locked.png")).unwrap()
});
static GRASS: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/grass.png")).unwrap());
static GRASS_LOCKED: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/clock/grass_locked.png")).unwrap()
});
static ICE: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/ice.png")).unwrap());
static ICE_LOCKED: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/ice_locked.png")).unwrap());
static ROCK: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/rock.png")).unwrap());
static ROCK_LOCKED: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/clock/rock_locked.png")).unwrap()
});
static WATER: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/water.png")).unwrap());
static WATER_LOCKED: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/clock/water_locked.png")).unwrap()
});
static WIND: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/clock/wind.png")).unwrap());
static WIND_LOCKED: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/clock/wind_locked.png")).unwrap()
});

pub fn get_clock_image(f: impl AsRef<str>, locked: bool) -> Option<&'static DynamicImage> {
    Some(
        match (f.as_ref().to_lowercase(), locked) {
            (f, true) if f == "electric" || f == "elec" => &ELECTRIC_LOCKED,
            (f, true) if f == "fire" => &FIRE_LOCKED,
            (f, true) if f == "grass" => &GRASS_LOCKED,
            (f, true) if f == "ice" => &ICE_LOCKED,
            (f, true) if f == "rock" => &ROCK_LOCKED,
            (f, true) if f == "water" => &WATER_LOCKED,
            (f, true) if f == "wind" => &WIND_LOCKED,
            (f, false) if f == "electric" || f == "elec" => &ELECTRIC,
            (f, false) if f == "fire" => &FIRE,
            (f, false) if f == "grass" => &GRASS,
            (f, false) if f == "ice" => &ICE,
            (f, false) if f == "rock" => &ROCK,
            (f, false) if f == "water" => &WATER,
            (f, false) if f == "wind" => &WIND,
            _ => return None,
        }
    )
}

pub fn get_grade_image(score: f64, part: Option<ReliquaryType>) -> Option<DynamicImage> {
    let scores = get_scores_for_part(part);
    let grade = r_get_grade_image(if score >= scores.ss {
        "SS"
    } else if score >= scores.s {
        "S"
    } else if score >= scores.a {
        "A"
    } else {
        "B"
    });
    grade.map(|i| i.resize(45, 45, FilterType::Nearest))
}

pub fn get_rarity_image(rarity: u8) -> Option<DynamicImage> {
    r_get_rarity_image(rarity).map(|i| {
        i.resize_exact(
            (i.width() as f32 * 0.9).round() as u32,
            (i.height() as f32 * 0.9).round() as u32,
            Triangle,
        )
    })
}
