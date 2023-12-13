use rand::Rng;

pub fn filter_tips(mut tips: Vec<String>, uid_tip: String, uid: bool) -> Option<String> {
    if uid {
        tips.push(uid_tip);
    }
    let rng = rand::thread_rng().gen_range(0..tips.len() as u32 * 2);
    if rng < tips.len() as u32 {
        Some(tips[rng as usize].clone())
    } else {
        None
    }
}
