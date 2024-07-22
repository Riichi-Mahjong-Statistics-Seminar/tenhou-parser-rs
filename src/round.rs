use std::io::{BufRead};
use std::path::Path;
use log::warn;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::enums::{MajEvent, Pai};
use crate::utils::{GetAttribute, IntoActor, IntoNumVec, IntoPaiVec};
use serde::{Serialize};

#[derive(Debug, Default, Serialize)]
pub struct Game {
    pub r#type: u16,
    pub lobby: Option<u16>,
    pub id: [String; 4],
    pub dan: [u8; 4],
    pub rate: [f32; 4],
    pub sex: [String; 4],
    pub games: Vec<Round>,
    pub owari: [i32; 4],
}
#[derive(Debug, Default, Serialize)]
pub struct RoundData {
    pub bakaze: String,
    pub dora_marker: Pai,
    pub honba: u8,
    pub kyoku: u8,
    pub kyotaku: u8,
    pub oya: u8,
    pub scores: [i32; 4],
    pub tehais: [[Pai; 13]; 4],
}

#[derive(Debug, Default, Serialize)]
pub struct Round {
    #[serde(skip)]
    pub junme: [u8; 4],
    #[serde(skip)]
    pub last_draw: Option<Pai>,
    pub data: RoundData,
    pub game: Vec<MajEvent>,
}

