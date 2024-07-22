use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::enums::{MajEvent, Pai, PaiColor};

impl Serialize for Pai {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num = if self.num == 5 && self.idx == 0 && self.color != PaiColor::Jihai { 0 } else { self.num };
        let color = match self.color {
            PaiColor::Manzu => 'm',
            PaiColor::Pinzu => 'p',
            PaiColor::Souzu => 's',
            PaiColor::Jihai => 'z',
            _ => panic!("Invalid color for Pai"),
        };
        serializer.serialize_str(&format!("{}{}", num, color))
    }
}


impl Serialize for MajEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MajEvent::Unknown => {
                serializer.serialize_str("Unknown")
            }
            MajEvent::Init { bakaze, dora_marker, honba, kyoku, kyotaku, oya, scores, tehais } => {
                let mut state = serializer.serialize_struct("Init", 7)?;
                state.serialize_field("bakaze", bakaze)?;
                state.serialize_field("dora_marker", dora_marker)?;
                state.serialize_field("honba", honba)?;
                state.serialize_field("kyoku", kyoku)?;
                state.serialize_field("kyotaku", kyotaku)?;
                state.serialize_field("oya", oya)?;
                state.serialize_field("scores", scores)?;
                state.serialize_field("tehais", tehais)?;
                state.end()
            }
            MajEvent::Ryuukyoku { reason, r#type } => {
                let mut state = serializer.serialize_struct("Ryuukyoku", 2)?;
                state.serialize_field("reason", reason)?;
                state.serialize_field("type", r#type)?;
                state.end()
            }
            MajEvent::Dora { dora_marker, r#type } => {
                let mut state = serializer.serialize_struct("Dora", 2)?;
                state.serialize_field("dora_marker", dora_marker)?;
                state.serialize_field("type", r#type)?;
                state.end()
            }

            MajEvent::Reach { junme, actor, r#type } => {
                let mut state = serializer.serialize_struct("Reach", 3)?;
                state.serialize_field("junme", junme)?;
                state.serialize_field("actor", actor)?;
                state.serialize_field("type", r#type)?;
                state.end()
            }
            MajEvent::Dahai { junme, actor, pai, r#type, tsumogiri } => {
                let mut state = serializer.serialize_struct("Dahai", 4)?;
                state.serialize_field("junme", junme)?;
                state.serialize_field("actor", actor)?;
                state.serialize_field("pai", pai)?;
                state.serialize_field("type", r#type)?;
                state.serialize_field("tsumogiri", tsumogiri)?;
                state.end()
            }
            MajEvent::Tsumo { junme, actor, pai, r#type } => {
                let mut state = serializer.serialize_struct("Tsumo", 4)?;
                state.serialize_field("junme", junme)?;
                state.serialize_field("actor", actor)?;
                state.serialize_field("pai", pai)?;
                state.serialize_field("type", r#type)?;
                state.end()
            }
            MajEvent::Naki { junme, actor, consumed, pai, target, r#type } => {
                let mut state = serializer.serialize_struct("Naki", 5)?;
                if junme.is_some() {
                    state.serialize_field("junme", junme)?;
                    state.serialize_field("actor", actor)?;
                }

                state.serialize_field("consumed", consumed)?;
                state.serialize_field("pai", pai)?;
                state.serialize_field("target", target)?;
                state.serialize_field("type", r#type)?;
                state.end()
            }
            MajEvent::Agari { honba, kyotaku, junme, hai, naki, machi, han, hu, score, yaku, dora_marker, ura_marker, actor, fromwho, paowho, r#type } => {
                let mut state = serializer.serialize_struct("Agari", 14)?;
                state.serialize_field("honba", honba)?;
                state.serialize_field("kyotaku", kyotaku)?;
                state.serialize_field("junme", junme)?;
                state.serialize_field("hai", hai)?;
                state.serialize_field("naki", naki)?;
                state.serialize_field("machi", machi)?;
                state.serialize_field("han", han)?;
                state.serialize_field("hu", hu)?;
                state.serialize_field("score", score)?;
                state.serialize_field("yaku", yaku)?;
                state.serialize_field("dora_marker", dora_marker)?;
                state.serialize_field("ura_marker", ura_marker)?;
                state.serialize_field("actor", actor)?;
                state.serialize_field("fromwho", fromwho)?;
                state.serialize_field("paowho", paowho)?;
                state.serialize_field("type", r#type)?;
                state.end()
            }
        }
    }
}

