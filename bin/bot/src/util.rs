use localization::t;

pub fn convert_rgb(rgb: [u8; 3]) -> u32 {
    let [r, g, b] = rgb;
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

pub enum ScoreCounter {
    Normal,
    Hp,
    Def,
    ElementalMastery,
    ChargeEfficiency,
}

pub enum HsrScoreCounter {
    Attack,
}

impl From<&str> for ScoreCounter {
    fn from(s: &str) -> Self {
        match s {
            "normal" => ScoreCounter::Normal,
            "hp" => ScoreCounter::Hp,
            "def" => ScoreCounter::Def,
            "mastery" => ScoreCounter::ElementalMastery,
            "charge" => ScoreCounter::ChargeEfficiency,
            _ => ScoreCounter::Normal,
        }
    }
}

impl From<&str> for HsrScoreCounter {
    fn from(s: &str) -> Self {
        match s {
            "attack" => HsrScoreCounter::Attack,
            _ => HsrScoreCounter::Attack,
        }
    }
}

pub fn get_score_calc(lang: String, s: impl Into<ScoreCounter>) -> String {
    match s.into() {
        ScoreCounter::Normal => {
            t!(lang, "main:calculationFormula.attack")
        }
        ScoreCounter::Hp => {
            t!(lang, "main:calculationFormula.hp")
        }
        ScoreCounter::Def => {
            t!(lang, "main:calculationFormula.defense")
        }
        ScoreCounter::ElementalMastery => {
            t!(lang, "main:calculationFormula.elementalMastery")
        }
        ScoreCounter::ChargeEfficiency => {
            t!(lang, "main:calculationFormula.energyRecharge")
        }
    }
}

pub fn get_hsr_score_calc(lang: String, s: impl Into<HsrScoreCounter>) -> String {
    match s.into() {
        HsrScoreCounter::Attack => {
            t!(lang, "main:hsrCalculationFormula.attack")
        }
    }
}
