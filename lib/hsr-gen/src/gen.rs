use std::{io::Cursor, str::FromStr};

use gen_utils::{get_hsr_grade_image, get_rarity_image};
use image::{
    imageops::{crop_imm, overlay, resize, FilterType},
    DynamicImage, ImageOutputFormat,
};
use imageproc::drawing::draw_text_mut;
use localization::t;
use mihoyo_api::{
    api::Api,
    character::{Attribute, Character, Relic},
};
use rusttype::{Font, Scale};
use std::io::BufWriter;

use crate::format::ImageFormat;

const FONT: &[u8; 4917640] = include_bytes!("../../../assets/hsr-font.ttf");

pub async fn generate(
    api: &Api,
    character: &Character,
    mut base_image: DynamicImage,
    format: ImageFormat,
    lang: &str,
    counter: &ScoreCounter,
) -> Option<Vec<u8>> {
    let lang = match lang {
        "en" | "en-US" | "en-GB" => "en-US",
        _ => "ja-JP",
    };
    let font = Font::try_from_bytes(FONT)?;
    // character
    let img = api.asset(&character.portrait).await.ok()?;
    let img = crop_imm(
        &resize(&img, 1000, 1000, FilterType::Triangle),
        240,
        165,
        570,
        670,
    )
    .to_image();
    overlay(&mut base_image, &img, 60, 45);
    // character text
    let text = character.name.clone();
    draw_text_mut(
        &mut base_image,
        image::Rgba([255, 255, 255, 255]),
        50,
        50,
        Scale::uniform(50.0),
        &font,
        &text,
    );
    /*let promo = format!("R{}", character.promotion);
    draw_text_mut(
        &mut base_image,
        image::Rgba([255, 255, 255, 255]),
        220,
        80,
        Scale::uniform(20.0),
        &font,
        &promo,
    );*/
    let level = format!("Lv.{}/ S{}", character.level, character.promotion);
    draw_text_mut(
        &mut base_image,
        image::Rgba([255, 255, 255, 255]),
        50,
        110,
        Scale::uniform(25.0),
        &font,
        &level,
    );
    // character rarity
    let img = get_rarity_image(character.rarity)?;
    overlay(&mut base_image, &img, 50, 150);
    // weapon
    let img = api.asset(&character.light_cone.portrait).await.ok()?;
    let img = resize(&img, 115, 155, FilterType::Triangle);
    overlay(&mut base_image, &img, 72, 800);
    let img = get_rarity_image(character.light_cone.rarity)?;
    let img = resize(&img, 120, 30, FilterType::Triangle);
    overlay(&mut base_image, &img, 70, 940);
    // weapon text
    let text = character.light_cone.name.clone();
    draw_text_mut(
        &mut base_image,
        image::Rgba([255, 255, 255, 255]),
        325,
        765,
        Scale::uniform(50.0),
        &font,
        &text,
    );
    let promo = character.light_cone.promotion;
    let level = character.light_cone.level;
    let text = format!("Lv.{}/ S{}", level, promo);
    draw_text_mut(
        &mut base_image,
        image::Rgba([255, 255, 255, 255]),
        325,
        820,
        Scale::uniform(20.0),
        &font,
        &text,
    );
    for (index, attr) in character.light_cone.attributes.iter().enumerate() {
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            325,
            870 + 50 * index as i32,
            Scale::uniform(25.0),
            &font,
            &attr.name,
        );
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            475,
            870 + 50 * index as i32,
            Scale::uniform(25.0),
            &font,
            &attr.display,
        );
    }
    // relics
    let mut total_score = 0.0;
    for (index, relic) in character.relics.iter().enumerate() {
        let img = api.asset(&relic.icon).await.ok()?;
        let img = resize(&img, 120, 120, FilterType::Triangle);
        overlay(&mut base_image, &img, 1300, 45 + 173 * index as i64);
        let img = get_rarity_image(relic.rarity)?;
        let img = resize(&img, 120, 30, FilterType::Triangle);
        overlay(&mut base_image, &img, 1300, 135 + 173 * index as i64);
        // main stats
        let text = relic.main_affix.name.clone();
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            1425,
            60 + 173 * index as i32,
            Scale::uniform(25.0),
            &font,
            &text,
        );
        let img = api.asset(&relic.main_affix.icon).await.ok()?;
        let img = resize(&img, 30, 30, FilterType::Triangle);
        overlay(&mut base_image, &img, 1415, 90 + 173 * index as i64);
        let text = relic.main_affix.display.clone();
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            1445,
            90 + 173 * index as i32,
            Scale::uniform(25.0),
            &font,
            &text,
        );
        // relic text
        let text = relic.name.clone();
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            1550,
            50 + 173 * index as i32,
            Scale::uniform(30.0),
            &font,
            &text,
        );
        let level = format!("+{}", relic.level);
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            1425,
            135 + 173 * index as i32,
            Scale::uniform(25.0),
            &font,
            &level,
        );
        for (rindex, affix) in relic.sub_affix.iter().enumerate() {
            draw_text_mut(
                &mut base_image,
                image::Rgba([255, 255, 255, 255]),
                1550,
                85 + 25 * rindex as i32 + 173 * index as i32,
                Scale::uniform(20.0),
                &font,
                &affix.name,
            );
            draw_text_mut(
                &mut base_image,
                image::Rgba([255, 255, 255, 255]),
                1700,
                85 + 25 * rindex as i32 + 173 * index as i32,
                Scale::uniform(20.0),
                &font,
                &affix.display,
            );
        }
        // relic score
        let relic_score = get_score(&relic, &counter);
        total_score += relic_score;
        let score = format!("{:.1}", relic_score);
        let img = get_hsr_grade_image("B")?;
        let img = resize(&img, 70, 70, FilterType::Triangle);
        overlay(&mut base_image, &img, 1770, 75 + 173 * index as i64);
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            1780,
            140 + 173 * index as i32,
            Scale::uniform(30.0),
            &font,
            &score,
        );
    }
    // character stats
    for (index, stats) in resolve_stats(character)?.iter().enumerate() {
        let img = api.asset(&stats.icon).await.ok()?;
        let img = resize(&img, 30, 30, FilterType::Triangle);
        overlay(&mut base_image, &img, 850, 100 + 50 * index as i64);
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            900,
            100 + 50 * index as i32,
            Scale::uniform(25.0),
            &font,
            &stats.name,
        );
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            1100,
            100 + 50 * index as i32,
            Scale::uniform(25.0),
            &font,
            &stats.display,
        );
    }
    // element
    let img = api.asset(&character.element.icon).await.ok()?;
    let img = resize(&img, 60, 60, FilterType::Triangle);
    overlay(&mut base_image, &img, 700, 100);
    let img = api.asset(&character.path.icon).await.ok()?;
    let img = resize(&img, 60, 60, FilterType::Triangle);
    overlay(&mut base_image, &img, 700, 180);
    // skill
    for (index, skill) in character.skills.iter().enumerate() {
        let img = api.asset(&skill.icon).await.ok()?;
        let img = resize(&img, 60, 60, FilterType::Triangle);
        overlay(&mut base_image, &img, 765, 100 + 80 * index as i64);
        let level = format!("Lv.{}", skill.level);
        draw_text_mut(
            &mut base_image,
            image::Rgba([255, 255, 255, 255]),
            780,
            160 + 80 * index as i32,
            Scale::uniform(15.0),
            &font,
            &level,
        );
    }
    // all
    let total = t!(lang, "main:hsr.total");
    draw_text_mut(
        &mut base_image,
        image::Rgba([255, 255, 255, 255]),
        700,
        750,
        Scale::uniform(50.0),
        &font,
        &total,
    );
    let score = format!("{:.1}", total_score);
    let img = get_hsr_grade_image("B")?;
    let img = resize(&img, 120, 120, FilterType::Triangle);
    overlay(&mut base_image, &img, 780, 830);
    draw_text_mut(
        &mut base_image,
        image::Rgba([255, 255, 255, 255]),
        980,
        850,
        Scale::uniform(70.0),
        &font,
        &score,
    );
    convert(base_image, format).await
}

