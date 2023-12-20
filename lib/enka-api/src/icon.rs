use image::{ImageBuffer, RgbaImage};
use usvg::FitTo;

use crate::{api::Api, character::Stats, element::Element};
use resvg::tiny_skia::{PixmapMut, Transform};
use std::collections::HashMap;

const CDREDUCTION_PRIMARY: &[u8; 459] =
    include_bytes!("../../../assets/icon/CdReduction_primary.svg");
const CONST: &[u8; 9504] = include_bytes!("../../../assets/icon/Const.svg");
const FIGHT_PROP_ATTACK: &[u8; 595] = include_bytes!("../../../assets/icon/FIGHT_PROP_ATTACK.svg");
const FIGHT_PROP_ATTACK_PERCENT: &[u8; 2550] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_ATTACK_PERCENT.svg");
const FIGHT_PROP_BASE_ATTACK: &[u8; 595] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_BASE_ATTACK.svg");
const FIGHT_PROP_CHARGE_EFFICIENCY: &[u8; 1036] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_CHARGE_EFFICIENCY.svg");
const FIGHT_PROP_CRITICAL: &[u8; 1471] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_CRITICAL.svg");
const FIGHT_PROP_CRITICAL_HURT: &[u8; 2212] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_CRITICAL_HURT.svg");
const FIGHT_PROP_DEFENSE: &[u8; 501] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_DEFENSE.svg");
const FIGHT_PROP_DEFENSE_PERCENT: &[u8; 2407] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_DEFENSE_PERCENT.svg");
const FIGHT_PROP_ELEC_ADD_HURT: &[u8; 1350] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_ELEC_ADD_HURT.svg");
const FIGHT_PROP_ELEMENT_MASTERY: &[u8; 848] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_ELEMENT_MASTERY.svg");
const FIGHT_PROP_FIRE_ADD_HURT: &[u8; 1105] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_FIRE_ADD_HURT.svg");
const FIGHT_PROP_GRASS_ADD_HURT: &[u8; 2575] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_GRASS_ADD_HURT.svg");
const FIGHT_PROP_HEALED_ADD: &[u8; 1271] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_HEALED_ADD.svg");
const FIGHT_PROP_HEAL_ADD: &[u8; 478] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_HEAL_ADD.svg");
const FIGHT_PROP_HP: &[u8; 827] = include_bytes!("../../../assets/icon/FIGHT_PROP_HP.svg");
const FIGHT_PROP_HP_PERCENT: &[u8; 2651] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_HP_PERCENT.svg");
const FIGHT_PROP_ICE_ADD_HURT: &[u8; 5436] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_ICE_ADD_HURT.svg");
const FIGHT_PROP_PHYSICAL_ADD_HURT: &[u8; 737] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_PHYSICAL_ADD_HURT.svg");
const FIGHT_PROP_ROCK_ADD_HURT: &[u8; 1573] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_ROCK_ADD_HURT.svg");
const FIGHT_PROP_SHIELD_COST_MINUS_RATIO: &[u8; 509] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_SHIELD_COST_MINUS_RATIO.svg");
const FIGHT_PROP_WATER_ADD_HURT: &[u8; 1126] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_WATER_ADD_HURT.svg");
const FIGHT_PROP_WIND_ADD_HURT: &[u8; 1381] =
    include_bytes!("../../../assets/icon/FIGHT_PROP_WIND_ADD_HURT.svg");
const FRIENDSHIP: &[u8; 2872] = include_bytes!("../../../assets/icon/Friendship.svg");

