use std::str::FromStr;

use image::{imageops::overlay, load_from_memory, DynamicImage};
use once_cell::sync::Lazy;
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

const HSR_EFFECT: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/hsr_effect/orange.png")).unwrap()
});
const BELOBOG: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/belobog.png")).unwrap());
const EVERWINTER: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/hsr_base/everwinter.png")).unwrap()
});
const FU_XUAN: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/fu_xuan.png")).unwrap());
const JAR: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/jar.png")).unwrap());
const SEAL: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/seal.png")).unwrap());
const SLIDE: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/slide.png")).unwrap());
const SVAROG: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/svarog.png")).unwrap());
const TRAIN: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/train.png")).unwrap());
const TREE: Lazy<DynamicImage> =
    Lazy::new(|| load_from_memory(include_bytes!("../../../assets/hsr_base/tree.png")).unwrap());
const UNDERGROUND: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/hsr_base/underground.png")).unwrap()
});
const UNIVERSE: Lazy<DynamicImage> = Lazy::new(|| {
    load_from_memory(include_bytes!("../../../assets/hsr_base/universe.png")).unwrap()
});

pub fn get_base_image(img: BaseImage) -> DynamicImage {
    let effect = HSR_EFFECT.clone();
    let on = match img {
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
    }
    .clone();
    let mut on = on.blur(5.0);
    overlay(&mut on, &effect, 0, 0);
    on
}
