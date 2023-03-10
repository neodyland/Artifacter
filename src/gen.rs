use std::collections::HashMap;

use enkanetwork_rs::{Character, Element, EnkaNetwork, IconData, Reliquary, ReliquaryType, Stats};
use image::{
    imageops::{self, resize, FilterType},
    io::Reader,
    DynamicImage, Rgba,
};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreCounter {
    Normal,
    Hp,
    Def,
    ElementalMastery,
    ChargeEfficiency,
}

impl ScoreCounter {
    fn en(&self) -> &str {
        match self {
            ScoreCounter::Normal => "Normal",
            ScoreCounter::Hp => "Hp",
            ScoreCounter::Def => "Def",
            ScoreCounter::ElementalMastery => "ElementalMastery",
            ScoreCounter::ChargeEfficiency => "ChargeEfficiency",
        }
    }
    fn ja(&self) -> &str {
        match self {
            ScoreCounter::Normal => "通常型",
            ScoreCounter::Hp => "HP型",
            ScoreCounter::Def => "防御型",
            ScoreCounter::ElementalMastery => "熟知型",
            ScoreCounter::ChargeEfficiency => "チャージ型",
        }
    }
    pub fn to_string(&self, lang: &str) -> String {
        match lang {
            "ja" => self.ja().to_string(),
            _ => self.en().to_string(),
        }
    }
}

