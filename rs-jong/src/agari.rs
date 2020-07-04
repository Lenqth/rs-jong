use std::collections::VecDeque;
use std::sync::RwLock;

use crate::structs::*;
use crate::util::sweep_isolated;

struct FullRecurer {
    full_pops: Vec<i8>,
    allow_chow: bool,
    base: usize,

    state: Vec<Group>,
    results: Vec<Vec<Group>>,
}

impl FullRecurer {
    fn new(pops: &Vec<i8>) -> FullRecurer {
        FullRecurer {
            full_pops: pops.clone(),
            allow_chow: true,
            base: 0,

            state: Vec::new(),
            results: Vec::new(),
        }
    }

    fn start(&mut self, claimed_mentu: &Vec<Group>) -> Vec<Agari> {
        let mut all_results: [Vec<Vec<Group>>; 4] = [
            Vec::default(),
            Vec::default(),
            Vec::default(),
            Vec::default(),
        ];
        let full_pops = self.full_pops.clone();
        let mut pops = [0; 16];
        pops.copy_from_slice(&full_pops[0..16]);
        self.results = Vec::new();
        self.base = 0;
        self.recur_mentu(&mut pops, 1, 0, 0, false);
        all_results[0] = self.results.clone();

        pops.copy_from_slice(&full_pops[16..32]);
        self.results = Vec::new();
        self.base = 16;
        self.recur_mentu(&mut pops, 1, 0, 0, false);
        all_results[1] = self.results.clone();

        pops.copy_from_slice(&full_pops[32..48]);
        self.results = Vec::new();
        self.base = 32;
        self.recur_mentu(&mut pops, 1, 0, 0, false);
        all_results[2] = self.results.clone();

        pops.copy_from_slice(&full_pops[48..64]);
        self.results = Vec::new();
        self.base = 48;
        self.allow_chow = false;
        self.recur_mentu(&mut pops, 1, 0, 0, false);
        all_results[3] = self.results.clone();

        let mut result: Vec<Agari> = Vec::new();
        for a in all_results[0].iter() {
            for b in all_results[1].iter() {
                for c in all_results[2].iter() {
                    for d in all_results[3].iter() {
                        let mut i = Vec::new();
                        i.extend(claimed_mentu.clone());
                        i.extend(a.clone());
                        i.extend(b.clone());
                        i.extend(c.clone());
                        i.extend(d.clone());

                        let mut cnt = 0;
                        for t in i.iter() {
                            if let Group::Pair(_) = t {
                                cnt += 1;
                                if cnt >= 2 {
                                    break;
                                }
                            }
                        }
                        if cnt >= 2 {
                            continue;
                        }
                        i.sort();

                        result.push(Agari::Normal(i));
                    }
                }
            }
        }
        result.sort();
        result
    }

    fn check_kotu(&mut self, pops: &mut [i8], k: usize, s3: u8, at: bool) {
        if pops[k] >= 3 {
            self.state.push(Group::ConcPong((k + self.base) as u8));
            pops[k] -= 3;
            self.recur_mentu(pops, k, 1, s3 + 1, at);
            pops[k] += 3;
            self.state.pop();
        }
    }
    fn check_shuntu(&mut self, pops: &mut [i8], k: usize, s3: u8, at: bool) {
        if k <= 7 {
            if pops[k] >= 1 && pops[k + 1] >= 1 && pops[k + 2] >= 1 {
                self.state.push(Group::ConcChow((k + self.base) as u8));
                pops[k] -= 1;
                pops[k + 1] -= 1;
                pops[k + 2] -= 1;
                self.recur_mentu(pops, k, 1, s3 + 1, at);
                pops[k] += 1;
                pops[k + 1] += 1;
                pops[k + 2] += 1;
                self.state.pop();
            }
        }
    }
    fn check_toitu(&mut self, pops: &mut [i8], k: usize, s3: u8, _at: bool) {
        if pops[k] >= 2 {
            self.state.push(Group::Pair((k + self.base) as u8));
            pops[k] -= 2;
            self.recur_mentu(pops, k + 1, 0, s3, true);
            pops[k] += 2;
            self.state.pop();
        }
    }
    fn recur_mentu(&mut self, pops: &mut [i8], k: usize, step: u8, s3: u8, at: bool) {
        if k > 9 {
            self.results.push(self.state.clone());
            return;
        }
        if step <= 0 {
            self.check_kotu(pops, k, s3, at);
        }
        if step <= 1 && self.allow_chow {
            self.check_shuntu(pops, k, s3, at);
        }
        if !at {
            if step <= 2 {
                self.check_toitu(pops, k, s3, at);
            }
        }
        if pops[k] == 0 {
            self.recur_mentu(pops, k + 1, 0, s3, at);
        }
    }
}