pub struct IconData(HashMap<String, Vec<u8>>);
impl Api {
    pub fn icon_data(&self) -> IconData {
        IconData::load(self)
    }
}
impl IconData {
    pub fn load(_: &Api) -> Self {
        let mut datas = HashMap::new();

        datas.insert(
            "CdReduction_primary.svg".to_string(),
            CDREDUCTION_PRIMARY.to_vec(),
        );
        datas.insert("Const.svg".to_string(), CONST.to_vec());
        datas.insert(
            "FIGHT_PROP_ATTACK.svg".to_string(),
            FIGHT_PROP_ATTACK.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_ATTACK_PERCENT.svg".to_string(),
            FIGHT_PROP_ATTACK_PERCENT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_BASE_ATTACK.svg".to_string(),
            FIGHT_PROP_BASE_ATTACK.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_CHARGE_EFFICIENCY.svg".to_string(),
            FIGHT_PROP_CHARGE_EFFICIENCY.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_CRITICAL.svg".to_string(),
            FIGHT_PROP_CRITICAL.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_CRITICAL_HURT.svg".to_string(),
            FIGHT_PROP_CRITICAL_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_DEFENSE.svg".to_string(),
            FIGHT_PROP_DEFENSE.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_DEFENSE_PERCENT.svg".to_string(),
            FIGHT_PROP_DEFENSE_PERCENT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_ELEC_ADD_HURT.svg".to_string(),
            FIGHT_PROP_ELEC_ADD_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_ELEMENT_MASTERY.svg".to_string(),
            FIGHT_PROP_ELEMENT_MASTERY.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_FIRE_ADD_HURT.svg".to_string(),
            FIGHT_PROP_FIRE_ADD_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_GRASS_ADD_HURT.svg".to_string(),
            FIGHT_PROP_GRASS_ADD_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_HEALED_ADD.svg".to_string(),
            FIGHT_PROP_HEALED_ADD.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_HEAL_ADD.svg".to_string(),
            FIGHT_PROP_HEAL_ADD.to_vec(),
        );
        datas.insert("FIGHT_PROP_HP.svg".to_string(), FIGHT_PROP_HP.to_vec());
        datas.insert(
            "FIGHT_PROP_HP_PERCENT.svg".to_string(),
            FIGHT_PROP_HP_PERCENT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_ICE_ADD_HURT.svg".to_string(),
            FIGHT_PROP_ICE_ADD_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_PHYSICAL_ADD_HURT.svg".to_string(),
            FIGHT_PROP_PHYSICAL_ADD_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_ROCK_ADD_HURT.svg".to_string(),
            FIGHT_PROP_ROCK_ADD_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_SHIELD_COST_MINUS_RATIO.svg".to_string(),
            FIGHT_PROP_SHIELD_COST_MINUS_RATIO.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_WATER_ADD_HURT.svg".to_string(),
            FIGHT_PROP_WATER_ADD_HURT.to_vec(),
        );
        datas.insert(
            "FIGHT_PROP_WIND_ADD_HURT.svg".to_string(),
            FIGHT_PROP_WIND_ADD_HURT.to_vec(),
        );
        datas.insert("Friendship.svg".to_string(), FRIENDSHIP.to_vec());
        Self(datas)
    }
    pub fn svg(&self, path: impl AsRef<str>) -> Option<&Vec<u8>> {
        self.0.get(path.as_ref())
    }
    pub fn image(&self, path: impl AsRef<str>, zoom: f32) -> Option<RgbaImage> {
        let bytes = self.svg(path)?;
        let ops = usvg::Options::default();
        let tree = usvg::Tree::from_data(bytes, &ops).ok()?;
        let size = tree.size;
        let fit = FitTo::Zoom(zoom);
        let tf = Transform::identity();
        let width = size.width() * zoom as f64;
        let height = size.height() * zoom as f64;
        let mut rgba8 = vec![0; width as usize * height as usize * 4];
        let pxmap = PixmapMut::from_bytes(&mut rgba8, width as u32, height as u32)?;
        resvg::render(&tree, fit, tf, pxmap);
        let img: RgbaImage = ImageBuffer::from_raw(width as u32, height as u32, rgba8)?;
        Some(img)
    }
    pub fn image_color(
        &self,
        path: impl AsRef<str>,
        zoom: f32,
        color: image::Rgba<u8>,
    ) -> Option<RgbaImage> {
        let mut img = self.image(path, zoom)?;
        for px in img.pixels_mut() {
            px.0 = [color.0[0], color.0[1], color.0[2], px.0[3]];
        }
        Some(img)
    }
}
impl Element {
    pub fn image(&self, data: &IconData, zoom: f32) -> Option<RgbaImage> {
        data.image(
            format!("FIGHT_PROP_{}_ADD_HURT.svg", self.fight_prop_name()),
            zoom,
        )
    }
    pub fn image_color(
        &self,
        data: &IconData,
        zoom: f32,
        color: image::Rgba<u8>,
    ) -> Option<RgbaImage> {
        data.image_color(
            format!("FIGHT_PROP_{}_ADD_HURT.svg", self.fight_prop_name()),
            zoom,
            color,
        )
    }
}
impl Stats {
    pub fn image(&self, data: &IconData, zoom: f32) -> Option<RgbaImage> {
        data.image(format!("{}.svg", self.id()), zoom)
    }
}
