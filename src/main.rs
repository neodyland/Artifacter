use enkanetwork_rs::{EnkaNetwork, IconData};

mod dupe;
mod gen;

fn main() -> anyhow::Result<()> {
    let api = EnkaNetwork::new()?;
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(_main(&api))?;
    Ok(())
}

async fn _main(api: &EnkaNetwork) -> anyhow::Result<()> {
    let icons = IconData::load(api).await;
    let uid = 827106332;
    let data = api.simple(uid).await.unwrap();
    let data_id = data.profile().show_character_list()[0].clone();
    let data = data.character(data_id).unwrap().to_owned();
    let image = gen::generate(data, &api, "ja", &icons).await.unwrap();
    image.save("./tests/image.png").unwrap();
    Ok(())
}
