use enka_api::{api::Api, character::CharacterId};
use gen::gen::{generate, ImageFormat, Lang, ScoreCounter};
use tokio::fs::write;

const UID: i32 = 827106332;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api = Api::new();
    let icons = api.icon_data();
    let (user, _cached) = api.simple(UID).await.unwrap();
    let character = user.character(CharacterId(10000031)).unwrap();
    let format = ImageFormat::Png;
    let counter = ScoreCounter::Hp;
    let raw_lang = Lang::Ja;
    let res = generate(character.clone(), &api, &raw_lang, &icons, counter, format)
        .await
        .unwrap();
    write("test.png", res).await.unwrap();
}
