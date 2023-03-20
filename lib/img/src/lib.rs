use enkanetwork_rs::{EnkaNetwork, ReliquaryType, Stats, StatsValue};
use gen::{
    convert, is_percent,
    locale::{json, Locale},
    mini_score,
    types::is_valid_subop,
    ImageFormat, Lang, ScoreCounter,
};
use image::load_from_memory;
use regex::Regex;
use tesseract::Tesseract;

const UID_MATCHHER: &str = r"uid(.|)([0-9]{9})";

mod ja {
    pub const VALUE_MATCHHER: &str =
        r"(会心ダメージ|会心率|攻撃力|防御力|元素熟知|元素チャージ効率|hp)\+([0-9]+\.?[0-9]*)(%?)";
    pub const PARTS_MATCHER: &str = r"(生の花|死の羽|時の砂|理の冠|空の杯)";
}

mod en {
    pub const VALUE_MATCHHER: &str =
        r"(critrate|critdmg|atk|def|elementalmastery|energyrecharge|hp)\+([0-9]+\.?[0-9]*)(%?)";
    pub const PARTS_MATCHER: &str =
        r"(floweroflife|plumeofdeath|gobletofeonothem|circletoflogos|sandsofeon)";
}

fn get_value_matcher(lang: &str) -> &str {
    match lang {
        "ja" => ja::VALUE_MATCHHER,
        "en" => en::VALUE_MATCHHER,
        _ => "",
    }
}

fn get_parts_matcher(lang: &str) -> &str {
    match lang {
        "ja" => ja::PARTS_MATCHER,
        "en" => en::PARTS_MATCHER,
        _ => "",
    }
}

fn try_into_part(part: &str) -> Option<ReliquaryType> {
    match part {
        "生の花" => Some(ReliquaryType::Flower),
        "死の羽" => Some(ReliquaryType::Feather),
        "時の砂" => Some(ReliquaryType::Goblet),
        "理の冠" => Some(ReliquaryType::Circlet),
        "空の杯" => Some(ReliquaryType::Sands),
        "floweroflife" => Some(ReliquaryType::Flower),
        "plumeofdeath" => Some(ReliquaryType::Feather),
        "gobletofeonothem" => Some(ReliquaryType::Goblet),
        "circletoflogos" => Some(ReliquaryType::Circlet),
        "sandsofeon" => Some(ReliquaryType::Sands),
        _ => None,
    }
}

fn try_into_reltype(stat: &str, per: bool) -> Option<Stats> {
    match stat {
        "会心ダメージ" => Some(Stats::CriticalHurt),
        "会心率" => Some(Stats::Critical),
        "攻撃力" => Some(if per {
            Stats::AttackPercent
        } else {
            Stats::Attack
        }),
        "防御力" => Some(if per {
            Stats::DefensePercent
        } else {
            Stats::Defense
        }),
        "元素熟知" => Some(Stats::ElementMastery),
        "元素チャージ効率" => Some(Stats::ChargeEfficiency),
        "hp" => Some(if per { Stats::HpPercent } else { Stats::Hp }),
        "critrate" => Some(Stats::CriticalHurt),
        "cirtdmg" => Some(Stats::Critical),
        "atk" => Some(if per {
            Stats::AttackPercent
        } else {
            Stats::Attack
        }),
        "def" => Some(if per {
            Stats::DefensePercent
        } else {
            Stats::Defense
        }),
        "elementalmastery" => Some(Stats::ElementMastery),
        "energyrecharge" => Some(Stats::ChargeEfficiency),
        _ => None,
    }
}

fn match_value(str: &str, lang: &str) -> Option<MatchResult> {
    let re_stat = Regex::new(get_value_matcher(lang)).unwrap();
    let stat = re_stat.captures(str);
    if stat.is_some() {
        let stat = stat.unwrap();
        return Some(MatchResult::new_stat(
            try_into_reltype(
                stat.get(1)?.as_str(),
                stat.get(3).map(|e| e.as_str().eq("%")).unwrap_or(false),
            )?,
            stat.get(2)?.as_str().parse::<f64>().ok()?,
        ));
    }
    let re_uid = Regex::new(UID_MATCHHER).unwrap();
    let uid = re_uid.captures(str);
    if uid.is_some() {
        return Some(MatchResult::new_uid(
            uid.unwrap().get(2)?.as_str().parse::<i32>().ok()?,
        ));
    }
    let re_part = Regex::new(get_parts_matcher(lang)).unwrap();
    let part = re_part.find(str);
    if part.is_some() {
        return Some(MatchResult::new_part(try_into_part(
            part.unwrap().as_str(),
        )?));
    }
    None
}

fn get_variable_whitelist(lang: &str) -> Option<&str> {
    match lang {
        "ja" => Some("会心率ダメ攻撃元素チャ効率HP防御熟知力カージ+.0①②③④⑤⑥⑦⑧⑨123456789%UIDの生花死羽時砂理冠空杯"),
        "en" => Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789%+."),
        _ => None,
    }
}

fn try_into_api_lang(lang: &str) -> Option<&str> {
    match lang {
        "ja" => Some("jpn_fast"),
        "en" => Some("eng_best"),
        _ => None,
    }
}

fn replace_str(str: &str) -> String {
    let s = str.to_string();
    let s = s.replace(" ", "");
    let s = s.replace("①", "1");
    let s = s.replace("②", "2");
    let s = s.replace("③", "3");
    let s = s.replace("④", "4");
    let s = s.replace("⑤", "5");
    let s = s.replace("⑥", "6");
    let s = s.replace("⑦", "7");
    let s = s.replace("⑧", "8");
    let s = s.replace("⑨", "9");
    let s = s.to_lowercase();
    s
}

