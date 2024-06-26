use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufWriter, Cursor},
    str::FromStr,
};

use constants::get_rarity_image;
use enka_api::{
    api::Api,
    character::Character,
    character::Reliquary,
    character::{ReliquaryType, Stats, StatsValue},
    element::Element,
    icon::IconData,
};
use image::{
    imageops::{
        self, overlay, resize,
        FilterType::{Nearest, Triangle},
    },
    DynamicImage, ImageOutputFormat, Rgba,
};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub use crate::default::get_default;
use serde::Deserialize;

use crate::{
    constants::{self, get_clock_image, FONT},
    dupe, locale,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub enum ImageFormat {
    #[default]
    Png,
    Jpeg,
    Pixel,
}

impl From<ImageFormat> for Option<ImageOutputFormat> {
    fn from(val: ImageFormat) -> Self {
        match val {
            ImageFormat::Png => Some(ImageOutputFormat::Png),
            ImageFormat::Jpeg => Some(ImageOutputFormat::Jpeg(20)),
            ImageFormat::Pixel => None,
        }
    }
}

impl ToString for ImageFormat {
    fn to_string(&self) -> String {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Pixel => "pixel",
        }
        .to_string()
    }
}

impl FromStr for ImageFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpeg" => Ok(ImageFormat::Jpeg),
            "pixel" => Ok(ImageFormat::Pixel),
            _ => Err(format!("{} is not ImageFormat", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
pub enum ScoreCounter {
    #[default]
    Normal,
    Hp,
    Def,
    ElementalMastery,
    ChargeEfficiency,
}

impl ScoreCounter {
    fn en(&self) -> &str {
        match self {
            ScoreCounter::Normal => "Attack",
            ScoreCounter::Hp => "Hp",
            ScoreCounter::Def => "Def",
            ScoreCounter::ElementalMastery => "Mastery",
            ScoreCounter::ChargeEfficiency => "Charge",
        }
    }
    fn ja(&self) -> &str {
        match self {
            ScoreCounter::Normal => "攻撃型",
            ScoreCounter::Hp => "HP型",
            ScoreCounter::Def => "防御型",
            ScoreCounter::ElementalMastery => "熟知型",
            ScoreCounter::ChargeEfficiency => "チャージ型",
        }
    }
    pub fn to_string_locale(&self, lang: &str) -> String {
        match lang {
            "ja" => self.ja().to_string(),
            _ => self.en().to_string(),
        }
    }
}

impl Display for ScoreCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.en())
    }
}

impl FromStr for ScoreCounter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(ScoreCounter::Normal),
            "Attack" => Ok(ScoreCounter::Normal),
            "Hp" => Ok(ScoreCounter::Hp),
            "Def" => Ok(ScoreCounter::Def),
            "ElementalMastery" => Ok(ScoreCounter::ElementalMastery),
            "ChargeEfficiency" => Ok(ScoreCounter::ChargeEfficiency),
            "normal" => Ok(ScoreCounter::Normal),
            "attack" => Ok(ScoreCounter::Normal),
            "hp" => Ok(ScoreCounter::Hp),
            "def" => Ok(ScoreCounter::Def),
            "elementalmastery" => Ok(ScoreCounter::ElementalMastery),
            "mastery" => Ok(ScoreCounter::ElementalMastery),
            "chargeefficiency" => Ok(ScoreCounter::ChargeEfficiency),
            "charge" => Ok(ScoreCounter::ChargeEfficiency),
            _ => Err(format!("{} is not ScoreCounter", s)),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub enum Lang {
    En,
    #[default]
    Ja,
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            Lang::En => "en",
            Lang::Ja => "ja",
        }
        .to_string()
    }
}

impl From<&str> for Lang {
    fn from(s: &str) -> Self {
        match s {
            "en" => Lang::En,
            "ja" => Lang::Ja,
            "en-US" => Lang::En,
            "en-GB" => Lang::En,
            _ => Lang::Ja,
        }
    }
}

