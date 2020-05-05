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

enum Yaku {
    // 88
    FourWinds,
    ThreeDragons,
    AllGreen,
    NineGates,
    FourKongs,
    SevenShiftedPairs,
    ThirteenOrphans,

    // 66
    AllTerminals,
    LittleFourWinds,
    AllHonors,
    ConcPongs4,
    Souryu1,

    // 48
    SameChow4,
    ShiftPong4,
    // 32
}

struct Yaku {}

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
