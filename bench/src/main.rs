use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::{generate,ScoreCounter};

fn main() -> anyhow::Result<()>  {
    let api = EnkaNetwork::new()?;
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(_main(api))?;
    Ok(())
}

async fn _main(api: EnkaNetwork) -> anyhow::Result<()> {
    let icons = IconData::load(&api).await;
    let uid = 827106332;
    let user = api.simple(uid).await;
    if user.is_err() {
        return Ok(());
    }
    let user = user.unwrap();
    let charas = user.profile().show_character_list();
    let character_id = charas.first();
    if character_id.is_none() {
        return Ok(());
    }
    let character = user.character(*character_id.unwrap());
    if character.is_none() {
        return Ok(());
    }
    let character = character.unwrap();
    let img = generate(
        character.to_owned(),
        &api,
        "ja",
        &icons,
        ScoreCounter::Normal,
    )
    .await;
    if img.is_none() {
        return Ok(());
    }
    let _ = img.unwrap();
    Ok(())
}