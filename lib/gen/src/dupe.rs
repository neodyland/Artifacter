use std::collections::HashMap;

use serde::Deserialize;

use crate::constants::{DUPE, SUBOP};
use enka_api::character::{Reliquary, Stats};
use once_cell::sync::Lazy;

static DUPE_LIST: Lazy<Dupe> = Lazy::new(|| serde_json::from_str(DUPE).unwrap());
static SUBOP_LIST: Lazy<Subop> = Lazy::new(|| serde_json::from_str(SUBOP).unwrap());

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
    pub fn get(&self, s: &Stats) -> Option<HashMap<String, Vec<Vec<f64>>>> {
        Some(match s {
            Stats::Critical => self.crit_per.clone(),
            Stats::CriticalHurt => self.crit_dmg.clone(),
            Stats::ChargeEfficiency => self.charge_per.clone(),
            Stats::DefensePercent => self.def_per.clone(),
            Stats::AttackPercent => self.atk_per.clone(),
            Stats::HpPercent => self.hp_per.clone(),
            Stats::Hp => self.hp.clone(),
            Stats::Attack => self.atk.clone(),
            Stats::Defense => self.def.clone(),
            Stats::ElementMastery => self.mastery.clone(),
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
    pub fn get(&self, s: &Stats) -> Option<HashMap<String, Vec<f64>>> {
        Some(match s {
            Stats::Critical => self.crit_per.clone(),
            Stats::CriticalHurt => self.crit_dmg.clone(),
            Stats::ChargeEfficiency => self.charge_per.clone(),
            Stats::DefensePercent => self.def_per.clone(),
            Stats::AttackPercent => self.atk_per.clone(),
            Stats::HpPercent => self.hp_per.clone(),
            Stats::Hp => self.hp.clone(),
            Stats::Attack => self.atk.clone(),
            Stats::Defense => self.def.clone(),
            Stats::ElementMastery => self.mastery.clone(),
            _ => return None,
        })
    }
}

pub fn resolve_op(art: &Reliquary) -> Option<Vec<Vec<f64>>> {
    let sub = art.sub_stats.iter().collect::<Vec<_>>();
    let mut max_count = (art.rarity + art.level / 4 - 1) as usize;
    let mut dupes = vec![vec![vec![]]; 4];
    let mut subops = vec![vec![0.0; 6]; 4];
    for (index, sub) in sub.iter().enumerate() {
        if let Some(sub) = sub {
            let trim = trim(sub.1);
            let not_trim = sub.1.to_string();
            let dupe_list = DUPE_LIST.get(&sub.0)?;
            let dupe = dupe_list.get(&trim).or(dupe_list.get(&not_trim));
            let subop_list = SUBOP_LIST.get(&sub.0)?;
            let mut subop = subop_list.get(&trim).or(subop_list.get(&not_trim))?;
            if let Some(dupe) = dupe {
                dupes[index] = dupe.clone();
                let dupe = dupe.iter().min_by_key(|d| d.len())?;
                subop = dupe;
            }
            subops[index] = subop.clone();
            max_count -= subop.len();
        }
    }
    for (index, dupe) in dupes.iter_mut().enumerate() {
        if dupe.first().map(|f| f.is_empty()).unwrap_or(false) {
            continue;
        }
        dupe.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for dupe in dupe {
            if max_count == 0 {
                break;
            }
            subops[index] = dupe.clone();
            max_count -= 1;
        }
    }
    Some(subops)
}

fn trim(s: f64) -> String {
    if s.fract() != 0.0 {
        return s.to_string();
    }
    format!("{}.0", s)
}
