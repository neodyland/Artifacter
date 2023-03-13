use enkanetwork_rs::{Character, EnkaNetwork};
use gen::Lang;
use poise::serenity_prelude::{
    CreateActionRow, CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};
use serde_json::Value;

pub use serde_json::json;

pub fn create_components(
    characters: Vec<&Character>,
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
                .description(name),
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

pub struct Locale {
    pub ja: String,
    pub en: String,
}

impl Locale {
    pub fn get(&self, lang: &Lang) -> &str {
        match lang {
            Lang::Ja => &self.ja,
            Lang::En => &self.en,
        }
    }
}

impl From<Value> for Locale {
    fn from(value: Value) -> Self {
        let ja = value["ja"].as_str().unwrap_or("空白").to_owned();
        let en = value["en"].as_str().unwrap_or("Empty").to_owned();
        Self { ja, en }
    }
}
