use enkanetwork_rs::{Character, EnkaNetwork, IconData};
use image::{
    imageops::{self, resize},
    io::Reader,
    DynamicImage,
};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

pub async fn generate(
    data: Character,
    api: &EnkaNetwork,
    lang: &str,
    icons: &IconData,
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
        let weapon_sub = format!("{} {}%", stats.0.name(api, lang)?, stats.1);
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
    for artifact in artifacts {
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
        artifact_x += 370;
    }

    Some(image)
}
