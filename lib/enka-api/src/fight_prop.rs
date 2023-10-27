use std::collections::HashMap;

use serde_json::Value;

use crate::{element::Element, store::Store};

#[derive(Clone, Debug)]
pub struct FightProp {
    pub base_hp: f64,
    pub hp: f64,
    pub display_current_hp: f64,
    pub display_max_hp: f64,
    pub display_attack: f64,
    pub display_defense: f64,
    pub display_speed: f64,
    pub hp_percent: f64,
    pub base_attack: f64,
    pub attack: f64,
    pub attack_percent: f64,
    pub base_defense: f64,
    pub defense: f64,
    pub defense_percent: f64,
    pub base_speed: f64,
    pub speed_percent: f64,
    pub critical_rate: f64,
    pub critical_damage: f64,
    pub energy_recharge: f64,
    pub healing_bonus: f64,
    pub incoming_healing_bonus: f64,
    pub elemental_mastery: f64,
    pub cooldown_reduction: f64,
    pub shield_strength: f64,
    pub elemental_reaction_critical: ElementalReactionCritical,
    pub resist: HashMap<Element, f64>,
    pub damage_bonus: HashMap<Element, f64>,
    pub enegry_cost: HashMap<Element, f64>,
    pub current_energy: HashMap<Element, f64>,
}
pub struct FightPropLocale<'a> {
    pub base_hp: &'a str,
    pub max_hp: &'a str,
    pub current_hp: &'a str,
    pub attack: &'a str,
    pub defense: &'a str,
    pub base_attack: &'a str,
    pub attack_percent: &'a str,
    pub defense_percent: &'a str,
    pub base_defense: &'a str,
    pub base_speed: &'a str,
    pub speed_percent: &'a str,
    pub critical_rate: &'a str,
    pub critical_damage: &'a str,
    pub energy_recharge: &'a str,
    pub elemental_mastery: &'a str,
    pub cooldown_reduction: &'a str,
    pub shield_strength: &'a str,
}
#[derive(Clone, Debug)]
pub struct ElementalReactionCritical {
    pub rate: f64,
    pub damage: f64,
    pub base_rate: f64,
    pub base_damage: f64,
    pub overloaded_rate: f64,
    pub overloaded_damage: f64,
    pub swirl_rate: f64,
    pub swirl_damage: f64,
    pub electro_charged_rate: f64,
    pub electro_charged_damage: f64,
    pub superconduct_rate: f64,
    pub superconduct_damage: f64,
    pub burn_rate: f64,
    pub burn_charged_damage: f64,
    pub frozen_shattered_rate: f64,
    pub frozen_shattered_damage: f64,
    pub bloom_rate: f64,
    pub bloom_damage: f64,
    pub burgeon_rate: f64,
    pub burgeon_damage: f64,
    pub hyperbloom_rate: f64,
    pub hyperbloom_damage: f64,
}
fn value_or(json: &Value, key: impl AsRef<str>, def: impl Into<f64>) -> f64 {
    match json.get(key.as_ref()) {
        Some(value) => match value.as_f64() {
            Some(f) => f,
            None => def.into(),
        },
        None => def.into(),
    }
}
fn parse_erc(json: &Value) -> ElementalReactionCritical {
    ElementalReactionCritical {
        rate: value_or(json, "3025", 0),
        damage: value_or(json, "3026", 0),
        overloaded_rate: value_or(json, "3027", 0),
        overloaded_damage: value_or(json, "3028", 0),
        swirl_rate: value_or(json, "3029", 0),
        swirl_damage: value_or(json, "3030", 0),
        electro_charged_rate: value_or(json, "3031", 0),
        electro_charged_damage: value_or(json, "3032", 0),
        superconduct_rate: value_or(json, "3033", 0),
        superconduct_damage: value_or(json, "3034", 0),
        burn_rate: value_or(json, "3035", 0),
        burn_charged_damage: value_or(json, "3036", 0),
        frozen_shattered_rate: value_or(json, "3037", 0),
        frozen_shattered_damage: value_or(json, "3038", 0),
        bloom_rate: value_or(json, "3039", 0),
        bloom_damage: value_or(json, "3040", 0),
        burgeon_rate: value_or(json, "3041", 0),
        burgeon_damage: value_or(json, "3042", 0),
        hyperbloom_rate: value_or(json, "3043", 0),
        hyperbloom_damage: value_or(json, "3044", 0),
        base_rate: value_or(json, "3045", 0),
        base_damage: value_or(json, "3046", 0),
    }
}
impl FightProp {
    fn id_to_element(id: i32) -> Element {
        match id % 10 {
            0 => Element::Fire,
            1 => Element::Electric,
            2 => Element::Water,
            3 => Element::Grass,
            4 => Element::Wind,
            5 => {
                if id > 70 {
                    Element::Ice
                } else {
                    Element::Rock
                }
            }
            6 => {
                if id > 70 {
                    Element::Rock
                } else {
                    Element::Ice
                }
            }
            _ => Element::None,
        }
    }
    pub fn from_json(json: &Value) -> FightProp {
        let mut fp = Self {
            base_hp: value_or(json, "1", 0),
            hp: value_or(json, "2", 0),
            hp_percent: value_or(json, "3", 0),
            base_attack: value_or(json, "4", 0),
            attack: value_or(json, "5", 0),
            attack_percent: value_or(json, "6", 0),
            base_defense: value_or(json, "7", 0),
            defense: value_or(json, "8", 0),
            defense_percent: value_or(json, "9", 0),
            base_speed: value_or(json, "10", 0),
            speed_percent: value_or(json, "11", 0),
            critical_rate: value_or(json, "20", 0),
            critical_damage: value_or(json, "22", 0),
            energy_recharge: value_or(json, "23", 1),
            healing_bonus: value_or(json, "26", 0),
            incoming_healing_bonus: value_or(json, "27", 0),
            elemental_mastery: value_or(json, "28", 0),
            cooldown_reduction: value_or(json, "80", 0),
            shield_strength: value_or(json, "81", 0),
            display_current_hp: value_or(json, "1010", 0),
            display_max_hp: value_or(json, "2000", 0),
            display_attack: value_or(json, "2001", 0),
            display_defense: value_or(json, "2002", 0),
            display_speed: value_or(json, "2003", 0),
            elemental_reaction_critical: parse_erc(json),
            resist: HashMap::new(),
            damage_bonus: HashMap::new(),
            enegry_cost: HashMap::new(),
            current_energy: HashMap::new(),
        };
        let phys_res = value_or(json, "29", 0);
        if phys_res != 0f64 {
            fp.resist.insert(Element::None, phys_res);
        }
        let phys_bonus = value_or(json, "30", 0);
        if phys_bonus != 0f64 {
            fp.damage_bonus.insert(Element::None, phys_bonus);
        }
        for i in 40..47 {
            let v = value_or(json, format!("{}", i), 0);
            if v != 0f64 {
                let e = Self::id_to_element(i);
                fp.damage_bonus.insert(e, v);
            }
        }
        for i in 50..57 {
            let v = value_or(json, format!("{}", i), 0);
            if v != 0f64 {
                let e = Self::id_to_element(i);
                fp.resist.insert(e, v);
            }
        }
        for i in 70..77 {
            let v = value_or(json, format!("{}", i), 0);
            if v != 0f64 {
                let e = Self::id_to_element(i);
                fp.enegry_cost.insert(e, v);
            }
        }
        for i in 1000..1007 {
            let v = value_or(json, format!("{}", i), 0);
            if v != 0f64 {
                let e = Self::id_to_element(i);
                fp.current_energy.insert(e, v);
            }
        }
        fp
    }
}

