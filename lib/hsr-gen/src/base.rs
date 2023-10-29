use std::str::FromStr;

use image::{load_from_memory, DynamicImage};

pub enum BaseImage {
    Blazer,
    Blazerpom,
    Clara,
}

impl FromStr for BaseImage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blazer" => Ok(BaseImage::Blazer),
            "blazerpom" => Ok(BaseImage::Blazerpom),
            "clara" => Ok(BaseImage::Clara),
            _ => Err(()),
        }
    }
}

pub fn get_base_image(img: BaseImage) -> DynamicImage {
    load_from_memory(match img {
        BaseImage::Blazer => include_bytes!("../../../assets/hsr_base/blazer.png"),
        BaseImage::Blazerpom => include_bytes!("../../../assets/hsr_base/blazerpom.png"),
        BaseImage::Clara => include_bytes!("../../../assets/hsr_base/clara.png"),
    })
    .unwrap()
}
