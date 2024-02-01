use std::borrow::Cow;

use apitype::hsr::{User, UserCharacter};
use base64::{engine::general_purpose, Engine as _};
use localization::t;
use poise::serenity_prelude as serenity;
use serenity::{
    CreateActionRow, CreateAttachment, CreateButton, CreateEmbed, CreateEmbedFooter,
    CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, Timestamp,
};

use crate::{
    api::Api,
    cache::HsrCacheValue,
    util::{convert_rgb, get_hsr_score_calc},
};

pub async fn hsr_generate_components(
    locale: String,
    uid: String,
    user: User,
    cache: &HsrCacheValue,
    api: &Api,
) -> Option<(
    CreateEmbed<'static>,
    Vec<CreateActionRow<'static>>,
    Option<CreateAttachment<'static>>,
)> {
    if let Some(cid) = cache.character.clone() {
        if let Ok((img, score)) = api
            .hsr_generate(
                Some(locale.clone()),
                uid.clone(),
                cid.clone(),
                cache.score.clone(),
                cache.format.clone(),
                cache.base_img.clone(),
            )
            .await
        {
            let components =
                create_components(user.characters.clone(), locale.clone(), uid.clone());
            let attachment = Some(CreateAttachment::bytes(img, "image.png"));
            let name = user
                .characters
                .iter()
                .find(|x| x.id.to_string() == cid)?
                .name
                .clone();
            let embed = CreateEmbed::new()
                .title(t!(locale, "main:general.generated", name))
                .footer(CreateEmbedFooter::new(uid.to_string()))
                .color(convert_rgb([0x00, 0xff, 0x00]))
                .description(format!(
                    "{}{}",
                    get_hsr_score_calc(locale.clone(), score.as_str()),
                    if user.from_cache {
                        format!("\n{}", t!(locale, "main:general.isCached"))
                    } else {
                        "".to_string()
                    }
                ))
                .image("attachment://image.png")
                .timestamp(Timestamp::from_unix_timestamp(user.lastupdate as i64).unwrap());
            return Some((embed, components, attachment));
        }
    }
    log::info!("Failed to generate image");
    None
}

pub fn hsr_profile_components<'a>(
    locale: String,
    uid: String,
    user: User,
) -> (
    CreateEmbed<'a>,
    Vec<CreateActionRow<'a>>,
    Option<CreateAttachment<'a>>,
) {
    let footer = CreateEmbedFooter::new(uid.to_string());
    let embed = CreateEmbed::default()
        .title(format!(
            "{}({},{})",
            user.name, user.level, user.world_level,
        ))
        .footer(footer)
        .color(convert_rgb([0x00, 0xff, 0x00]))
        .description(format!(
            "{}{}",
            user.description,
            if user.from_cache {
                t!(locale, "main:general.isCached")
            } else {
                "".to_string()
            }
        ))
        .image("attachment://name_card.png")
        .timestamp(Timestamp::from_unix_timestamp(user.lastupdate as i64).unwrap())
        .fields(vec![(
            t!(locale, "main:general.achievements"),
            user.achievement.to_string(),
            true,
        )]);
    let components = create_components(user.characters, locale, uid);
    let attachment = if let Some(card) = user.avatar {
        let card = general_purpose::STANDARD_NO_PAD
            .decode(card.as_bytes())
            .unwrap();
        Some(CreateAttachment::bytes(card, "name_card.png"))
    } else {
        None
    };
    (embed, components, attachment)
}

pub fn create_components<'a>(
    characters: Vec<UserCharacter>,
    locale: String,
    uid: String,
) -> Vec<CreateActionRow<'a>> {
    let mut options = Vec::<CreateSelectMenuOption>::new();
    for character in characters {
        options.push(
            CreateSelectMenuOption::new(character.name, format!("{}", &character.id))
                .description(format!("{}Lv", character.level)),
        )
    }
    let chara = CreateSelectMenu::new(
        "hsr_character",
        CreateSelectMenuKind::String {
            options: Cow::Owned(options),
        },
    )
    .placeholder(t!(locale, "main:general.selectCharacter"))
    .max_values(1)
    .min_values(1);
    let chara = CreateActionRow::SelectMenu(chara);
    let score = CreateSelectMenu::new(
        "hsr_score",
        CreateSelectMenuKind::String {
            options: [(t!(locale, "main:calculationMethod.attack"), "normal")]
                .iter()
                .map(|x| CreateSelectMenuOption::new(x.0.clone(), x.1))
                .collect(),
        },
    )
    .max_values(1)
    .min_values(1)
    .placeholder(t!(locale, "main:general.selectCalculationMethod"));
    let score = CreateActionRow::SelectMenu(score);
    let format = CreateSelectMenu::new(
        "hsr_format",
        CreateSelectMenuKind::String {
            options: [("PNG", "png"), ("JPEG", "jpeg")]
                .iter()
                .map(|x| CreateSelectMenuOption::new(x.0, x.1))
                .collect(),
        },
    )
    .max_values(1)
    .min_values(1)
    .placeholder(t!(locale, "main:general.selectFileFormat"));
    let format = CreateActionRow::SelectMenu(format);
    let base_img = CreateSelectMenu::new(
        "hsr_base_img",
        CreateSelectMenuKind::String {
            options: [
                ("Belobog", "belobog"),
                ("Everwinter", "everwinter"),
                ("Fu Xuan", "fu_xuan"),
                ("Jar", "jar"),
                ("Seal", "seal"),
                ("Slide", "slide"),
                ("Svarog", "svarog"),
                ("Train", "train"),
                ("Tree", "tree"),
                ("Underground", "underground"),
                ("Universe", "universe"),
            ]
            .iter()
            .map(|x| CreateSelectMenuOption::new(x.0, x.1))
            .collect(),
        },
    )
    .max_values(1)
    .min_values(1)
    .placeholder(t!(locale, "main:general.selectBaseImage"));
    let base_img = CreateActionRow::SelectMenu(base_img);
    let button = CreateActionRow::Buttons(vec![
        CreateButton::new_link(format!("https://enka.network/hsr/{}", uid)).label("Enka Network"),
        CreateButton::new_link(format!(
            "https://artifacter.neody.land/ja-JP/hsr/generate?uid={}",
            uid
        ))
        .label(t!(locale, "main:general.webapp")),
        /*CreateButton::new("end")
        .style(ButtonStyle::Danger)
        .label(t!(locale, "main:general.close")),*/
    ]);
    vec![chara, score, format, base_img, button]
}
