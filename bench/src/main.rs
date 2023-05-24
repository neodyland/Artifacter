use std::fs;

use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::{generate, ImageFormat, Lang, ScoreCounter};

fn main() -> anyhow::Result<()> {
    let api = EnkaNetwork::new()?;
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(_main(api))
}

async fn _main(api: EnkaNetwork) -> anyhow::Result<()> {
    let icons = IconData::load(&api).await;
    let uid = 827106332;
    let user = api.simple(uid).await;
    if user.is_err() {
        println!("Error: {}", user.err().unwrap());
        return Ok(());
    }
    let (user, _) = user.unwrap();
    let charas = user.profile().show_character_list();
    let character_id = charas.get(0);
    if character_id.is_none() {
        println!("Error: No character");
        return Ok(());
    }
    let character = user.character(*character_id.unwrap());
    if character.is_none() {
        println!("Error: No character");
        return Ok(());
    }
    let character = character.unwrap();
    let img = generate(
        character.to_owned(),
        &api,
        &Lang::Ja,
        &icons,
        ScoreCounter::Normal,
        ImageFormat::Png,
    )
    .await;
    if img.is_none() {
        return Ok(());
    }
    let img = img.unwrap();
    fs::write("test.png", img)?;
    Ok(())
}