pub async fn generate(
    data: Character,
    api: &EnkaNetwork,
    lang: &str,
    icons: &IconData,
    counter: ScoreCounter,
) -> Option<DynamicImage> {
    let font = include_bytes!("../assets/font.ttf");
    let font = Font::try_from_bytes(font)?;
    let mut image = Reader::open(format!(
        "assets/base/{}.png",
        data.element.to_string().to_lowercase()
    ))
    .ok()?
    .decode()
    .ok()?;
    let character_image = data.image_gacha_splash(api).await?;
    let character_image = character_image.resize_exact(1200, 600, imageops::FilterType::Nearest);
    imageops::overlay(&mut image, &character_image, -225, 50);

    let character_name = data.name(api, lang).ok()?;
    let character_level = format!("Lv.{},{}", data.level, data.friendship().to_string());
    let white = image::Rgba([255, 255, 255, 255]);
    draw_text_mut(
        &mut image,
        white.clone(),
        30,
        20,
        Scale::uniform(60.0),
        &font,
        &character_name,
    );
    draw_text_mut(
        &mut image,
        white.clone(),
        35,
        80,
        Scale::uniform(32.0),
        &font,
        &character_level,
    );

    let scale = Scale::uniform(25.0);
    let skills = data.skills();
    let mut skill_y = 320;
    let mut first = true;
    for skill in skills {
        let img = skill.image(api).await.ok()?;
        let img = img.resize(80, 80, image::imageops::Triangle);
        image::imageops::overlay(&mut image, &img, 20, skill_y as i64);
        let ex = skill.extra_level();
        let lv = skill.level();
        let text = format!("Lv.{}", lv + ex);
        let (tw, th) = text_size(scale, &font, &text);
        let color = if lv + ex == 13 || (lv == 10 && first) {
            image::Rgba([255, 0, 0, 255])
        } else {
            white.clone()
        };
        draw_text_mut(
            &mut image,
            color,
            60 - tw / 2,
            skill_y + th + 50,
            scale,
            &font,
            &text,
        );
        skill_y += 100;
        first = false;
    }

    let clocks = data.consts();
    let mut clock_y = 100;
    for clock in clocks {
        let img = clock.image(api).await.ok()?;
        let mut img = img.into_rgba8();
        let mut base_img = DynamicImage::new_rgba8(60, 60).into_rgba8();
        if let Some(c) = icons.image("Const.svg", 5.0) {
            image::imageops::overlay(&mut base_img, &c, 0, 0);
        }
        if !clock.is_unlock() {
            for p in img.pixels_mut() {
                p.0 = [100, 100, 100, p.0[3]];
            }
        }
        let img = resize(&img, 40, 40, image::imageops::Triangle);
        image::imageops::overlay(&mut base_img, &img, 15, 15);
        image::imageops::overlay(&mut image, &base_img, 680, clock_y as i64);
        clock_y += 80;
    }

    let weapon = data.weapon();
    let weapon_img = weapon.image_icon(api).await.ok()?;
    let weapon_img = weapon_img.resize_exact(129, 128, image::imageops::Triangle);
    image::imageops::overlay(&mut image, &weapon_img, 1430, 50);
    let weapon_rarity_img =
        Reader::open(format!("assets/rarity/{}.png", weapon.rarity.to_string())).ok()?;
    let weapon_rarity_img = weapon_rarity_img.decode().ok()?;
    let weapon_rarity_img = weapon_rarity_img.resize_exact(
        (weapon_rarity_img.width() as f32 * 0.9).round() as u32,
        (weapon_rarity_img.height() as f32 * 0.9).round() as u32,
        image::imageops::Triangle,
    );
    image::imageops::overlay(&mut image, &weapon_rarity_img, 1422, 173);
    let ascension = format!("R{}", weapon.refinement);
    draw_text_mut(
        &mut image,
        white.clone(),
        1435,
        45,
        scale,
        &font,
        &ascension,
    );
    let weapon_level = format!("Lv.{}", weapon.level);
    let weapon_name = weapon.name(api, lang)?;
    let scale = Scale::uniform(30.0);
    draw_text_mut(
        &mut image,
        white.clone(),
        1600,
        45,
        scale,
        &font,
        &weapon_name,
    );
    draw_text_mut(
        &mut image,
        white.clone(),
        1600,
        85,
        scale,
        &font,
        &weapon_level,
    );
    let scale = Scale::uniform(25.0);
    let weapon_damage = format!("ATK:{}", weapon.base_attack);
    let mut damage_image = icons.image("FIGHT_PROP_ATTACK.svg", 1.8)?;
    for p in damage_image.pixels_mut() {
        p.0 = [255, 255, 255, p.0[3]];
    }
    draw_text_mut(
        &mut image,
        white.clone(),
        1630,
        125,
        scale,
        &font,
        &weapon_damage,
    );
    image::imageops::overlay(&mut image, &damage_image, 1600, 125);
    if weapon.stats.is_some() {
        let stats = weapon.stats.unwrap();
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
        draw_text_mut(
            &mut image,
            white.clone(),
            1630,
            160,
            scale,
            &font,
            &weapon_sub,
        );
        image::imageops::overlay(&mut image, &weapon_sub_image, 1600, 160);
    }

    let artifacts = data.reliquarys();
    let mut artifact_x = 30;
    let mut artifact_scores = 0.0;
    for artifact in artifacts {
        let (score, used) = get_score(artifact, &counter);
        artifact_scores += score;
        let rank_img = get_rank_img(score, Some(artifact.position))?;
        image::imageops::overlay(&mut image, &rank_img, artifact_x + 50, 1015);
        let score = round_to_1_decimal_places(score).to_string();
        let scale = Scale::uniform(40.0);
        let score_width = text_size(scale.clone(), &font, &score).0;
        draw_text_mut(
            &mut image,
            white.clone(),
            artifact_x as i32 - score_width + 350,
            1015,
            scale,
            &font,
            &score,
        );
        let img = artifact.image_icon(api).await.ok()?;
        let mut img = img
            .resize_exact(256, 256, image::imageops::Triangle)
            .into_rgba8();
        img.pixels_mut().for_each(|p| {
            let p3 = p.0[3].clone();
            if p3 > 100 {
                p.0[3] -= 100;
            } else if p3 > 20 {
                p.0[3] -= 20;
            }
        });
        image::imageops::overlay(&mut image, &img, artifact_x, 630);
        let main = artifact.main_stats;
        let main_type = main.0.name(api, lang)?;
        let main_value = if is_percent(&main.0) {
            format!("{}%", main.1)
        } else {
            main.1.to_string()
        };
        let mut main_image = main.0.image(icons, 1.8)?;
        for p in main_image.pixels_mut() {
            p.0 = [255, 255, 255, p.0[3]];
        }
        let scale = Scale::uniform(30.0);
        let main_type_width = text_size(scale, &font, &main_type).0;
        draw_text_mut(
            &mut image,
            white.clone(),
            artifact_x as i32 + 340 - main_type_width,
            660,
            scale,
            &font,
            &main_type,
        );
        let scale = Scale::uniform(60.0);
        let main_value_width = text_size(scale, &font, &main_value).0;
        draw_text_mut(
            &mut image,
            white.clone(),
            artifact_x as i32 + 340 - main_value_width,
            690,
            scale,
            &font,
            &main_value,
        );
        image::imageops::overlay(
            &mut image,
            &main_image,
            artifact_x + 310 - main_type_width as i64,
            660,
        );
        let level = format!("+{}", artifact.level);
        let scale = Scale::uniform(25.0);
        let level_width = text_size(scale, &font, &level).0;
        draw_text_mut(
            &mut image,
            white.clone(),
            artifact_x as i32 + 340 - level_width,
            750,
            scale,
            &font,
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
                white.clone()
            } else {
                gray.clone()
            };
            let sub_type = sub.0.name(api, lang)?;
            let sub_value = if is_percent(&sub.0) {
                format!("{}%", sub.1)
            } else {
                sub.1.to_string()
            };
            let mut sub_image = sub.0.image(icons, 1.8)?;
            for p in sub_image.pixels_mut() {
                p.0 = [255, 255, 255, p.0[3]];
            }
            let scale = Scale::uniform(30.0);
            draw_text_mut(
                &mut image,
                color.clone(),
                artifact_x as i32 + 60,
                sub_y,
                scale,
                &font,
                &sub_type,
            );
            let sub_value_width = text_size(scale, &font, &sub_value).0;
            draw_text_mut(
                &mut image,
                color.clone(),
                artifact_x as i32 + 340 - sub_value_width,
                sub_y,
                scale,
                &font,
                &sub_value,
            );
            image::imageops::overlay(&mut image, &sub_image, artifact_x + 20, sub_y.into());
            sub_y += 52;
        }
        artifact_x += 373;
    }
    let rank_img = get_rank_img(artifact_scores, None)?;
    image::imageops::overlay(&mut image, &rank_img, 1810, 355);
    let text = round_to_1_decimal_places(artifact_scores).to_string();
    let scale = Scale::uniform(90.0);
    let (text_w, text_h) = text_size(scale.clone(), &font, &text);
    draw_text_mut(
        &mut image,
        white.clone(),
        1630 - text_w / 2,
        450 - text_h / 2,
        scale.clone(),
        &font,
        &text,
    );

    let default_status = data.fight_prop();
    let mut status_y = 65;
    let statuslist = vec![
        (default_status.display_max_hp.round() as u32).to_string(),
        (default_status.display_attack.round() as u32).to_string(),
        (default_status.display_defense.round() as u32).to_string(),
        format!(
            "{}%",
            round_to_1_decimal_places(default_status.elemental_mastery)
        ),
        format!(
            "{}%",
            round_to_1_decimal_places(default_status.energy_recharge)
        ),
        format!(
            "{}%",
            round_to_1_decimal_places(default_status.critical_rate)
        ),
        format!(
            "{}%",
            round_to_1_decimal_places(default_status.critical_damage)
        ),
        format!(
            "{}%",
            round_to_1_decimal_places(
                default_status
                    .damage_bonus
                    .get(&data.element)
                    .unwrap_or(&0.0)
                    .to_owned()
            )
        ),
    ];
    let scale = Scale::uniform(35.0);
    for status in statuslist {
        let status_width = text_size(scale, &font, &status).0;
        draw_text_mut(
            &mut image,
            white.clone(),
            1350 - status_width,
            status_y,
            scale,
            &font,
            &status,
        );
        status_y += 70;
    }
    status_y = status_y - 70;
    let img = data.element.image(icons, 2.5)?;
    image::imageops::overlay(&mut image, &img, 790, status_y.into());
    let element_name = Stats::ElementAddHurt(data.element).name(api, lang)?;
    draw_text_mut(
        &mut image,
        white.clone(),
        845,
        status_y,
        scale,
        &font,
        &element_name,
    );

    let sets = data.reliquarys().into_iter().map(|x| x.set_name(api, lang)).collect::<Vec<Option<&str>>>();
    let mut set = HashMap::<String, u32>::new();
    for s in sets {
        if s.is_none() {
            continue;
        }
        let s = s.unwrap();
        if set.contains_key(s) {
            let count = set.get(s).unwrap();
            set.insert(s.to_string(), count + 1);
        } else {
            set.insert(s.to_string(), 1);
        }
    }
    let largest_set = set.iter().max_by_key(|x| x.1);
    let set_name = if largest_set.is_none() {
        "None".to_string()
    } else {
        largest_set.unwrap().0.to_string()
    };
    let set_count = if largest_set.is_none() {
        0
    } else {
        *largest_set.unwrap().1
    };
    draw_text_mut(
        &mut image,
        white.clone(),
        1590,
        260,
        scale,
        &font,
        &set_name,
    );
    draw_text_mut(
        &mut image,
        white.clone(),
        1820,
        260,
        scale,
        &font,
        &format!("{}", set_count),
    );

    let kind = counter.to_string(&lang);
    let scale = Scale::uniform(35.0);
    let (kind_w, _) = text_size(scale.clone(), &font, &kind);
    draw_text_mut(
        &mut image,
        white.clone(),
        1870 - kind_w,
        580,
        scale.clone(),
        &font,
        &kind,
    );

    Some(image)
}

pub fn is_percent(stat: &Stats) -> bool {
    match stat {
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
        | Stats::HpPercent => true,
        _ => false,
    }
}

fn round_to_1_decimal_places(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

fn get_score(data: &Reliquary, counter: &ScoreCounter) -> (f64, Vec<String>) {
    let mut score = 0.0;
    let mut used: Vec<Stats> = Vec::new();
    for sub in data.sub_stats {
        if sub.is_none() {
            continue;
        }
        let sub = sub.unwrap();
        let stat = sub.0;
        let value = sub.1;
        match stat {
            Stats::Critical => {
                score += value;
                used.push(Stats::Critical);
            }
            Stats::CriticalHurt => {
                score += value * 2.0;
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

fn get_rank_img(score: f64, part: Option<ReliquaryType>) -> Option<DynamicImage> {
    let scores = get_scores_for_part(part);
    let score = if score >= scores.ss {
        "SS".to_string()
    } else if score >= scores.s {
        "S".to_string()
    } else if score >= scores.a {
        "A".to_string()
    } else {
        "B".to_string()
    };
    let mut image = image::open(format!("assets/grades/{}.png", score)).ok()?;
    image = image.resize(45, 45, FilterType::Nearest);
    Some(image)
}
