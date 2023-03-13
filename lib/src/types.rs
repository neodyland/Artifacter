use std::{collections::HashMap, num::TryFromIntError};

use enkanetwork_rs::{Reliquary, Stats};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Dupe {
    pub crit_dmg: HashMap<String, Vec<Vec<f64>>>,
    pub crit_per: HashMap<String, Vec<Vec<f64>>>,
    pub charge_per: HashMap<String, Vec<Vec<f64>>>,
    pub def_per: HashMap<String, Vec<Vec<f64>>>,
    pub atk_per: HashMap<String, Vec<Vec<f64>>>,
    pub hp_per: HashMap<String, Vec<Vec<f64>>>,
    pub hp: HashMap<String, Vec<Vec<f64>>>,
    pub atk: HashMap<String, Vec<Vec<f64>>>,
    pub def: HashMap<String, Vec<Vec<f64>>>,
    pub mastery: HashMap<String, Vec<Vec<f64>>>,
}

impl Dupe {
    pub fn new() -> Self {
        serde_json::from_str(include_str!("../../assets/dupe.json")).unwrap()
    }
    pub fn get(&self, s: &str) -> Option<HashMap<String, Vec<Vec<f64>>>> {
        Some(match s {
            "crit_dmg" => self.crit_dmg.clone(),
            "crit_per" => self.crit_per.clone(),
            "charge_per" => self.charge_per.clone(),
            "def_per" => self.def_per.clone(),
            "atk_per" => self.atk_per.clone(),
            "hp_per" => self.hp_per.clone(),
            "hp" => self.hp.clone(),
            "atk" => self.atk.clone(),
            "def" => self.def.clone(),
            "mastery" => self.mastery.clone(),
            _ => return None,
        })
    }
}

#[derive(Deserialize)]
pub struct Subop {
    pub crit_dmg: HashMap<String, Vec<f64>>,
    pub crit_per: HashMap<String, Vec<f64>>,
    pub charge_per: HashMap<String, Vec<f64>>,
    pub def_per: HashMap<String, Vec<f64>>,
    pub atk_per: HashMap<String, Vec<f64>>,
    pub hp_per: HashMap<String, Vec<f64>>,
    pub hp: HashMap<String, Vec<f64>>,
    pub atk: HashMap<String, Vec<f64>>,
    pub def: HashMap<String, Vec<f64>>,
    pub mastery: HashMap<String, Vec<f64>>,
}

impl Subop {
    pub fn new() -> Self {
        serde_json::from_str(include_str!("../../assets/subop.json")).unwrap()
    }
    pub fn get(&self, s: &str) -> Option<HashMap<String, Vec<f64>>> {
        Some(match s {
            "crit_dmg" => self.crit_dmg.clone(),
            "crit_per" => self.crit_per.clone(),
            "charge_per" => self.charge_per.clone(),
            "def_per" => self.def_per.clone(),
            "atk_per" => self.atk_per.clone(),
            "hp_per" => self.hp_per.clone(),
            "hp" => self.hp.clone(),
            "atk" => self.atk.clone(),
            "def" => self.def.clone(),
            "mastery" => self.mastery.clone(),
            _ => return None,
        })
    }
}

fn to_string(s: &Stats) -> Option<String> {
    let s = match s {
        Stats::Critical => "crit_dmg",
        Stats::CriticalHurt => "crit_per",
        Stats::ChargeEfficiency => "charge_per",
        Stats::DefensePercent => "def_per",
        Stats::AttackPercent => "atk_per",
        Stats::HpPercent => "hp_per",
        Stats::Hp => "hp",
        Stats::Attack => "atk",
        Stats::Defense => "def",
        Stats::ElementMastery => "mastery",
        _ => return None,
    };
    Some(s.to_string())
}

pub fn resolve_op(art: &Reliquary) -> Option<Vec<Vec<f64>>> {
    let dupel = Dupe::new();
    let subop = Subop::new();
    let mut result = Vec::new();
    let mut count = u64::from(art.level / 4 + art.rarity - 1);
    let mut dupes = Vec::<Vec<Vec<f64>>>::new();
    for sub in art.sub_stats {
        if sub.is_none() {
            result.push(vec![]);
            dupes.push(vec![]);
            continue;
        }
        let sub = sub.unwrap();
        let s = to_string(&sub.0)?;
        let v = sub.1;
        let duper = dupel.get(&s)?;
        let subs = subop.get(&s)?;
        let mut dupe = duper.get(&v.to_string());
        let vstr = v.to_string();
        if dupe.is_none() {
            dupe = duper.get(&trim(&vstr));
        }
        let mut sub = subs.get(&vstr).map(|x| x.to_owned());
        if sub.is_none() {
            sub = subs.get(&trim(&vstr)).map(|x| x.to_owned());
        }
        if sub.is_none() {
            sub = Some(vec![v]);
        }
        if sub.is_none() {
            result.push(vec![]);
            dupes.push(vec![]);
            continue;
        }
        let sub = sub.unwrap();
        if dupe.is_none() {
            dupes.push(vec![]);
            result.push(sub.to_vec());
            let c = u64::try_from(sub.len()).ok()?;
            if c > count {
                count = 0;
            } else {
                count = count - c;
            }
        } else {
            let dupe = dupe.unwrap();
            dupes.push(dupe.to_vec());
            result.push(vec![]);
        }
    }
    let dupe_counter = dupes
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| u64::try_from(y.len()))
                .collect::<Vec<Result<u64, TryFromIntError>>>()
        })
        .collect::<Vec<Vec<Result<u64, TryFromIntError>>>>();
    let mut dupe_counts = vec![];
    for d in dupe_counter {
        let mut dupe_count = Vec::<u64>::new();
        for i in d {
            if i.is_err() {
                return None;
            }
            let i = i.unwrap();
            dupe_count.push(i);
        }
        dupe_counts.push(dupe_count);
    }
    let mut dupe_count_result = Vec::<u64>::new();
    for _ in 0..dupes.len() {
        dupe_count_result.push(0)
    }
    let mut empty = Vec::<usize>::new();
    let mut i = 10;
    while count > 0 && i > 0 {
        if empty.len() == dupe_counts.len() {
            break;
        }
        for dp in 0..dupe_counts.len() {
            let d = &mut dupe_counts[dp];
            if d.is_empty() {
                empty.push(dp);
                continue;
            }
            let x = d.remove(0);
            if i64::try_from(count).ok()? - i64::try_from(x).ok()? < 0 {
                continue;
            }
            count = count - x;
            dupe_count_result[dp] = dupe_count_result[dp] + 1;
        }
        i = i - 1;
    }
    for i in 0..dupes.len() {
        if dupes[i].is_empty() {
            continue;
        }
        let dupe = dupes[i].clone();
        let dupe_result = dupe_count_result[i];
        let sub = dupe.get(usize::try_from(dupe_result).ok()?);
        if sub.is_none() {
            continue;
        }
        result[i] = sub.unwrap().to_vec();
    }
    Some(result)
}

fn trim(s: &str) -> String {
    format!("{}.0", s)
}
