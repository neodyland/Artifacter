#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api = enka_api::api::Api::new();
    let icons = api.icon_data();
    let (user, _cached) = api.simple(827106332).await.unwrap();
    let character = user
        .character(enka_api::character::CharacterId(10000031))
        .unwrap();
    let format = gen::gen::ImageFormat::Png;
    let counter = gen::gen::ScoreCounter::Hp;
    let raw_lang = gen::gen::Lang::Ja;
    let res = gen::gen::generate(character.clone(), &api, &raw_lang, &icons, counter, format)
        .await
        .unwrap();
    tokio::fs::write("test.png", res).await.unwrap();
    /*let api = enka_api::api::Api::new();
    let (user, _cached) = api.simple(882746077).await.unwrap();
    let character = user.character(enka_api::character::CharacterId(10000048)).unwrap();
    let req = character
        .reliquarys()
        .iter()
        .find(|x| x.position == enka_api::character::ReliquaryType::Sands)
        .unwrap();
    let op = gen::dupe::resolve_op(req);
    println!("{:?}", op);
    println!("{:?}", req);*/
}