#[derive(Debug, Clone)]
pub struct Rel {
    pub kind: ReliquaryType,
    pub substat: Vec<StatsValue>,
}

#[derive(Debug)]
enum MatchResult {
    SubStat(StatsValue),
    Uid(i32),
    Part(ReliquaryType),
}

impl MatchResult {
    pub fn new_stat(kind: Stats, val: f64) -> MatchResult {
        MatchResult::SubStat(StatsValue(kind, val))
    }
    pub fn new_uid(v: i32) -> MatchResult {
        MatchResult::Uid(v)
    }
    pub fn new_part(q: ReliquaryType) -> MatchResult {
        MatchResult::Part(q)
    }
}

fn read_image_raw(img: Vec<u8>, lang: &str) -> Option<(Option<Rel>, Option<i32>)> {
    let api_lang = try_into_api_lang(lang);
    if api_lang.is_none() {
        return None;
    }
    let api_lang = api_lang.unwrap();
    let mut text = Tesseract::new(Some("assets/trained"), Some(api_lang)).ok()?;
    let var = get_variable_whitelist(lang);
    if var.is_some() {
        text = text
            .set_variable("tessedit_char_whitelist", var.unwrap())
            .ok()?;
    }
    let text = text
        .set_image_from_mem(img.as_slice())
        .ok()?
        .recognize()
        .ok()?
        .get_text()
        .ok()?;
    let text = text
        .split("\n")
        .map(|e| match_value(&replace_str(e), lang))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<MatchResult>>();
    let mut uid = None;
    let mut rel = Rel {
        kind: ReliquaryType::Flower,
        substat: vec![],
    };
    for e in text {
        match e {
            MatchResult::SubStat(v) => {
                if is_valid_subop(&v) {
                    rel.substat.push(v);
                }
            }
            MatchResult::Uid(v) => {
                uid = Some(v);
            }
            MatchResult::Part(v) => {
                rel.kind = v;
            }
        }
    }
    rel.substat.truncate(4);
    if rel.substat.len() == 0 {
        if uid.is_none() {
            return None;
        }
        return Some((None, uid));
    }
    Some((Some(rel), uid))
}

pub async fn read_image(img: Vec<u8>, lang: &str) -> Option<(Option<Rel>, Option<i32>)> {
    let lang = lang.to_string();
    tokio::spawn(async move { read_image_raw(img, &lang) })
        .await
        .ok()
        .flatten()
}

fn get_rel_name(rel: ReliquaryType, lang: &str) -> String {
    let r = match &rel {
        ReliquaryType::Flower => match lang {
            "ja" => "生の花",
            "en" => "Flower",
            _ => "Flower",
        },
        ReliquaryType::Feather => match lang {
            "ja" => "死の羽",
            "en" => "Feather",
            _ => "Feather",
        },
        ReliquaryType::Sands => match lang {
            "ja" => "時の砂",
            "en" => "Sands",
            _ => "Sands",
        },
        ReliquaryType::Goblet => match lang {
            "ja" => "空の杯",
            "en" => "Goblet",
            _ => "Goblet",
        },
        ReliquaryType::Circlet => match lang {
            "ja" => "理の冠",
            "en" => "Circlet",
            _ => "Circlet",
        },
    };
    r.to_string()
}

fn make_score(s: &Vec<StatsValue>, c: ScoreCounter) -> String {
    let s = s.iter().map(|x| Some(x)).collect::<Vec<_>>();
    let mut r: [Option<StatsValue>; 4] = [None; 4];
    for i in 0..4 {
        if s.len() <= i {
            break;
        }
        if s[i].is_none() {
            continue;
        }
        r[i] = s[i].copied();
    }
    mini_score(r, &c).0.to_string()
}

pub async fn read_image_trimed(
    img: Vec<u8>,
    api: &EnkaNetwork,
    lang: &str,
) -> Option<(Option<(String, String, String)>, Option<Rel>, Option<i32>)> {
    let img = load_from_memory(&img).ok()?;
    let img = img.grayscale();
    let img = convert(img, ImageFormat::Png)?;
    let l = Lang::from(lang);
    let i = read_image(img, lang).await?;
    let mut s = String::new();
    let mut n = String::new();
    let mut v = String::new();
    if i.0.is_some() {
        let i = i.0.clone().unwrap();
        n = get_rel_name(i.kind, lang);
        for e in i.substat.iter() {
            let p = if is_percent(&e.0) { "%" } else { "" };
            s.push_str(&format!("{}: {}{}\n", e.0.name(api, lang)?, e.1, p));
        }
        let normal = Locale::from(json!({"ja":"通常","en": "Normal"}));
        let hp = Locale::from(json!({"ja":"HP型","en": "HP"}));
        let def = Locale::from(json!({"ja":"防御型","en": "Def"}));
        let mas = Locale::from(json!({"ja":"熟知型","en": "Mastery"}));
        let ch = Locale::from(json!({"ja":"チャージ型","en": "Charge"}));
        let r = [
            [
                normal.get(&l),
                &make_score(&i.substat, ScoreCounter::Normal),
            ],
            [hp.get(&l), &make_score(&i.substat, ScoreCounter::Hp)],
            [def.get(&l), &make_score(&i.substat, ScoreCounter::Def)],
            [
                mas.get(&l),
                &make_score(&i.substat, ScoreCounter::ElementalMastery),
            ],
            [
                ch.get(&l),
                &make_score(&i.substat, ScoreCounter::ChargeEfficiency),
            ],
        ];
        for [e, r] in r.iter() {
            v.push_str(&format!("{}: {}\n", e, r));
        }
    }
    if s.len() == 0 {
        return Some((None, i.0, i.1));
    }
    Some((Some((n, s, v)), i.0, i.1))
}
