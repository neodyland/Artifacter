use enkanetwork_rs::{EnkaNetwork, IconData};
use gen::{generate, ImageFormat, Lang, ScoreCounter};
use show_image::{create_window, ImageInfo, ImageView};

fn main() -> anyhow::Result<()> {
    let api = EnkaNetwork::new()?;
    show_image::run_context(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(_main(api))
    });
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
    let character_id = charas.get(1);
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
        &Lang::Ja,
        &icons,
        ScoreCounter::Normal,
        ImageFormat::Pixel,
    )
    .await;
    if img.is_none() {
        return Ok(());
    }
    let img = img.unwrap();
    let window = create_window("test", Default::default())?;
    window.set_image(
        "a",
        ImageView::new(ImageInfo::rgba8(1920, 1080), img.as_slice()),
    )?;
    window.wait_until_destroyed()?;
    Ok(())
}
