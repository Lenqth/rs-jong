use crate::structs::*;

use crate::yaku_rules::{Yaku, YakuEntry, YakuPos, YakuRules, ORIGINAL};

pub fn aggregate_tiles(tiles: Vec<Tile>, groups: Vec<Group>) -> TileFreq {
    let mut all_tiles_freq: TileFreq = vec![0; 128];
    for tile in tiles {
        all_tiles_freq[tile as usize] += 1;
    }
    for group in groups {
        for tile in group.components() {
            all_tiles_freq[tile as usize] += 1;
        }
    }
    return all_tiles_freq;
}

pub struct YakuCalc {
    pub agari: Agari,
    pub yakus: Vec<YakuPos>,
    pub score: u32,
}

impl YakuCalc {
    pub fn get_yaku(self) -> Vec<Yaku> {
        self.yakus.into_iter().map(|x| x.yaku).collect()
    }
    fn reduce(self) {}
    pub fn calc_all_yaku(agari: AgariInfo) -> Option<YakuCalc> {
        let mut max_item: Option<Agari> = None;
        let mut max_yaku: Vec<YakuPos> = Vec::new();
        let mut max_score: u32 = 0;
        for item in agari.agaris.iter() {
            let res = Self::calc_yaku(SingleAgariInfo {
                tiles: &agari.tiles,
                agari: item,
                last_tile: agari.last_tile,
            });

            let total: u32 = res.iter().map(|x| x.score()).sum();
            if total > max_score {
                max_item = Some(item.clone());
                max_score = total;
                max_yaku = res.clone();
            }
        }
        if let Some(agari) = max_item {
            return Some(YakuCalc {
                agari: agari,
                yakus: max_yaku,
                score: max_score,
            });
        }
        None
    }
    fn calc_yaku(agari: SingleAgariInfo) -> Vec<YakuPos> {
        let mut res = Vec::<YakuPos>::new();
        match agari.agari {
            Agari::Normal(pat) => {
                res.extend(Self::calc_kongs(pat));
            }
            Agari::SevenPairs => {
                let mut state = -1;
                for v in agari.tiles.iter() {
                    if state == -1 {
                        if *v == 2 {
                            state = 1;
                        } else if *v > 2 {
                            res.push(Yaku::SevenPairs.into());
                            break;
                        }
                    } else {
                        if *v == 2 {
                            state += 1;
                            if state == 7 {
                                res.push(Yaku::SevenShiftedPairs.into());
                                break;
                            }
                        } else {
                            res.push(Yaku::SevenPairs.into());
                            break;
                        }
                    }
                }
            }
            Agari::ThirteenOrphans => {
                res.push(Yaku::ThirteenOrphans.into());
            }
            Agari::Knitted => {
                res.push(Yaku::GreaterKnits.into());
            }
        }
        res
    }
    fn calc_limited(tiles: &Vec<Tile>) -> Vec<YakuPos> {
        let res = vec![];
        res
    }
    fn calc_chows(groups: &Vec<Group>) -> Vec<YakuPos> {
        //    DoubleChow1c,
        //    DoubleChow2c,
        //    SixStraight,
        //    TwoTerminalChows,
        vec![]
    }
    fn calc_waits(groups: &Vec<Group>) -> Vec<YakuPos> {
        vec![]
    }
    fn calc_kongs(groups: &Vec<Group>) -> Vec<YakuPos> {
        let mut kongs: Vec<usize> = Vec::new();
        let mut c_kongs: Vec<usize> = Vec::new();
        for (i, g) in groups.iter().enumerate() {
            if g.is_kong() {
                if g.consealed() {
                    c_kongs.push(i);
                }
                kongs.push(i);
            }
        }
        if c_kongs.len() >= 2 {
            match kongs.len() {
                4 => vec![
                    Yaku::Kong4.with_pos(kongs),
                    Yaku::ConcKong2.with_pos(c_kongs),
                ],
                3 => vec![
                    Yaku::Kong3.with_pos(kongs),
                    Yaku::ConcKong2.with_pos(c_kongs),
                ],
                2 => vec![Yaku::ConcKong2.with_pos(c_kongs)],
                _ => panic!("Impossible"),
            }
        } else if c_kongs.len() >= 1 {
            match kongs.len() {
                4 => vec![
                    Yaku::Kong4.with_pos(kongs),
                    Yaku::ConcKong.with_pos(c_kongs),
                ],
                3 => vec![
                    Yaku::Kong3.with_pos(kongs),
                    Yaku::ConcKong.with_pos(c_kongs),
                ],
                2 => vec![
                    Yaku::Kong2.with_pos(kongs),
                    Yaku::ConcKong.with_pos(c_kongs),
                ],
                1 => vec![Yaku::ConcKong.with_pos(c_kongs)],
                _ => panic!("Impossible"),
            }
        } else {
            match kongs.len() {
                4 => vec![Yaku::Kong4.with_pos(kongs)],
                3 => vec![Yaku::Kong3.with_pos(kongs)],
                2 => vec![Yaku::Kong2.with_pos(kongs)],
                1 => vec![Yaku::Kong1.with_pos(kongs)],
                _ => vec![],
            }
        }
    }
}

#[test]
fn test_aggregate_tiles() {
    assert_eq!(
        aggregate_tiles(
            vec![1, 2, 3],
            vec![Group::Pong(7), Group::ConcKong(8), Group::Chow(3)]
        )[0..=9],
        [0, 1, 1, 2, 1, 1, 0, 3, 4, 0]
    )
}
use crate::agari::is_agari;
pub fn calcurate_score(t: TilesInfo) -> Option<YakuCalc> {
    let agari = is_agari(&t)?;
    YakuCalc::calc_all_yaku(agari)
}

mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_kongs() -> Result<(), Box<dyn std::error::Error>> {
        use self::Group::*;
        let kong4 = TilesInfo::parse_full("*1111mk *2222mk *3333mk *4444mk RR")?;
        assert_eq!(
            kong4,
            TilesInfo {
                tiles: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                mentu: vec![MinKong(1), MinKong(2), MinKong(3), MinKong(4)],
                last_tile: LastTile::Claimed(54)
            }
        );
        assert_eq!(
            calcurate_score(kong4).unwrap().get_yaku(),
            vec![Yaku::Kong4]
        );
        Ok(())
    }

    #[test]
    fn test_load() {
        assert_eq!(Yaku::Kong4.name(), "四槓");
        assert_eq!(Yaku::Kong4.score(), 88);
    }
}
