use hsr_gen::{
    base::{get_base_image, BaseImage},
    format::ImageFormat,
    gen::{generate, ScoreCounter},
};
use mihoyo_api::api::Api;
use tokio::fs::write;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api = Api::new();
    let lang = "ja".to_string();
    let (user, _d) = api.simple(803336796, lang.to_string()).await.unwrap();
    let character = user
        .characters
        .iter()
        .find(|x| x.id == *"1213")
        .unwrap();
    let base_image = get_base_image(BaseImage::Belobog);
    if let Some(img) = generate(
        &api,
        character,
        base_image,
        ImageFormat::Png,
        &lang,
        &ScoreCounter::Attack,
    )
    .await
    {
        write("test.png", img).await.unwrap();
    };
}
