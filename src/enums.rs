use serde::{Serialize};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum PaiColor {
    #[default]
    Unknown,
    Manzu,
    Pinzu,
    Souzu,
    Jihai,
}
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Pai {
    pub num: u8,
    pub color: PaiColor,
    pub idx: u8,
}


impl From<u8> for Pai {
    fn from(value: u8) -> Self {
        Self {
            num: (value / 4) % 9 + 1,
            color: match value / 4 {
                0..=8 => PaiColor::Manzu,
                9..=17 => PaiColor::Pinzu,
                18..=26 => PaiColor::Souzu,
                27..=33 => PaiColor::Jihai,
                _ => panic!("Invalid value for Pai"),
            },
            idx: value % 4,
        }
    }
}

#[derive(Debug, Default)]
pub enum MajEvent {
    #[default]
    Unknown,
    #[allow(dead_code)]
    Init {
        /// E S W N
        bakaze: &'static str,
        dora_marker: Pai,
        honba: u8,
        kyoku: u8,
        kyotaku: u8,
        oya: u8,
        scores: [i32; 4],
        tehais: [[Pai; 13]; 4],
    },
    Ryuukyoku {
        reason: String,
        ///ryuukyoku
        r#type: &'static str,
    },
    Dora {
        dora_marker: Pai,
        /// dora
        r#type: &'static str,
    },
    Reach {
        junme: u8,
        actor: u8,
        /// riichi or riichi_accepted
        r#type: &'static str,
    },
    Dahai {
        junme: u8,
        actor: u8,
        pai: Pai,
        /// dahai
        r#type: &'static str,
        tsumogiri: bool,
    },
    Tsumo {
        junme: u8,
        actor: u8,
        pai: Pai,
        /// tsumo
        r#type: &'static str,
    },
    Naki {
        junme: Option<u8>,
        actor: u8,
        consumed: Vec<Pai>,
        pai: Option<Pai>,
        target: Option<u8>,
        /// chii pon kakan ankan daiminkan
        r#type: &'static str,
    },
    Agari {
        honba: u8,
        kyotaku: u8,
        junme: u8,
        hai: Vec<Pai>,
        naki: Option<Vec<MajEvent>>,
        machi: Pai,
        han: u8,
        hu: u8,
        score: i32,
        yaku: Vec<u8>,
        dora_marker: Vec<Pai>,
        ura_marker: Option<Vec<Pai>>,
        actor: u8,
        fromwho: u8,
        paowho: Option<u8>,
        /// agari
        r#type: &'static str,
    },
}

