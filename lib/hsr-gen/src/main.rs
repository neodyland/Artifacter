use hsr_gen::{
    base::{get_base_image, BaseImage},
    format::ImageFormat,
    gen::{generate, ScoreCounter},
};
use mihomo_api::api::Api;
use tokio::fs::write;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let api = Api::new();
    let lang = "ja".to_string();
    let (user, _d) = api.simple(803336796, lang.to_string()).await.unwrap();
    let cl = user.characters.len();
    'a: for (i, c) in user.characters.iter().enumerate() {
        let base_image = get_base_image(BaseImage::Belobog);
        for (i2, counter) in vec![
            ScoreCounter::Attack,
            ScoreCounter::Defense,
            ScoreCounter::Speed,
            ScoreCounter::Be,
            ScoreCounter::Ehr,
            ScoreCounter::EhrOnly,
            ScoreCounter::Hp,
            ScoreCounter::HpOnly,
        ]
        .iter()
        .enumerate()
        {
            if let Some(img) = generate(
                &api,
                &c,
                base_image.clone(),
                ImageFormat::Png,
                &lang,
                &counter,
            )
            .await
            {
                let i = i * cl + i2;
                write(format!("test{}.png", i), img).await.unwrap();
                break 'a;
            };
        }
    }
}