fn is_agari_normal(hand: &TilesInfo) -> Vec<Agari> {
    let mut recurer = FullRecurer::new(&hand.tiles);
    recurer.start(&hand.mentu)
}

fn is_agari_7pairs(hand: &TilesInfo) -> Vec<Agari> {
    let mut pairs = 0;
    for v in hand.tiles.iter() {
        if v % 2 == 0 {
            pairs += v / 2;
        } else {
            return Vec::new();
        }
    }
    if pairs != 7 {
        return Vec::new();
    }
    vec![Agari::SevenPairs]
}

fn is_agari_13orphans(hand: &TilesInfo) -> Vec<Agari> {
    let oks: [usize; 13] = [
        1,
        9,
        16 + 1,
        16 + 9,
        32 + 1,
        32 + 9,
        48 + 1,
        48 + 2,
        48 + 3,
        48 + 4,
        48 + 5,
        48 + 6,
        48 + 7,
    ];
    let mut two = false;
    for i in oks.iter() {
        match hand.tiles[*i] {
            1 => {
                continue;
            }
            2 => {
                if two {
                    return Vec::new();
                }
                two = true;
            }
            _ => {
                return Vec::new();
            }
        }
    }
    vec![Agari::ThirteenOrphans]
}

fn is_agari_knitnormal(hand: &TilesInfo) -> Vec<Agari> {
    const KNITS: [[usize; 3]; 4] = [[0, 0, 0], [1, 4, 7], [2, 5, 8], [3, 6, 9]];
    const COMBS: [(usize, usize, usize); 6] = [
        (1, 2, 3),
        (1, 3, 2),
        (2, 1, 3),
        (2, 3, 1),
        (3, 1, 2),
        (3, 1, 2),
    ];
    let mut res = Vec::new();
    'outer: for (m, p, s) in COMBS.iter() {
        let mut fq = hand.tiles.clone();
        for i in KNITS[*m].iter() {
            if fq[MAN + i] <= 0 {
                continue 'outer;
            }
            fq[MAN + i] -= 1;
        }
        for i in KNITS[*p].iter() {
            if fq[PIN + i] <= 0 {
                continue 'outer;
            }
            fq[PIN + i] -= 1;
        }
        for i in KNITS[*s].iter() {
            if fq[SOU + i] <= 0 {
                continue 'outer;
            }
            fq[SOU + i] -= 1;
        }
        let mut recurer = FullRecurer::new(&fq);
        for item in recurer.start(&Vec::new()).iter() {
            if let Agari::Normal(v) = item {
                let mut v = v.clone();
                v.push(Group::KnitChow((MAN + m) as u8));
                v.push(Group::KnitChow((PIN + p) as u8));
                v.push(Group::KnitChow((SOU + s) as u8));
                v.sort();
                res.push(Agari::Normal(v));
            }
        }
    }
    return res;
}