pub async fn convert(image: DynamicImage, format: ImageFormat) -> Option<Vec<u8>> {
    let format = match format {
        ImageFormat::Png => Some(ImageOutputFormat::Png),
        ImageFormat::Jpeg => Some(ImageOutputFormat::Jpeg(100)),
        ImageFormat::Raw => None,
    };
    let format = Into::<Option<ImageOutputFormat>>::into(format);
    if let Some(format) = format {
        let mut buf = BufWriter::new(Cursor::new(Vec::new()));
        image.write_to(&mut buf, format).ok()?;
        Some(buf.into_inner().ok()?.into_inner())
    } else {
        Some(image.into_bytes())
    }
}

fn resolve_stats(character: &Character) -> Option<Vec<Attribute>> {
    let mut attr = character.attributes.iter().clone();
    let additional = character.additions.iter().clone();
    let mut hp = attr.find(|attr| attr.field == "hp".to_string())?.clone();
    let mut atk = attr.find(|attr| attr.field == "atk".to_string())?.clone();
    let mut def = attr.find(|attr| attr.field == "def".to_string())?.clone();
    let mut speed = attr.find(|attr| attr.field == "spd".to_string())?.clone();
    let mut crit_rate = attr
        .find(|attr| attr.field == "crit_rate".to_string())?
        .clone();
    let mut crit_dmg = attr
        .find(|attr| attr.field == "crit_dmg".to_string())?
        .clone();
    for add in additional.clone() {
        match add.field.as_str() {
            "hp" => hp.value += add.value,
            "atk" => atk.value += add.value,
            "def" => def.value += add.value,
            "spd" => speed.value += add.value,
            "crit_rate" => crit_rate.value += add.value,
            "crit_dmg" => crit_dmg.value += add.value,
            _ => {}
        }
    }
    let mut stats = vec![hp, atk, def, crit_rate, crit_dmg, speed];
    for attr in additional {
        if attr.field == "effect_hit" || attr.field == "effect_res" || attr.field == "break_dmg" {
            stats.push(attr.clone());
        }
    }
    Some(
        stats
            .iter()
            .map(|a| {
                let mut a = a.clone();
                a.display = if a.percent {
                    a.value = a.value * 100.0;
                    format!("{:.2}%", a.value)
                } else {
                    format!("{}", (a.value.round() as i64))
                };
                a.clone()
            })
            .collect(),
    )
}

fn get_score(relic: &Relic, counter: &ScoreCounter) -> f64 {
    let mut score = 0.0;
    for affix in relic.sub_affix.iter() {
        match affix.field.as_str() {
            "atk" => {
                if *counter == ScoreCounter::Attack {
                    score += affix.value * 0.075;
                }
            }
            "atk_per" => {
                if *counter == ScoreCounter::Attack {
                    score += affix.value * 100.0;
                }
            }
            "crit_rate" => {
                score += affix.value * 200.0;
            }
            "crit_dmg" => {
                score += affix.value * 100.0;
            }
            _ => {}
        }
    }
    score
}

#[derive(PartialEq)]
pub enum ScoreCounter {
    Attack,
}

impl FromStr for ScoreCounter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attack" => Ok(ScoreCounter::Attack),
            _ => Err(()),
        }
    }
}

impl ToString for ScoreCounter {
    fn to_string(&self) -> String {
        match self {
            ScoreCounter::Attack => "attack".to_string(),
        }
    }
}