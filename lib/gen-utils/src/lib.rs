use image::DynamicImage;
const GRADES_B: &[u8; 45056] = include_bytes!("../../../assets/grades/B.png");
const GRADES_A: &[u8; 39600] = include_bytes!("../../../assets/grades/A.png");
const GRADES_S: &[u8; 58084] = include_bytes!("../../../assets/grades/S.png");
const GRADES_SS: &[u8; 82990] = include_bytes!("../../../assets/grades/SS.png");
const HSR_GRADES_B: &[u8; 42939] = include_bytes!("../../../assets/hsr-grades/B.png");
const HSR_GRADES_A: &[u8; 42379] = include_bytes!("../../../assets/hsr-grades/A.png");
const HSR_GRADES_S: &[u8; 53484] = include_bytes!("../../../assets/hsr-grades/S.png");
const HSR_GRADES_SS: &[u8; 55084] = include_bytes!("../../../assets/hsr-grades/SS.png");
const RARITY_1: &[u8; 2342] = include_bytes!("../../../assets/rarity/1.png");
const RARITY_2: &[u8; 2945] = include_bytes!("../../../assets/rarity/2.png");
const RARITY_3: &[u8; 3166] = include_bytes!("../../../assets/rarity/3.png");
const RARITY_4: &[u8; 3400] = include_bytes!("../../../assets/rarity/4.png");
const RARITY_5: &[u8; 3449] = include_bytes!("../../../assets/rarity/5.png");

pub fn get_grade_image(kind: &str) -> Option<DynamicImage> {
    let grade: &[u8] = match kind {
        "B" => GRADES_B,
        "A" => GRADES_A,
        "S" => GRADES_S,
        "SS" => GRADES_SS,
        _ => return None,
    };
    image::load_from_memory(grade).ok()
}

pub fn get_hsr_grade_image(kind: &str) -> Option<DynamicImage> {
    let grade: &[u8] = match kind {
        "B" => HSR_GRADES_B,
        "A" => HSR_GRADES_A,
        "S" => HSR_GRADES_S,
        "SS" => HSR_GRADES_SS,
        _ => return None,
    };
    image::load_from_memory(grade).ok()
}

pub fn get_rarity_image(rarity: u8) -> Option<DynamicImage> {
    let rarity: &[u8] = match rarity {
        1 => RARITY_1,
        2 => RARITY_2,
        3 => RARITY_3,
        4 => RARITY_4,
        5 => RARITY_5,
        _ => return None,
    };
    image::load_from_memory(rarity).ok()
}