impl<'a> FightPropLocale<'a> {
    pub fn parse(loc: &'a Store, language: impl AsRef<str>) -> Self {
        Self {
            base_hp: loc.get_or_empty(&language, "FIGHT_PROP_BASE_HP"),
            current_hp: loc.get_or_empty(&language, "FIGHT_PROP_CUR_HP"),
            max_hp: loc.get_or_empty(&language, "FIGHT_PROP_MAX_HP"),
            base_attack: loc.get_or_empty(&language, "FIGHT_PROP_BASE_ATTACK"),
            attack: loc.get_or_empty(&language, "FIGHT_PROP_ATTACK"),
            attack_percent: loc.get_or_empty(&language, "FIGHT_PROP_ATTACK_PERCENT"),
            base_defense: loc.get_or_empty(&language, "FIGHT_PROP_BASE_DEFENSE"),
            defense: loc.get_or_empty(&language, "FIGHT_PROP_DEFENSE"),
            defense_percent: loc.get_or_empty(&language, "FIGHT_PROP_DEFENSE_PERCENT"),
            base_speed: loc.get_or_empty(&language, "FIGHT_PROP_BASE_SPEED"),
            speed_percent: loc.get_or_empty(&language, "FIGHT_PROP_SPEED_PERCENT"),
            critical_rate: loc.get_or_empty(&language, "FIGHT_PROP_CRITICAL"),
            critical_damage: loc.get_or_empty(&language, "FIGHT_PROP_CRITICAL_HURT"),
            energy_recharge: loc.get_or_empty(&language, "FIGHT_PROP_CHARGE_EFFICIENCY"),
            elemental_mastery: loc.get_or_empty(&language, "FIGHT_PROP_ELEMENT_MASTERY"),
            cooldown_reduction: loc.get_or_empty(&language, "FIGHT_PROP_SKILL_CD_MINUS_RATIO"),
            shield_strength: loc.get_or_empty(&language, "FIGHT_PROP_SHIELD_COST_MINUS_RATIO"),
        }
    }
}
