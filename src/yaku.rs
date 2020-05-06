use crate::structs::*;

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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Yaku {
    // 88
    Wind4,
    Dragon3,
    AllGreen,
    NineGates,
    Kong4,
    SevenShiftedPairs,
    ThirteenOrphans,

    // 66
    AllTerminals,
    Wind4s,
    AllHonors,
    ConcPongs4,
    Souryu1c,

    // 48
    SameChow4,
    ShiftPong4,

    // 32
    ShiftChow4,
    Kong3,
    AllTermOrHonor,

    // 24
    SevenPairs,
    GreaterKnits,
    AllEvenPungs,
    FullFlush,
    SameChow3,
    ShiftPong3,
    All789,
    All456,
    All123,

    // 16
    PureStraight,
    Souryu3c,
    ShiftChow3,
    AllContain5,
    SamePong3,
    ConcPong3,

    // 12
    LesserKnits,
    KnitStraight,
    All1234,
    All6789,
    Wind3,

    // 8
    MixedStraight,
    Reversible,
    ShiftPong3c,
    ChickenHand,
    LastDraw,
    LastClaim,
    Replacement,
    RobKong,
    ConcKong2,

    // 6
    AllPong,
    HalfFlush,
    ShiftChow3c,
    AllTypes,
    AllMelded,
    Dragon2,

    // 4
    AllContainOutside,
    AllConc,
    Kong2,
    LastTile,

    // 2
    Dragon1,
    PrevWind,
    SeatWind,
    ConcealedHand,
    AllChows,
    FourTiles,
    SamePong2,
    ConcPong2,
    ConcKong,
    AllSimple,

    // 1
    DoubleChow1c,
    DoubleChow2c,
    SixStraight,
    TwoTerminalChows,
    TerminalPong,
    Kong1,
    OneVoid,
    NoHonor,
    EdgeWait,
    ClosedWait,
    SingleWait,
    SelfDraw,

    Flower,
}

impl Yaku {
    pub fn name(&self) -> &str {
        use self::Yaku::*;
        match self {
            // 88
            Wind4 => "",
            Dragon3 => "",
            AllGreen => "",
            NineGates => "",
            FourKongs => "",
            SevenShiftedPairs => "",
            ThirteenOrphans => "",

            // 66
            AllTerminals => "",
            Wind4s => "",
            AllHonors => "",
            ConcPongs4 => "",
            Souryu1c => "",

            // 48
            SameChow4 => "",
            ShiftPong4 => "",

            // 32
            ShiftChow4 => "",
            Kong3 => "",
            AllTermOrHonor => "",

            // 24
            SevenPairs => "",
            GreaterKnits => "",
            AllEvenPungs => "",
            FullFlush => "",
            SameChow3 => "",
            ShiftPong3 => "",
            All789 => "",
            All456 => "",
            All123 => "",

            // 16
            PureStraight => "",
            Souryu3c => "",
            ShiftChow3 => "",
            AllContain5 => "",
            SamePong3 => "",
            ConcPong3 => "",

            // 12
            LesserKnits => "",
            KnitStraight => "",
            All1234 => "",
            All6789 => "",
            Wind3 => "",

            // 8
            MixedStraight => "",
            Reversible => "",
            ShiftPong3c => "",
            ChickenHand => "",
            LastDraw => "",
            LastClaim => "",
            Replacement => "",
            RobKong => "",
            ConcKong2 => "",

            // 6
            AllPong => "",
            HalfFlush => "",
            ShiftChow3c => "",
            AllTypes => "",
            AllMelded => "",
            Dragon2 => "",

            // 4
            AllContainOutside => "",
            AllConc => "",
            Kong2 => "",
            LastTile => "",

            // 2
            Dragon1 => "",
            PrevWind => "",
            SeatWind => "",
            ConcealedHand => "",
            AllChows => "",
            FourTiles => "",
            SamePong2 => "",
            ConcPong2 => "",
            ConcKong => "",
            AllSimple => "",

            // 1
            DoubleChow1c => "",
            DoubleChow2c => "",
            SixStraight => "",
            TwoTerminalChows => "",
            TerminalPong => "",
            Kong1 => "",
            OneVoid => "",
            NoHonor => "",
            EdgeWait => "",
            ClosedWait => "",
            SingleWait => "",
            SelfDraw => "",

            Flower => "",
        }
    }

