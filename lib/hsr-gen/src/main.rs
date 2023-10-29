use hsr_gen::{
    base::{get_base_image, BaseImage},
    format::ImageFormat,
    gen::generate,
};
use mihoyo_api::api::Api;
use tokio::fs::write;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api = Api::new();
    let lang = "ja-JP".to_string();
    let (user, _d) = api.simple(804445063, lang.to_string()).await.unwrap();
    let character = user.characters.first().unwrap();
    let base_image = get_base_image(BaseImage::Blazerpom);
    if let Some(img) = generate(&api, character, base_image, ImageFormat::Png, &lang).await {
        write("test.png", img).await.unwrap();
    };
}
