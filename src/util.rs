use enkanetwork_rs::{Character, EnkaNetwork};
use poise::serenity_prelude::{
    CreateActionRow, CreateButton, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
};

pub fn create_components(
    characters: Vec<&Character>,
    api: &EnkaNetwork,
    lang: &str,
    uid: &i32,
) -> Vec<CreateActionRow> {
    let mut options = Vec::<CreateSelectMenuOption>::new();
    for character in characters {
        let name = &character.name(api, lang);
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
        .placeholder("キャラクターを選択してください")
        .max_values(1)
        .min_values(1);
    let chara = CreateActionRow::SelectMenu(chara);
    let score = CreateSelectMenu::new(
        "score",
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("通常", "normal"),
                CreateSelectMenuOption::new("HP型", "hp"),
                CreateSelectMenuOption::new("防御型", "def"),
                CreateSelectMenuOption::new("熟知型", "mastery"),
                CreateSelectMenuOption::new("チャージ型", "charge"),
            ],
        },
    )
    .max_values(1)
    .min_values(1)
    .placeholder("計算方法を選択してください");
    let score = CreateActionRow::SelectMenu(score);
    let button = CreateActionRow::Buttons(vec![
        CreateButton::new_link(format!("https://enka.network/u/{}", uid)).label("Enka Network"),
        /*CreateButton::new("end")
        .style(ButtonStyle::Danger)
        .label("終了"),*/
    ]);
    vec![chara, score, button]
}
