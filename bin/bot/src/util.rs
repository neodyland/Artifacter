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
    Hp,
    Defence,
    Ehr,
    Be,
    BeOnly,
    Speed,
    HpOnly,
    EhrOnly,
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
            "hp" => HsrScoreCounter::Hp,
            "def" => HsrScoreCounter::Defence,
            "ehr" => HsrScoreCounter::Ehr,
            "be" => HsrScoreCounter::Be,
            "be_only" => HsrScoreCounter::BeOnly,
            "speed" => HsrScoreCounter::Speed,
            "hp_only" => HsrScoreCounter::HpOnly,
            "ehr_only" => HsrScoreCounter::EhrOnly,
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
        HsrScoreCounter::Hp => {
            t!(lang, "main:hsrCalculationFormula.hp")
        }
        HsrScoreCounter::Defence => {
            t!(lang, "main:hsrCalculationFormula.defense")
        }
        HsrScoreCounter::Ehr => {
            t!(lang, "main:hsrCalculationFormula.ehr")
        }
        HsrScoreCounter::Be => {
            t!(lang, "main:hsrCalculationFormula.be")
        }
        HsrScoreCounter::BeOnly => {
            t!(lang, "main:hsrCalculationFormula.beOnly")
        }
        HsrScoreCounter::Speed => {
            t!(lang, "main:hsrCalculationFormula.speed")
        }
        HsrScoreCounter::HpOnly => {
            t!(lang, "main:hsrCalculationFormula.hpOnly")
        }
        HsrScoreCounter::EhrOnly => {
            t!(lang, "main:hsrCalculationFormula.ehrOnly")
        }
    }
}
