use crate::gen::ScoreCounter;

pub fn get_default(cid: &u32) -> ScoreCounter {
    match cid {
        // Ayaka
        10000002 => ScoreCounter::Normal,
        // Qin
        10000003 => ScoreCounter::Normal,
        // Aether
        10000005 => ScoreCounter::Normal,
        // Lisa
        10000006 => ScoreCounter::Normal,
        // Lumine
        10000007 => ScoreCounter::Normal,
        // Barbara
        10000014 => ScoreCounter::Hp,
        // Kaeya
        10000015 => ScoreCounter::Normal,
        // Diluc
        10000016 => ScoreCounter::Normal,
        // Razor
        10000020 => ScoreCounter::Normal,
        // Ambor
        10000021 => ScoreCounter::Normal,
        // Venti
        10000022 => ScoreCounter::ElementalMastery,
        // Xiangling
        10000023 => ScoreCounter::ChargeEfficiency,
        // Beidou
        10000024 => ScoreCounter::ChargeEfficiency,
        // Xingqiu
        10000025 => ScoreCounter::ChargeEfficiency,
        // Xiao
        10000026 => ScoreCounter::Normal,
        // Ningguang
        10000027 => ScoreCounter::Normal,
        // Klee
        10000029 => ScoreCounter::Normal,
        // Zhongli
        10000030 => ScoreCounter::Hp,
        // Fischl
        10000031 => ScoreCounter::Normal,
        // Bennett
        10000032 => ScoreCounter::Hp,
        // Tartaglia
        10000033 => ScoreCounter::Normal,
        // Noelle
        10000034 => ScoreCounter::Def,
        // Qiqi
        10000035 => ScoreCounter::Normal,
        // Chongyun
        10000036 => ScoreCounter::Normal,
        // Ganyu
        10000037 => ScoreCounter::Normal,
        // Albedo
        10000038 => ScoreCounter::Def,
        // Diona
        10000039 => ScoreCounter::Hp,
        // Mona
        10000041 => ScoreCounter::ChargeEfficiency,
        // Keqing
        10000042 => ScoreCounter::Normal,
        // Sucrose
        10000043 => ScoreCounter::ElementalMastery,
        // Xinyan
        10000044 => ScoreCounter::Normal,
        // Rosaria
        10000045 => ScoreCounter::Normal,
        // Hutao
        10000046 => ScoreCounter::Hp,
        // Kazuha
        10000047 => ScoreCounter::ElementalMastery,
        // Yanfei
        10000048 => ScoreCounter::Normal,
        // Yoimiya
        10000049 => ScoreCounter::Normal,
        // Thoma
        10000050 => ScoreCounter::ElementalMastery,
        // Eula
        10000051 => ScoreCounter::Normal,
        // Shougun
        10000052 => ScoreCounter::ChargeEfficiency,
        // Sayu
        10000053 => ScoreCounter::ElementalMastery,
        // Kokomi
        10000054 => ScoreCounter::Hp,
        // Gorou
        10000055 => ScoreCounter::Normal,
        // Sara
        10000056 => ScoreCounter::Normal,
        // Itto
        10000057 => ScoreCounter::Def,
        // Yae
        10000058 => ScoreCounter::Normal,
        // Heizou
        10000059 => ScoreCounter::Normal,
        // Yelan
        10000060 => ScoreCounter::Hp,
        // Aloy
        10000062 => ScoreCounter::Normal,
        // Shenhe
        10000063 => ScoreCounter::Normal,
        // Yunjin
        10000064 => ScoreCounter::Normal,
        // Shinobu
        10000065 => ScoreCounter::ElementalMastery,
        // Ayato
        10000066 => ScoreCounter::Normal,
        // Collei
        10000067 => ScoreCounter::ElementalMastery,
        // Dori
        10000068 => ScoreCounter::Normal,
        // Tighnari
        10000069 => ScoreCounter::ElementalMastery,
        // Nilou
        10000070 => ScoreCounter::Hp,
        // Cyno
        10000071 => ScoreCounter::Normal,
        // Candace
        10000072 => ScoreCounter::Normal,
        // Nahida
        10000073 => ScoreCounter::ElementalMastery,
        // Layla
        10000074 => ScoreCounter::Hp,
        // Wanderer
        10000075 => ScoreCounter::Normal,
        // Faruzan
        10000076 => ScoreCounter::Normal,
        // Yaoyao
        10000077 => ScoreCounter::Normal,
        // Alhatham
        10000078 => ScoreCounter::ElementalMastery,
        // Dehya
        10000079 => ScoreCounter::Normal,
        // Mika
        10000080 => ScoreCounter::Normal,
        // Kaveh
        10000081 => ScoreCounter::ElementalMastery,
        // Baizhu
        10000082 => ScoreCounter::Hp,
        // Lynette
        10000083 => ScoreCounter::ElementalMastery,
        // Lyney
        10000084 => ScoreCounter::Normal,
        // Freminet
        10000085 => ScoreCounter::ElementalMastery,
        // Wriothesley
        10000086 => ScoreCounter::Hp
        // Neuvilette
        10000087 => ScoreCounter::Hp
        _ => ScoreCounter::Normal,
    }
}