impl Game {
    fn parse_naki(actor: u8, m: u32, junme: Option<u8>) -> MajEvent {
        if m & 4 != 0 {
            //chii
            let tile_detail = [(m >> 3) & 3, (m >> 5) & 3, (m >> 7) & 3];
            let block1 = m >> 10;
            let called = block1 % 3;
            let base = (block1 / 21) * 8 + (block1 / 3) * 4;
            let target = (actor + 3) % 4;
            let consumed_hai = tile_detail[called as usize] + 4 * called + base;
            let hai = Pai::from(consumed_hai as u8);
            let consumed_num = (0..3).filter(|&i| i != called).map(|i| (tile_detail[i as usize] + 4 * i + base) as u8).collect::<Vec<u8>>();
            let consumed = consumed_num.iter().map(|&x: &u8| Pai::from(x)).collect();
            MajEvent::Naki {
                junme,
                actor,
                consumed,
                pai: Some(hai),
                target: Some(target),
                r#type: "chii",
            }
        } else if m & 24 != 0 {
            //pon
            let tile4th = (m >> 5) & 3;
            let target_r = m & 3;
            let block1 = m >> 9;
            let called = block1 % 3;
            let base = 4 * (block1 / 3);
            let target = (actor + target_r as u8) % 4;
            let r#type = if m & 8 != 0 { "pon" } else { "kakan" };
            let pon_tile = (0..4).filter(|&i| i != tile4th).map(|i| (i + base) as u8).collect::<Vec<u8>>();
            let (consumed_hai, consumed_num) = if r#type == "pon" {
                let consumed_hai = pon_tile[called as usize];
                let consumed_num = (0..3).filter(|&i| i != called).map(|i| pon_tile[i as usize]).collect::<Vec<u8>>();
                (consumed_hai, consumed_num)
            } else {
                let consumed_hai = tile4th + base;
                let consumed_num = pon_tile;
                (consumed_hai as u8, consumed_num)
            };
            let consumed = consumed_num.iter().map(|&x: &u8| Pai::from(x)).collect();
            let hai = Pai::from(consumed_hai);
            MajEvent::Naki {
                junme,
                actor,
                consumed,
                pai: Some(hai),
                target: Some(target),
                r#type,
            }
        } else {
            //kan
            let target_r = m & 3;
            let target = (actor + target_r as u8) % 4;
            let block1 = m >> 8;
            let called = block1 % 4;
            let base = 4 * (block1 / 4);
            let consumed_num = (0..4).filter(|&i| i != called).map(|i| (i + base) as u8).collect::<Vec<u8>>();
            let consumed_hai = called + base;
            let hai = Pai::from(consumed_hai as u8);
            if target == actor {
                let r#type = "ankan";
                let consumed = (0..4).map(|i| i + base).map(|x| Pai::from(x as u8)).collect();
                MajEvent::Naki {
                    junme,
                    actor,
                    consumed,
                    pai: None,
                    target: None,
                    r#type,
                }
            } else {
                let r#type = "daiminkan";
                let consumed = consumed_num.iter().map(|&x: &u8| Pai::from(x)).collect();
                MajEvent::Naki {
                    junme,
                    actor,
                    consumed,
                    pai: Some(hai),
                    target: Some(target),
                    r#type,
                }
            }
        }
    }

    fn update_owari(&mut self, e: &BytesStart) {
        if e.get_attribute("owari").is_some() {
            self.owari = e.get_attribute("owari").unwrap().split(',').step_by(2).map(|s| s.parse().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
        }
    }

    fn update(&mut self, e: &BytesStart) {
        match String::from_utf8(e.name().as_ref().to_vec()).unwrap().as_str() {
            "SHUFFLE" => {}
            "GO" => {
                self.r#type = e.get_attribute("type").unwrap().parse().unwrap();
            }
            "UN" => {
                if e.attributes().count() < 4 {
                    return;
                }
                self.id = [
                    e.get_attribute("n0").unwrap(),
                    e.get_attribute("n1").unwrap(),
                    e.get_attribute("n2").unwrap(),
                    e.get_attribute("n3").unwrap(),
                ];
                self.dan = e.get_attribute("dan").unwrap().into_num_vec().as_slice().try_into().unwrap();
                self.rate = e.get_attribute("rate").unwrap().into_num_vec().as_slice().try_into().unwrap();
                self.sex = e.get_attribute("sx").unwrap().split(',').map(|s| s.to_string()).collect::<Vec<String>>().try_into().unwrap();
            }
            "TAIKYOKU" => {}
            "INIT" => {
                let seed: Vec<u8> = e.get_attribute("seed").unwrap().into_num_vec();
                let now_kyu: u8 = seed[0];
                let bakaze: &str = if now_kyu < 4 { "E" } else if now_kyu < 8 { "S" } else { "W" };
                let dora_marker: Pai = Pai::from(seed[5]);
                let honba: u8 = seed[1];
                let kyotaku: u8 = seed[2];
                let kyoku: u8 = (now_kyu % 4) + 1;
                let oya: u8 = e.get_attribute("oya").unwrap().parse().unwrap();
                let scores: [i32; 4] = e.get_attribute("ten").unwrap().into_num_vec().iter().map(|&x: &i32| x * 100).collect::<Vec<i32>>().try_into().unwrap();
                let tehais: [[Pai; 13]; 4] = [
                    e.get_attribute("hai0").unwrap().into_pai_vec().as_slice().try_into().unwrap(),
                    e.get_attribute("hai1").unwrap().into_pai_vec().as_slice().try_into().unwrap(),
                    e.get_attribute("hai2").unwrap().into_pai_vec().as_slice().try_into().unwrap(),
                    e.get_attribute("hai3").unwrap().into_pai_vec().as_slice().try_into().unwrap(),
                ];
                self.games.push(Round {
                    junme: [0, 0, 0, 0],
                    last_draw: None,
                    data: RoundData {
                        bakaze: bakaze.to_string(),
                        dora_marker,
                        honba,
                        kyoku,
                        kyotaku,
                        oya,
                        scores,
                        tehais,
                    },
                    game: Vec::new(),
                });
            }
            t if b"TUVW".contains(&t.as_bytes()[0]) && e.attributes().count() == 0 => {
                let tag = t.chars().next().unwrap();
                let actor: u8 = tag.into_actor();
                let game = self.games.last_mut().unwrap();
                game.junme[actor as usize] += 1;
                let pai_num: u8 = t[1..].to_string().parse().unwrap();
                let pai = Pai::from(pai_num);
                game.last_draw = Some(pai);
                game.game.push(MajEvent::Tsumo {
                    junme: game.junme[actor as usize],
                    actor,
                    pai,
                    r#type: "tsumo",
                });
            }
            t if b"DEFG".contains(&t.as_bytes()[0]) && e.attributes().count() == 0 => {
                let tag = t.chars().next().unwrap();
                let actor: u8 = tag.into_actor();
                let game = self.games.last_mut().unwrap();
                let pai_num: u8 = t[1..].to_string().parse().unwrap();
                let pai = Pai::from(pai_num);
                let tsumogiri = Some(pai) == game.last_draw;
                game.game.push(MajEvent::Dahai {
                    junme: game.junme[actor as usize],
                    actor,
                    pai,
                    r#type: "dahai",
                    tsumogiri,
                });
                game.last_draw = None;
            }
            "RYUUKYOKU" => {
                let reason = e.get_attribute("type").unwrap_or("howanpai".to_string());
                let game = self.games.last_mut().unwrap();
                game.game.push(MajEvent::Ryuukyoku {
                    reason,
                    r#type: "ryuukyoku",
                });
                self.update_owari(e);
            }
            "DORA" => {
                let pai_num: u8 = e.get_attribute("hai").unwrap().parse().unwrap();
                let pai = Pai::from(pai_num);
                let game = self.games.last_mut().unwrap();
                game.game.push(MajEvent::Dora {
                    dora_marker: pai,
                    r#type: "dora",
                });
            }
            "REACH" => {
                let actor: u8 = e.get_attribute("who").unwrap().parse().unwrap();
                let typenum: u8 = e.get_attribute("step").unwrap().parse().unwrap();
                let r#type = if typenum == 1 { "riichi" } else { "riichi_accepted" };
                let game = self.games.last_mut().unwrap();
                game.game.push(MajEvent::Reach {
                    junme: game.junme[actor as usize],
                    actor,
                    r#type,
                });
            }
            "AGARI" => {
                let ba = e.get_attribute("ba").unwrap().into_num_vec();
                let ten = e.get_attribute("ten").unwrap().into_num_vec();
                let honba = ba[0];
                let kyotaku = ba[1];
                let hu = ten[0] as u8;
                let score = ten[1];
                let yaku = if let Some(yaku) = e.get_attribute("yaku") {
                    yaku.into_num_vec().chunks(2).flat_map(|y| {
                        let [nowyaku, val] = [y[0], y[1]];
                        return if nowyaku == 52 || nowyaku == 53 || nowyaku == 54 {
                            (0..val).map(|_| (nowyaku, 1)).collect::<Vec<_>>()
                        } else {
                            vec![(nowyaku, val)]
                        };
                    }).collect()
                } else {
                    e.get_attribute("yakuman").unwrap().into_num_vec().iter().map(|&x: &u8| (x, 13)).collect::<Vec<_>>()
                };
                let han = yaku.iter().map(|&(_, val)| val).sum();
                let yaku = yaku.iter().map(|&(nowyaku, _)| nowyaku).collect();
                let hai = e.get_attribute("hai").unwrap().into_pai_vec();
                let machi_num: u8 = e.get_attribute("machi").unwrap().parse().unwrap();
                let machi = Pai::from(machi_num);
                let game = self.games.last_mut().unwrap();
                let actor = e.get_attribute("who").unwrap().parse().unwrap();
                let junme = game.junme[actor as usize];
                let paowho = e.get_attribute("paoWho").map(|x| x.parse().unwrap());
                let fromwho = e.get_attribute("fromWho").unwrap().parse().unwrap();
                let naki = if let Some(naki_raw_list) = e.get_attribute("m") {
                    Some(naki_raw_list.into_num_vec().iter().map(|&naki_raw: &i32| {
                        Self::parse_naki(actor, naki_raw as u32, None)
                    }).collect())
                } else {
                    None
                };
                let dora_marker = e.get_attribute("doraHai").unwrap().into_num_vec().iter().map(|&x: &u8| Pai::from(x)).collect();
                let ura_marker = e.get_attribute("doraHaiUra").map(|x| x.into_num_vec().iter().map(|&x: &u8| Pai::from(x)).collect());
                game.game.push(MajEvent::Agari {
                    honba,
                    kyotaku,
                    junme,
                    hai,
                    naki,
                    machi,
                    han,
                    hu,
                    score,
                    yaku,
                    dora_marker,
                    ura_marker,
                    actor,
                    fromwho,
                    paowho,
                    r#type: "agari",
                });
                self.update_owari(e);
            }
            "N" => {
                let actor: u8 = e.get_attribute("who").unwrap().parse().unwrap();
                let m = e.get_attribute("m").unwrap().parse().unwrap();
                let game = self.games.last_mut().unwrap();
                game.junme[actor as usize] += 1;
                game.game.push(Self::parse_naki(actor, m, Some(game.junme[actor as usize])));
            }
            _ => {
                warn!("Unknown tag: {:?}", e.name());
            }
        }
    }

    fn parse_reader<R: BufRead>(mut xml_reader: Reader<R>) -> Self {
        let mut buf = Vec::new();
        let mut game = Game::default();

        loop {
            match xml_reader.read_event_into(&mut buf) {
                Err(e) => {
                    panic!("Error reading XML at position {}: {:?}", xml_reader.buffer_position(), e);
                }
                Ok(Event::Eof) | Ok(Event::End(_)) => break,
                Ok(Event::Start(ref e)) => {
                    assert_eq!(e.name().as_ref(), b"mjloggm");
                    let ver = e.attributes().find(|a| {
                        a.as_ref().unwrap().key.as_ref() == b"ver"
                    }).unwrap().unwrap();
                    let ver = ver.value.as_ref();
                    if ver != b"2.3" {
                        warn!("Unsupported mjlog version: {:?}. The only supported version is 2.3", ver);
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    game.update(e);
                }
                _ => {}
            }
        }
        game
    }

    pub fn parse_xml_file<P: AsRef<Path>>(path: P) -> Self {
        let reader = Reader::from_file(path).unwrap();
        Self::parse_reader(reader)
    }

    pub fn write_to_json<P: AsRef<Path>>(&self, path: P) {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, json).unwrap();
    }
}