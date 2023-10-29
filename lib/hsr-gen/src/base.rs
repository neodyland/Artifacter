use std::str::FromStr;

use image::{imageops::overlay, load_from_memory, DynamicImage};
pub use rand::Rng;

pub enum BaseImage {
    Belobog,
    Everwinter,
    FuXuan,
    Jar,
    Seal,
    Slide,
    Svarog,
    Train,
    Tree,
    Underground,
    Universe,
}

impl FromStr for BaseImage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "belobog" => Ok(BaseImage::Belobog),
            "everwinter" => Ok(BaseImage::Everwinter),
            "fu_xuan" => Ok(BaseImage::FuXuan),
            "jar" => Ok(BaseImage::Jar),
            "seal" => Ok(BaseImage::Seal),
            "slide" => Ok(BaseImage::Slide),
            "svarog" => Ok(BaseImage::Svarog),
            "train" => Ok(BaseImage::Train),
            "tree" => Ok(BaseImage::Tree),
            "underground" => Ok(BaseImage::Underground),
            "universe" => Ok(BaseImage::Universe),
            _ => Err(()),
        }
    }
}

pub fn random_base_image() -> BaseImage {
    match rand::thread_rng().gen_range(0..11) {
        0 => BaseImage::Belobog,
        1 => BaseImage::Everwinter,
        2 => BaseImage::FuXuan,
        3 => BaseImage::Jar,
        4 => BaseImage::Seal,
        5 => BaseImage::Slide,
        6 => BaseImage::Svarog,
        7 => BaseImage::Train,
        8 => BaseImage::Tree,
        9 => BaseImage::Underground,
        _ => BaseImage::Universe,
    }
}

const HSR_EFFECT: &[u8; 165992] = include_bytes!("../../../assets/hsr_effect/orange.png");
const BELOBOG: &[u8; 3008217] = include_bytes!("../../../assets/hsr_base/belobog.png");
const EVERWINTER: &[u8; 2395309] = include_bytes!("../../../assets/hsr_base/everwinter.png");
const FU_XUAN: &[u8; 2881624] = include_bytes!("../../../assets/hsr_base/fu_xuan.png");
const JAR: &[u8; 3611677] = include_bytes!("../../../assets/hsr_base/jar.png");
const SEAL: &[u8; 2456088] = include_bytes!("../../../assets/hsr_base/seal.png");
const SLIDE: &[u8; 2432260] = include_bytes!("../../../assets/hsr_base/slide.png");
const SVAROG: &[u8; 2225647] = include_bytes!("../../../assets/hsr_base/svarog.png");
const TRAIN: &[u8; 2609576] = include_bytes!("../../../assets/hsr_base/train.png");
const TREE: &[u8; 2791487] = include_bytes!("../../../assets/hsr_base/tree.png");
const UNDERGROUND: &[u8; 3147197] = include_bytes!("../../../assets/hsr_base/underground.png");
const UNIVERSE: &[u8; 2334599] = include_bytes!("../../../assets/hsr_base/universe.png");

pub fn get_base_image(img: BaseImage) -> DynamicImage {
    let effect = load_from_memory(HSR_EFFECT).unwrap();
    let on = load_from_memory(match img {
        BaseImage::Belobog => BELOBOG,
        BaseImage::Everwinter => EVERWINTER,
        BaseImage::FuXuan => FU_XUAN,
        BaseImage::Jar => JAR,
        BaseImage::Seal => SEAL,
        BaseImage::Slide => SLIDE,
        BaseImage::Svarog => SVAROG,
        BaseImage::Train => TRAIN,
        BaseImage::Tree => TREE,
        BaseImage::Underground => UNDERGROUND,
        BaseImage::Universe => UNIVERSE,
    })
    .unwrap();
    let mut on = on.blur(5.0);
    overlay(&mut on, &effect, 0, 0);
    on
}
