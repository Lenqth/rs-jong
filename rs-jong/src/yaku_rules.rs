use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct YakuEntry {
    name_en: String,
    name_ch: String,
    score: u32,
    overrides: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct YakuRules {
    pub yaku_info: HashMap<String, YakuEntry>,
}

lazy_static! {
    pub static ref ORIGINAL: YakuRules = {
        use std::io::Read;
        let mut f = std::fs::File::open("./src/original.yaml").unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        serde_yaml::from_str(buffer.as_str()).unwrap()
    };
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
    SamePong3c,
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct YakuPos {
    pub yaku: Yaku,
    pub target_mentu_pos: Vec<usize>,
}

impl From<Yaku> for YakuPos {
    fn from(yaku: Yaku) -> Self {
        return Self {
            yaku: yaku,
            target_mentu_pos: vec![],
        };
    }
}

impl YakuPos {
    pub fn new(yaku: Yaku, p: impl Into<Vec<usize>>) -> Self {
        return Self {
            yaku: yaku,
            target_mentu_pos: p.into(),
        };
    }

    pub fn name(&self) -> &str {
        return self.yaku.name();
    }

    pub fn score(&self) -> u32 {
        return self.yaku.score();
    }
}

impl std::fmt::Display for Yaku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Yaku {
    pub fn name(&self) -> &str {
        let entry = ORIGINAL.yaku_info.get(self.to_string().as_str()).unwrap();
        return entry.name_ch.as_str();
    }

    pub fn score(&self) -> u32 {
        let entry = ORIGINAL.yaku_info.get(self.to_string().as_str()).unwrap();
        return entry.score as u32;
    }

    pub fn with_pos(self, p: impl Into<Vec<usize>>) -> YakuPos {
        YakuPos::new(self, p)
    }
}