fn is_agari_knitted(hand: &TilesInfo) -> Vec<Agari> {
    const KNITS: [[usize; 3]; 4] = [[0, 0, 0], [1, 4, 7], [2, 5, 8], [3, 6, 9]];
    const COMBS: [(usize, usize, usize); 6] = [
        (1, 2, 3),
        (1, 3, 2),
        (2, 1, 3),
        (2, 3, 1),
        (3, 1, 2),
        (3, 1, 2),
    ];
    let fq: &Vec<i8> = &hand.tiles;
    'outer: for (m, p, s) in COMBS.iter() {
        let mut cnt = 14;
        for i in 49..56 {
            if fq[i] <= 0 {
                continue;
            }
            if fq[i] >= 2 {
                continue 'outer;
            }
            cnt -= 1;
        }
        for i in KNITS[*m].iter() {
            let i = MAN + i;
            if fq[i] <= 0 {
                continue;
            }
            if fq[i] >= 2 {
                continue 'outer;
            }
            cnt -= 1;
        }
        for i in KNITS[*p].iter() {
            let i = PIN + i;
            if fq[i] <= 0 {
                continue;
            }
            if fq[i] >= 2 {
                continue 'outer;
            }
            cnt -= 1;
        }
        for i in KNITS[*s].iter() {
            let i = SOU + i;
            if fq[i] <= 0 {
                continue;
            }
            if fq[i] >= 2 {
                continue 'outer;
            }
            cnt -= 1;
        }
        if cnt == 0 {
            return vec![Agari::Knitted];
        }
    }
    Vec::new()
}

pub fn is_agari(hand: &TilesInfo) -> Option<AgariInfo> {
    let mut res: Vec<Agari> = Vec::new();
    res.extend(is_agari_normal(&hand));
    res.extend(is_agari_7pairs(&hand));
    res.extend(is_agari_13orphans(&hand));
    res.extend(is_agari_knitnormal(&hand));
    res.extend(is_agari_knitted(&hand));
    res.sort();
    if res.is_empty() {
        None
    } else {
        Some(AgariInfo {
            tiles: hand.tiles.clone(),
            agaris: res,
            last_tile: hand.last_tile,
        })
    }
}

