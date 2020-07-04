pub fn sweep_isolated(tile_fq: &Vec<i8>) -> Vec<i8> {
    use std::cmp::{max, min};
    let capacity = tile_fq.len();
    let mut res = vec![0; capacity];
    let mut i: usize = 0;
    while i < capacity {
        let l = max(0, (i as i32) - 2) as usize;
        let r = min(capacity - 1, i + 2) as usize;
        if tile_fq[i] >= 2 {
            res[i] = tile_fq[i];
        } else {
            for j in l..=r {
                if i == j {
                    continue;
                }
                if tile_fq[j] > 0 {
                    res[i] = tile_fq[i];
                    break;
                }
            }
        }
        i += 1;
    }
    res
}

use crate::structs::TilesInfo;
use rand::seq::SliceRandom; // 0.7.2

pub fn random_tiles(n: usize) -> TilesInfo {
    let mut res = TilesInfo::new();
    let tiles: Vec<usize> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 17, 18, 19, 20, 21, 22, 23, 24, 25, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 49, 50, 51, 52, 53, 54, 55,
    ];
    for i in tiles.repeat(4).choose_multiple(&mut rand::thread_rng(), n) {
        res.tiles[*i] += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_random() {
        assert_eq!(
            random_tiles(136).tiles,
            vec![
                0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0,
                0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
}
