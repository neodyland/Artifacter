use enkanetwork_rs::{Character, EnkaNetwork};
use gen::{Lang, ScoreCounter};
use poise::serenity_prelude::{
    CreateActionRow, CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};
use serde_json::Value;

pub use gen::locale::Locale;
pub use serde_json::json;

pub fn create_components(
    characters: Vec<Character>,
    api: &EnkaNetwork,
    lang: &Lang,
    uid: &i32,
) -> Vec<CreateActionRow> {
    let lang_str = &lang.to_string();
    let mut options = Vec::<CreateSelectMenuOption>::new();
    for character in characters {
        let name = &character.name(api, lang_str);
        if name.is_err() {
            continue;
        }
        let name = name.as_ref().unwrap().to_owned();
        options.push(
            CreateSelectMenuOption::new(name.clone(), format!("{}", &character.id.0))
                .description(format!("{}Lv", character.level)),
        )
    }
    let chara = CreateSelectMenu::new("character", CreateSelectMenuKind::String { options })
        .placeholder(
            Locale::from(json!({"ja":"キャラクターを選択してください","en": "The character"}))
                .get(lang),
        )
        .max_values(1)
        .min_values(1);
    let chara = CreateActionRow::SelectMenu(chara);
    let score = CreateSelectMenu::new(
        "score",
        CreateSelectMenuKind::String {
            options: vec![
                (
                    Locale::from(json!({"ja":"通常","en": "Normal"})).get(lang),
                    "normal",
                ),
                (
                    Locale::from(json!({"ja":"HP型","en": "HP"})).get(lang),
                    "hp",
                ),
                (
                    Locale::from(json!({"ja":"防御型","en": "Def"})).get(lang),
                    "def",
                ),
                (
                    Locale::from(json!({"ja":"熟知型","en": "Mastery"})).get(lang),
                    "mastery",
                ),
                (
                    Locale::from(json!({"ja":"チャージ型","en": "Charge"})).get(lang),
                    "charge",
                ),
            ]
            .iter()
            .map(|x| CreateSelectMenuOption::new(x.0, x.1))
            .collect(),
        },
    )
    .max_values(1)
    .min_values(1)
    .placeholder(
        Locale::from(json!({"ja":"計算方法を選択してください","en": "The way to calculate"}))
            .get(lang),
    );
    let score = CreateActionRow::SelectMenu(score);
    let format = CreateSelectMenu::new(
        "format",
        CreateSelectMenuKind::String {
            options: vec![("PNG", "png"), ("JPEG", "jpeg")]
                .iter()
                .map(|x| CreateSelectMenuOption::new(x.0, x.1))
                .collect(),
        },
    )
    .max_values(1)
    .min_values(1)
    .placeholder(
        Locale::from(json!({"ja":"拡張子を選択してください","en": "File format"})).get(lang),
    );
    let format = CreateActionRow::SelectMenu(format);
    let button = CreateActionRow::Buttons(vec![
        CreateButton::new_link(format!("https://enka.network/u/{}", uid)).label("Enka Network"),
        /*CreateButton::new("end")
        .style(ButtonStyle::Danger)
        .label("終了"),*/
    ]);
    vec![chara, score, format, button]
}

pub fn convert_rgb(rgb: [u8; 3]) -> u32 {
    let [r, g, b] = rgb;
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

pub fn get_score_calc(s: &ScoreCounter) -> Value {
    match s {
        ScoreCounter::Normal => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + 攻撃力(%)", "en": "Critical Rate × 2 + Critical Damage + Attack(%)"})
        }
        ScoreCounter::Hp => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + HP(%)", "en": "Critical Rate × 2 + Critical Damage + HP(%)"})
        }
        ScoreCounter::Def => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + 防御(%)", "en": "Critical Rate × 2 + Critical Damage + Defense(%)"})
        }
        ScoreCounter::ElementalMastery => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + (熟知 ÷ 4)", "en": "Critical Rate × 2 + Critical Damage + (Elemental Mastery ÷ 4)"})
        }
        ScoreCounter::ChargeEfficiency => {
            json!({"ja": "会心率 × 2 + 会心ダメージ + 元素チャージ効率", "en": "Critical Rate × 2 + Critical Damage + Elemental Charge Efficiency"})
        }
    }
}
