type Tile = u8;
type TileFreq = [u8; 128];

enum EnumGroup {
    Chow,
    Pong,
    ConcKong,
    ApKong,
    MinKong,
    KnitChow,
}

struct Group {
    head: u8,
    group_type: EnumGroup,
}
impl Group {
    fn new(head: u8, group_type: EnumGroup) -> Group {
        return Group {
            head: head,
            group_type: group_type,
        };
    }
    fn components(self) -> Vec<Tile> {
        let head = self.head;
        match self.group_type {
            EnumGroup::Chow => vec![head, head + 1, head + 2],
            EnumGroup::Pong => vec![head, head, head],
            EnumGroup::ConcKong => vec![head, head, head, head],
            EnumGroup::ApKong => vec![head, head, head, head],
            EnumGroup::MinKong => vec![head, head, head, head],
            EnumGroup::KnitChow => panic!(),
        }
    }
}

fn aggregate_tiles(tiles: Vec<Tile>, groups: Vec<Group>) -> TileFreq {
    let mut all_tiles_freq: TileFreq = [0; 128];
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

#[test]
fn test_aggregate_tiles() {
    assert_eq!(
        aggregate_tiles(
            vec![1, 2, 3],
            vec![
                Group::new(7, EnumGroup::Pong),
                Group::new(8, EnumGroup::ConcKong),
                Group::new(3, EnumGroup::Chow)
            ]
        )[0..9],
        [0, 1, 1, 2, 1, 1, 0, 3, 4, 0]
    )
}