#[cfg(test)]
mod tests {
    use self::Agari::*;
    use self::Group::*;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_13orphans() -> Result<(), Box<dyn std::error::Error>> {
        assert_ne!(
            is_agari_13orphans(&TilesInfo::from_str("19m19p19sNEWSHRG1m")?),
            Vec::<Agari>::new()
        );
        assert_ne!(
            is_agari_13orphans(&TilesInfo::from_str("19m19p19sNEWSHRGG")?),
            Vec::<Agari>::new()
        );
        assert_eq!(
            is_agari_13orphans(&TilesInfo::from_str("19m11p19sNEWSHRGG")?),
            Vec::<Agari>::new()
        );
        assert_eq!(
            is_agari_13orphans(&TilesInfo::from_str("19m11p179sNEWSHRG")?),
            Vec::<Agari>::new()
        );
        assert_eq!(
            is_agari_13orphans(&TilesInfo::from_str("19m1p19sNEWSHRGGG")?),
            Vec::<Agari>::new()
        );
        Ok(())
    }
    #[test]
    fn test_7pairs() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            is_agari_7pairs(&TilesInfo::from_str("11335577m224466p")?),
            vec![SevenPairs]
        );
        assert_eq!(
            is_agari_7pairs(&TilesInfo::from_str("11335577m2266pSS")?),
            vec![SevenPairs]
        );
        assert_eq!(
            is_agari_7pairs(&TilesInfo::from_str("1111m225599pSSHH")?),
            vec![SevenPairs]
        );
        assert_eq!(
            is_agari_7pairs(&TilesInfo::from_str("11335567m2266pSS")?),
            Vec::<Agari>::new()
        );
        Ok(())
    }
    #[test]
    fn test_normal() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            is_agari_normal(&TilesInfo::from_str("123m456p789sEEESS")?),
            vec![Normal(vec![
                ConcChow(1),
                ConcChow(20),
                ConcChow(39),
                ConcPong(49),
                Pair(50)
            ])]
        );
        assert_eq!(
            is_agari_normal(&TilesInfo::from_str("123m456p789sESWNN")?),
            Vec::<Agari>::new()
        );
        assert_eq!(
            is_agari_normal(&TilesInfo::from_str("22334455667788m")?),
            vec![
                Normal(vec![
                    ConcChow(2),
                    ConcChow(2),
                    ConcChow(5),
                    ConcChow(5),
                    Pair(8)
                ]),
                Normal(vec![
                    ConcChow(2),
                    ConcChow(2),
                    ConcChow(6),
                    ConcChow(6),
                    Pair(5)
                ]),
                Normal(vec![
                    ConcChow(3),
                    ConcChow(3),
                    ConcChow(6),
                    ConcChow(6),
                    Pair(2)
                ])
            ]
        );
        Ok(())
    }
    #[test]
    fn test_knitted_normal() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            is_agari_knitnormal(&TilesInfo::from_str("147m258p369s567s11p")?),
            vec![Normal(vec![
                ConcChow(37),
                KnitChow(1),
                KnitChow(18),
                KnitChow(35),
                Pair(17),
            ])]
        );
        Ok(())
    }
    #[test]
    fn test_knitted() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            is_agari_knitted(&TilesInfo::from_str("147m2p369sESWNHRG")?),
            vec![Knitted]
        );
        assert_eq!(
            is_agari_knitted(&TilesInfo::from_str("147m2p399sESWNHRG")?),
            vec![]
        );
        Ok(())
    }
    #[test]
    fn test_total() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            is_agari(&TilesInfo::from_str("147m25811p369567s")?)
                .map(|x| x.agaris)
                .unwrap_or_else(|| vec![]),
            vec![Normal(vec![
                ConcChow(37),
                KnitChow(1),
                KnitChow(18),
                KnitChow(35),
                Pair(17),
            ])]
        );
        assert_eq!(
            is_agari(&TilesInfo::from_str("147m2p369sESWNHRG")?)
                .map(|x| x.agaris)
                .unwrap_or_else(|| vec![]),
            vec![Knitted]
        );
        assert_eq!(
            is_agari(&TilesInfo::from_str("123234345m67888p")?)
                .map(|x| x.agaris)
                .unwrap_or_else(|| vec![]),
            vec![Normal(vec![
                ConcChow(1),
                ConcChow(2),
                ConcChow(3),
                ConcChow(22),
                Pair(24)
            ])]
        );
        assert_eq!(
            is_agari(&TilesInfo::from_str("55678s45667p5p")?)
                .map(|x| x.agaris)
                .unwrap_or_else(|| vec![]),
            vec![Normal(vec![
                ConcChow(20),
                ConcChow(21),
                ConcChow(38),
                Pair(37),
            ])]
        );
        assert_eq!(
            is_agari(&TilesInfo::from_str("222m333m444m555m99m")?)
                .map(|x| x.agaris)
                .unwrap_or_else(|| vec![]),
            vec![
                Normal(vec![
                    ConcChow(2),
                    ConcChow(2),
                    ConcChow(2),
                    ConcPong(5),
                    Pair(9)
                ]),
                Normal(vec![
                    ConcChow(3),
                    ConcChow(3),
                    ConcChow(3),
                    ConcPong(2),
                    Pair(9)
                ]),
                Normal(vec![
                    ConcPong(2),
                    ConcPong(3),
                    ConcPong(4),
                    ConcPong(5),
                    Pair(9)
                ])
            ]
        );
        assert_eq!(
            is_agari(&TilesInfo::from_str("22334455667788m")?)
                .map(|x| x.agaris)
                .unwrap_or_else(|| vec![]),
            vec![
                Normal(vec![
                    ConcChow(2),
                    ConcChow(2),
                    ConcChow(5),
                    ConcChow(5),
                    Pair(8)
                ]),
                Normal(vec![
                    ConcChow(2),
                    ConcChow(2),
                    ConcChow(6),
                    ConcChow(6),
                    Pair(5)
                ]),
                Normal(vec![
                    ConcChow(3),
                    ConcChow(3),
                    ConcChow(6),
                    ConcChow(6),
                    Pair(2)
                ]),
                SevenPairs
            ]
        );
        assert_eq!(is_agari(&TilesInfo::from_str("147m3469p1258sSSG")?), None);

        Ok(())
    }
    use crate::util::random_tiles;
    #[bench]
    fn bench_agari(b: &mut Bencher) {
        let mut cases = Vec::new();
        for _ in 0..10000 {
            cases.push(random_tiles(14));
        }
        let cases = cases;
        b.iter(|| {
            (0..10000)
                .map(|i| is_agari(&cases[i]))
                .collect::<Vec<Option<AgariInfo>>>()
        });
    }
}
