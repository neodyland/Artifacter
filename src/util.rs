use enkanetwork_rs::{Character, EnkaNetwork};
use poise::serenity_prelude::{ButtonStyle, CreateComponents};

pub fn create_components(
    components: &mut CreateComponents,
    characters: Vec<&Character>,
    api: &EnkaNetwork,
    lang: &str,
    uid: &i32,
) {
    components.create_action_row(|f| {
        f.create_select_menu(|f| {
            f.custom_id("character");
            f.placeholder("キャラクターを選択してください");
            f.options(|f| {
                for character in characters {
                    let name = &character.name(api, lang);
                    if name.is_err() {
                        continue;
                    }
                    let name = name.as_ref().unwrap().to_owned();
                    f.create_option(|f| {
                        f.label(name.clone());
                        f.value(&character.id.0);
                        f.description(name);
                        f
                    });
                }
                f
            });
            f
        });
        f
    });
    components.create_action_row(|f| {
        f.create_select_menu(|f| {
            f.custom_id("score");
            f.placeholder("計算方式を選択してください");
            f.options(|f| {
                f.create_option(|f| {
                    f.label("通常");
                    f.value("normal");
                    f
                });
                f.create_option(|f| {
                    f.label("HP型");
                    f.value("hp");
                    f
                });
                f.create_option(|f| {
                    f.label("防御型");
                    f.value("def");
                    f
                });
                f.create_option(|f| {
                    f.label("熟知型");
                    f.value("mastery");
                    f
                });
                f.create_option(|f| {
                    f.label("チャージ型");
                    f.value("charge");
                    f
                });
                f
            })
        });
        f
    });
    components.create_action_row(|f| {
        f.create_button(|b| {
            b.style(ButtonStyle::Link)
                .label("Enka Network")
                .url(format!("https://enka.network/u/{}", uid))
        })
        .create_button(|b| b.style(ButtonStyle::Danger).label("終了").custom_id("end"))
    });
}