pub async fn generate(
    data: Character,
    api: &Api,
    raw_lang: &Lang,
    icons: &IconData,
    counter: ScoreCounter,
    format: ImageFormat,
) -> Option<Vec<u8>> {
    let lang = &raw_lang.to_string();
    let font = &FONT;
    let mut image = constants::get_base_image(&data.element)?;
    let character_image = data.image_gacha_splash(api).await?;
    match data.id.0 {
        // liuyun
        10000093 => {
            let character_image = character_image.resize_exact(800, 600, Nearest);
            imageops::overlay(&mut image, &character_image, 0, 50);
        }
        // main characters
        10000005 | 10000007 => {
            let character_image = character_image.resize_exact(600, 600, Nearest);
            imageops::overlay(&mut image, &character_image, 150, 50);
        }
        _ => {
            let character_image = character_image.resize_exact(1200, 600, Nearest);
            imageops::overlay(&mut image, &character_image, -225, 50);
        }
    };
    let character_name = data.name(api, lang).ok()?;
    let character_level = format!("Lv.{},{}", data.level, data.friendship());
    let white = image::Rgba([255, 255, 255, 255]);
    draw_text_mut(
        &mut image,
        white,
        30,
        20,
        Scale::uniform(60.0),
        font,
        character_name,
    );
    draw_text_mut(
        &mut image,
        white,
        35,
        80,
        Scale::uniform(32.0),
        font,
        &character_level,
    );

    let scale = Scale::uniform(25.0);
    for (index, skill) in data.skills().iter().enumerate() {
        let img = skill.image(api).await.ok()?;
        let img = img.resize(80, 80, Triangle);
        overlay(&mut image, &img, 20, 330 + index as i64 * 100);
        let lv = skill.level() + skill.extra_level();
        let color = if lv > 9 {
            image::Rgba([0, 255, 255, 255])
        } else {
            white
        };
        draw_text_mut(
            &mut image,
            color,
            40,
            400 + index as i32 * 100,
            scale,
            font,
            &format!("Lv.{}", lv),
        );
    }

    let clocks = data.talents();
    for (index, clock) in clocks.iter().enumerate() {
        let locked_image = get_clock_image(data.element.fight_prop_name(), !clock.is_unlock())?;
        let mut locked_image = resize(locked_image, 70, 70, Triangle);
        if clock.is_unlock() {
            let img = clock.image(api).await.ok()?;
            let img = resize(&img, 35, 35, Triangle);
            overlay(&mut locked_image, &img, 16, 15);
        }
        overlay(&mut image, &locked_image, 680, 100 + 80 * index as i64);
    }

    let weapon = data.weapon();
    let weapon_img = weapon.image_icon(api).await.ok()?;
    let weapon_img = weapon_img.resize_exact(129, 128, Triangle);
    overlay(&mut image, &weapon_img, 1430, 50);
    let weapon_rarity_img = get_rarity_image(weapon.rarity)?;
    overlay(&mut image, &weapon_rarity_img, 1422, 173);
    let ascension = format!("R{}", weapon.refinement + 1);
    draw_text_mut(&mut image, white, 1435, 45, scale, font, &ascension);
    let weapon_level = format!("Lv.{}", weapon.level);
    let weapon_name = weapon.name(api, lang)?;
    let scale = Scale::uniform(30.0);
    draw_text_resized(&mut image, white, 1600, 45, scale, font, weapon_name, 250);
    draw_text_mut(&mut image, white, 1600, 85, scale, font, &weapon_level);
    let scale = Scale::uniform(25.0);
    let weapon_damage = format!("ATK:{}", weapon.base_attack);
    let mut damage_image = icons.image("FIGHT_PROP_ATTACK.svg", 1.8)?;
    for p in damage_image.pixels_mut() {
        p.0 = [255, 255, 255, p.0[3]];
    }
    draw_text_mut(&mut image, white, 1630, 125, scale, font, &weapon_damage);
    overlay(&mut image, &damage_image, 1600, 125);
    if let Some(stats) = weapon.stats {
        let weapon_sub = format!(
            "{} {}{}",
            stats.0.name(api, lang)?,
            stats.1,
            if is_percent(&stats.0) { "%" } else { "" }
        );
        let mut weapon_sub_image = stats.0.image(icons, 1.8)?;
        for p in weapon_sub_image.pixels_mut() {
            p.0 = [255, 255, 255, p.0[3]];
        }
        draw_text_mut(&mut image, white, 1630, 160, scale, font, &weapon_sub);
        overlay(&mut image, &weapon_sub_image, 1600, 160);
    }
    let artifacts = data.reliquarys();
    let mut artifact_x = 30;
    let mut artifact_scores = 0.0;
    for artifact in get_artifacts(artifacts) {
        if artifact.is_none() {
            artifact_x += 373;
            continue;
        }
        let artifact = artifact.unwrap();
        let gray = image::Rgba([240, 240, 240, 200]);
        if let Some(o) = dupe::resolve_op(artifact) {
            let mut sub_y = 785;
            let o = o
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    if let Some(s) = artifact.sub_stats[i] {
                        x.iter()
                            .map(|y| {
                                if is_percent(&s.0) {
                                    round_to_1_decimal_places(*y)
                                } else {
                                    y.to_string()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("+")
                    } else {
                        "".to_string()
                    }
                })
                .collect::<Vec<_>>();
            let scale = Scale::uniform(15.0);
            for x in o {
                let width = text_size(scale, font, &x).0;
                sub_y += 52;
                draw_text_mut(
                    &mut image,
                    gray,
                    artifact_x as i32 + 340 - width,
                    sub_y,
                    scale,
                    font,
                    &x,
                );
            }
        };
        let (score, used) = get_score(artifact, &counter);
        artifact_scores += score;
        let rank_img = constants::get_grade_image(score, Some(artifact.position))?;
        overlay(&mut image, &rank_img, artifact_x + 50, 1015);
        let score = round_to_1_decimal_places(score);
        let scale = Scale::uniform(40.0);
        let score_width = text_size(scale, font, &score).0;
        draw_text_mut(
            &mut image,
            white,
            artifact_x as i32 - score_width + 350,
            1015,
            scale,
            font,
            &score,
        );
        let img = artifact.image_icon(api).await.ok()?;
        let mut img = img.resize_exact(256, 256, Triangle).into_rgba8();
        img.pixels_mut().for_each(|p| {
            let p3 = p.0[3];
            if p3 > 100 {
                p.0[3] -= 100;
            } else if p3 > 20 {
                p.0[3] -= 20;
            }
        });
        overlay(&mut image, &img, artifact_x, 630);
        let main = artifact.main_stats;
        let main_type = main.0.name(api, lang)?;
        let main_value = if is_percent(&main.0) {
            format!("{}%", round_to_1_decimal_places(main.1))
        } else {
            main.1.to_string()
        };
        let mut main_image = main.0.image(icons, 1.8)?;
        for p in main_image.pixels_mut() {
            p.0 = [255, 255, 255, p.0[3]];
        }
        let scale = Scale::uniform(30.0);
        let main_type_width = text_size(scale, font, main_type).0;
        draw_text_mut(
            &mut image,
            white,
            artifact_x as i32 + 340 - main_type_width,
            660,
            scale,
            font,
            main_type,
        );
        let scale = Scale::uniform(60.0);
        let main_value_width = text_size(scale, font, &main_value).0;
        draw_text_mut(
            &mut image,
            white,
            artifact_x as i32 + 340 - main_value_width,
            690,
            scale,
            font,
            &main_value,
        );
        overlay(
            &mut image,
            &main_image,
            artifact_x + 310 - main_type_width as i64,
            660,
        );
        let level = format!("+{}", artifact.level);
        let scale = Scale::uniform(25.0);
        let level_width = text_size(scale, font, &level).0;
        draw_text_mut(
            &mut image,
            white,
            artifact_x as i32 + 340 - level_width,
            750,
            scale,
            font,
            &level,
        );
        let mut sub_y = 810;
        let gray = Rgba([240, 240, 240, 200]);
        for (_, sub) in artifact.sub_stats.iter().enumerate() {
            if sub.is_none() {
                sub_y += 40;
                continue;
            }
            let sub = sub.unwrap();
            let color = if used.contains(&sub.0.id().to_string()) {
                white
            } else {
                gray
            };
            let sub_type = sub.0.name(api, lang)?;
            let sub_value = if is_percent(&sub.0) {
                format!("{}%", round_to_1_decimal_places(sub.1))
            } else {
                sub.1.to_string()
            };
            let mut sub_image = sub.0.image(icons, 1.8)?;
            for p in sub_image.pixels_mut() {
                p.0 = [255, 255, 255, p.0[3]];
            }
            let scale = Scale::uniform(30.0);
            let sub_type_width = text_size(scale, font, sub_type).0;
            if sub_type_width <= 200 {
                draw_text_mut(
                    &mut image,
                    color,
                    artifact_x as i32 + 60,
                    sub_y,
                    scale,
                    font,
                    sub_type,
                );
            } else {
                draw_text_mut(
                    &mut image,
                    color,
                    artifact_x as i32 + 60,
                    sub_y + 7,
                    Scale::uniform(20.0),
                    font,
                    sub_type,
                );
            }
            let sub_value_width = text_size(scale, font, &sub_value).0;
            draw_text_mut(
                &mut image,
                color,
                artifact_x as i32 + 340 - sub_value_width,
                sub_y,
                scale,
                font,
                &sub_value,
            );
            overlay(&mut image, &sub_image, artifact_x + 20, sub_y.into());
            sub_y += 52;
        }
        artifact_x += 373;
    }
    let rank_img = constants::get_grade_image(artifact_scores, None)?;
    overlay(&mut image, &rank_img, 1810, 355);
    let total_score = locale::Locale::from(locale::json!({
        "en": "Total Score",
        "ja": "総合スコア",
    }))
    .get(raw_lang)
    .to_string();
    let scale = Scale::uniform(30.0);
    draw_text_mut(&mut image, white, 1440, 350, scale, font, &total_score);
    let text = round_to_1_decimal_places(artifact_scores).to_string();
    let scale = Scale::uniform(90.0);
    let (text_w, text_h) = text_size(scale, font, &text);
    draw_text_mut(
        &mut image,
        white,
        1630 - text_w / 2,
        450 - text_h / 2,
        scale,
        font,
        &text,
    );

    let default_status = data.fight_prop();
    let mut status_y = 65;
    let statuslist = vec![
        (
            (default_status.display_max_hp.round() as u32).to_string(),
            Stats::Hp.name(api, lang)?,
        ),
        (
            (default_status.display_attack.round() as u32).to_string(),
            Stats::Attack.name(api, lang)?,
        ),
        (
            (default_status.display_defense.round() as u32).to_string(),
            Stats::Defense.name(api, lang)?,
        ),
        (
            (default_status.elemental_mastery.round() as i64).to_string(),
            Stats::ElementMastery.name(api, lang)?,
        ),
        (
            format!(
                "{}%",
                round_to_1_decimal_places(default_status.critical_rate * 100.0)
            ),
            Stats::Critical.name(api, lang)?,
        ),
        (
            format!(
                "{}%",
                round_to_1_decimal_places(default_status.critical_damage * 100.0)
            ),
            Stats::CriticalHurt.name(api, lang)?,
        ),
        (
            format!(
                "{}%",
                round_to_1_decimal_places(default_status.energy_recharge * 100.0)
            ),
            Stats::ChargeEfficiency.name(api, lang)?,
        ),
        (
            format!(
                "{}%",
                round_to_1_decimal_places(
                    default_status
                        .damage_bonus
                        .get(&data.element)
                        .unwrap_or(&0.0)
                        .to_owned()
                        * 100.0
                )
            ),
            Stats::ElementAddHurt(data.element).name(api, lang)?,
        ),
    ];
    let scale = Scale::uniform(35.0);
    for (status, code) in statuslist {
        let status_width = text_size(scale, font, &status).0;
        draw_text_mut(
            &mut image,
            white,
            1350 - status_width,
            status_y,
            scale,
            font,
            &status,
        );
        draw_text_mut(&mut image, white, 845, status_y, scale, font, code);
        status_y += 70;
    }
    status_y -= 70;
    let img = data.element.image(icons, 2.5)?;
    overlay(&mut image, &img, 790, status_y.into());

    let sets = data
        .reliquarys()
        .iter()
        .map(|x| x.set_name(api, lang))
        .collect::<Vec<Option<&str>>>();
    let mut set = HashMap::<String, u32>::new();
    for s in sets.into_iter().flatten() {
        if set.contains_key(s) {
            let count = set.get(s).unwrap();
            set.insert(s.to_string(), count + 1);
        } else {
            set.insert(s.to_string(), 1);
        }
    }
    let mut largest_set_key = 0;
    let mut largest_set: Option<(&String, &u32)> = None;
    let mut second_largest_set: Option<(&String, &u32)> = None;
    for (key, value) in set.iter() {
        if value >= &largest_set_key {
            second_largest_set = largest_set;
            largest_set = Some((key, value));
            largest_set_key = *value;
        }
    }
    let (set_name, set_count) = if let Some(largest_set) = largest_set {
        if largest_set.1 > &1 {
            (largest_set.0.to_string(), *largest_set.1)
        } else {
            ("None".to_string(), 0)
        }
    } else {
        ("None".to_string(), 0)
    };
    let (second_set_name, second_set_count) = if let Some(second_largest_set) = second_largest_set {
        if second_largest_set.1 > &1 {
            (
                Some(second_largest_set.0.to_string()),
                *second_largest_set.1,
            )
        } else {
            (None, 0)
        }
    } else {
        (None, 0)
    };
    let first_key_color = if largest_set_key > 3 {
        Rgba([0, 255, 255, 255])
    } else {
        white
    };
    if let Some(second_set_name) = second_set_name {
        draw_text_mut(
            &mut image,
            white,
            1820,
            240,
            scale,
            font,
            &format!("{}", set_count),
        );
        draw_text_mut(
            &mut image,
            white,
            1820,
            285,
            scale,
            font,
            &format!("{}", second_set_count),
        );
        draw_text_resized(&mut image, white, 1520, 240, scale, font, &set_name, 250);
        draw_text_resized(
            &mut image,
            white,
            1520,
            285,
            scale,
            font,
            &second_set_name,
            250,
        );
    } else {
        draw_text_resized(
            &mut image,
            first_key_color,
            1520,
            260,
            scale,
            font,
            &set_name,
            250,
        );
        draw_text_mut(
            &mut image,
            white,
            1820,
            260,
            scale,
            font,
            &format!("{}", set_count),
        );
    }

    let kind = counter.to_string_locale(lang);
    let scale = Scale::uniform(35.0);
    let (kind_w, _) = text_size(scale, font, &kind);
    draw_text_mut(&mut image, white, 1870 - kind_w, 580, scale, font, &kind);
    convert(image, format)
}

pub fn convert(image: DynamicImage, format: ImageFormat) -> Option<Vec<u8>> {
    let format = Into::<Option<ImageOutputFormat>>::into(format);
    if let Some(format) = format {
        let mut buf = BufWriter::new(Cursor::new(Vec::new()));
        image.write_to(&mut buf, format).ok()?;
        Some(buf.into_inner().ok()?.into_inner())
    } else {
        Some(image.into_bytes())
    }
}

pub fn is_percent(stat: &Stats) -> bool {
    matches!(
        stat,
        Stats::Critical
            | Stats::AttackPercent
            | Stats::ChargeEfficiency
            | Stats::CriticalHurt
            | Stats::DefensePercent
            | Stats::ElementAddHurt(Element::Electric)
            | Stats::ElementAddHurt(Element::Fire)
            | Stats::ElementAddHurt(Element::Ice)
            | Stats::ElementAddHurt(Element::Grass)
            | Stats::ElementAddHurt(Element::Water)
            | Stats::ElementAddHurt(Element::Wind)
            | Stats::ElementAddHurt(Element::Rock)
            | Stats::Heal
            | Stats::HpPercent
    )
}

fn round_to_1_decimal_places(x: f64) -> String {
    let s = ((x * 10.0).round() / 10.0).to_string();
    if !s.contains('.') {
        return format!("{}.0", s);
    }
    s
}

pub fn mini_score(data: [Option<StatsValue>; 4], counter: &ScoreCounter) -> (f64, Vec<String>) {
    let mut score = 0.0;
    let mut used: Vec<Stats> = Vec::new();
    for sub in data {
        if sub.is_none() {
            continue;
        }
        let sub = sub.unwrap();
        let stat = sub.0;
        let value = sub.1;
        match stat {
            Stats::Critical => {
                score += value * 2.0;
                used.push(Stats::Critical);
            }
            Stats::CriticalHurt => {
                score += value;
                used.push(Stats::CriticalHurt);
            }
            Stats::AttackPercent => {
                if counter == &ScoreCounter::Normal {
                    score += value;
                    used.push(Stats::AttackPercent);
                }
            }
            Stats::DefensePercent => {
                if counter == &ScoreCounter::Def {
                    score += value;
                    used.push(Stats::DefensePercent);
                }
            }
            Stats::HpPercent => {
                if counter == &ScoreCounter::Hp {
                    score += value;
                    used.push(Stats::HpPercent);
                }
            }
            Stats::ChargeEfficiency => {
                if counter == &ScoreCounter::ChargeEfficiency {
                    score += value;
                    used.push(Stats::ChargeEfficiency);
                }
            }
            Stats::ElementMastery => {
                if counter == &ScoreCounter::ElementalMastery {
                    score += value / 4.0;
                    used.push(Stats::ElementMastery);
                }
            }
            _ => {}
        }
    }
    (score, used.iter().map(|x| x.id().to_string()).collect())
}

pub fn get_score(data: &Reliquary, counter: &ScoreCounter) -> (f64, Vec<String>) {
    mini_score(data.sub_stats, counter)
}

fn draw_text_resized(
    canvas: &mut DynamicImage,
    color: image::Rgba<u8>,
    x: i32,
    y: i32,
    scale: Scale,
    font: &Font,
    text: &str,
    max_width: u32,
) {
    let width = font
        .layout(text, scale, rusttype::Point { x: 0.0, y: 0.0 })
        .filter_map(|g| g.pixel_bounding_box())
        .fold(0, |acc, g| acc + g.width());
    if width > max_width as i32 {
        let scale = Scale::uniform(scale.x * (max_width as f32 / width as f32));
        draw_text_mut(canvas, color, x, y, scale, font, text);
        return;
    }
    draw_text_mut(canvas, color, x, y, scale, font, text)
}

fn get_artifacts(artifacts: &Vec<Reliquary>) -> Vec<Option<&Reliquary>> {
    let mut result = vec![None; 5];
    for artifact in artifacts {
        let position = artifact.position;
        result[match position {
            ReliquaryType::Flower => 0,
            ReliquaryType::Feather => 1,
            ReliquaryType::Sands => 2,
            ReliquaryType::Goblet => 3,
            ReliquaryType::Circlet => 4,
        }] = Some(artifact);
    }
    result
}
