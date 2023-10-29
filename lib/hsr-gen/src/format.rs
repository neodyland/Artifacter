use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Raw,
}

impl FromStr for ImageFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(ImageFormat::Png),
            "jpeg" => Ok(ImageFormat::Jpeg),
            "raw" => Ok(ImageFormat::Raw),
            _ => Err(()),
        }
    }
}