    pub fn score(&self) -> u32 {
        use self::Yaku::*;
        match self {
            // 88
            Wind4 => 88,
            Dragon3 => 88,
            AllGreen => 88,
            NineGates => 88,
            FourKongs => 88,
            SevenShiftedPairs => 88,
            ThirteenOrphans => 88,

            // 66
            AllTerminals => 64,
            Wind4s => 64,
            AllHonors => 64,
            ConcPongs4 => 64,
            Souryu1c => 64,

            // 48
            SameChow4 => 48,
            ShiftPong4 => 48,

            // 32
            ShiftChow4 => 32,
            Kong3 => 32,
            AllTermOrHonor => 32,

            // 24
            SevenPairs => 24,
            GreaterKnits => 24,
            AllEvenPungs => 24,
            FullFlush => 24,
            SameChow3 => 24,
            ShiftPong3 => 24,
            All789 => 24,
            All456 => 24,
            All123 => 24,

            // 16
            PureStraight => 16,
            Souryu3c => 16,
            ShiftChow3 => 16,
            AllContain5 => 16,
            SamePong3 => 16,
            ConcPong3 => 16,

            // 12
            LesserKnits => 12,
            KnitStraight => 12,
            All1234 => 12,
            All6789 => 12,
            Wind3 => 12,

            // 8
            MixedStraight => 8,
            Reversible => 8,
            ShiftPong3c => 8,
            ChickenHand => 8,
            LastDraw => 8,
            LastClaim => 8,
            Replacement => 8,
            RobKong => 8,
            ConcKong2 => 8,

            // 6
            AllPong => 6,
            HalfFlush => 6,
            ShiftChow3c => 6,
            AllTypes => 6,
            AllMelded => 6,
            Dragon2 => 6,

            // 4
            AllContainOutside => 4,
            AllConc => 4,
            Kong2 => 4,
            LastTile => 4,

            // 2
            Dragon1 => 2,
            PrevWind => 2,
            SeatWind => 2,
            ConcealedHand => 2,
            AllChows => 2,
            FourTiles => 2,
            SamePong2 => 2,
            ConcPong2 => 2,
            ConcKong => 2,
            AllSimple => 2,

            // 1
            DoubleChow1c => 1,
            DoubleChow2c => 1,
            SixStraight => 1,
            TwoTerminalChows => 1,
            TerminalPong => 1,
            Kong1 => 1,
            OneVoid => 1,
            NoHonor => 1,
            EdgeWait => 1,
            ClosedWait => 1,
            SingleWait => 1,
            SelfDraw => 1,

            Flower => 1,
        }
    }
}

pub struct YakuCalc {
    pub agari: Agari,
    pub yakus: Vec<Yaku>,
    pub score: u32,
}

impl YakuCalc {
    pub fn calc_all_yaku(agari: AgariInfo) -> Option<YakuCalc> {
        let mut max_item: Option<Agari> = None;
        let mut max_yaku: Vec<Yaku> = Vec::new();
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
    fn calc_yaku(agari: SingleAgariInfo) -> Vec<Yaku> {
        let mut res = Vec::<Yaku>::new();
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
                            res.push(Yaku::SevenPairs);
                            break;
                        }
                    } else {
                        if *v == 2 {
                            state += 1;
                            if state == 7 {
                                res.push(Yaku::SevenShiftedPairs);
                                break;
                            }
                        } else {
                            res.push(Yaku::SevenPairs);
                            break;
                        }
                    }
                }
            }
            Agari::ThirteenOrphans => {
                res.push(Yaku::ThirteenOrphans);
            }
            Agari::Knitted => {
                res.push(Yaku::GreaterKnits);
            }
        }
        res
    }
    fn calc_kongs(groups: &Vec<Group>) -> Vec<Yaku> {
        let mut kongs = 0;
        let mut c_kongs = 0;
        for g in groups.iter() {
            if g.is_kong() {
                if g.consealed() {
                    c_kongs += 1;
                }
                kongs += 1;
            }
        }
        if c_kongs >= 2 {
            match kongs {
                4 => vec![Yaku::Kong4, Yaku::ConcKong2],
                3 => vec![Yaku::Kong3, Yaku::ConcKong2],
                2 => vec![Yaku::ConcKong2],
                _ => panic!("Impossible"),
            }
        } else if c_kongs >= 1 {
            match kongs {
                4 => vec![Yaku::Kong4, Yaku::ConcKong],
                3 => vec![Yaku::Kong3, Yaku::ConcKong],
                2 => vec![Yaku::Kong2, Yaku::ConcKong],
                1 => vec![Yaku::ConcKong],
                _ => panic!("Impossible"),
            }
        } else {
            match kongs {
                4 => vec![Yaku::Kong4],
                3 => vec![Yaku::Kong3],
                2 => vec![Yaku::Kong2],
                1 => vec![Yaku::Kong1],
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
            calcurate_score(kong4)
                .map(|x| x.yakus)
                .unwrap_or_else(|| vec![]),
            vec![Yaku::Kong4]
        );
        Ok(())
    }
}
