use std::fmt::{Display, Formatter};
use std::str::FromStr;
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Agari {
    Normal(Vec<Group>),
    SevenPairs,
    ThirteenOrphans,

    Knitted,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AgariInfo {
    pub tiles: Vec<i8>,
    pub agaris: Vec<Agari>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TilesInfo {
    pub tiles: Vec<i8>,
    pub mentu: Option<Vec<Group>>,
}

impl TilesInfo {
    pub fn new() -> TilesInfo {
        return TilesInfo {
            tiles: vec![0i8; 128],
            mentu: None,
        };
    }

    pub fn get_list(&self) -> Vec<Tile> {
        let mut res = Vec::<Tile>::new();
        for (i, v) in self.tiles.iter().enumerate() {
            for _ in 0..(*v) {
                res.push(i as Tile);
            }
        }
        res
    }

    pub fn from_list(list: Vec<Tile>) -> Self {
        let mut res = Self::new();
        for item in list.iter() {
            res.tiles[(*item) as usize] += 1;
        }
        res
    }

    fn parse(s: &str) -> Result<Vec<Tile>, std::fmt::Error> {
        let mut list = Vec::<Tile>::new();
        let mut pending_numbers = Vec::<u8>::new();
        for c in s.chars() {
            if !pending_numbers.is_empty() {
                match c {
                    'm' => {
                        let offset: u8 = 0;
                        for n in pending_numbers.iter() {
                            list.push(offset + n);
                        }
                        pending_numbers.clear();
                    }
                    'p' => {
                        let offset: u8 = 16;
                        for n in pending_numbers.iter() {
                            list.push(offset + n);
                        }
                        pending_numbers.clear();
                    }
                    's' => {
                        let offset: u8 = 32;
                        for n in pending_numbers.iter() {
                            list.push(offset + n);
                        }
                        pending_numbers.clear();
                    }
                    ' ' => {}
                    _ => {
                        if ('0'..='9').contains(&c) {
                            let n = (c as u8) - ('0' as u8);
                            pending_numbers.push(n);
                        } else {
                            return Err(std::fmt::Error);
                        }
                    }
                }
            } else {
                match c {
                    'E' => list.push(49),
                    'S' => list.push(50),
                    'W' => list.push(51),
                    'N' => list.push(52),
                    'H' => list.push(53),
                    'R' => list.push(54),
                    'G' => list.push(55),
                    'F' => list.push(65),
                    ' ' => {}
                    _ => {
                        if ('0'..='9').contains(&c) {
                            let n = (c as u8) - ('0' as u8);
                            pending_numbers.push(n);
                        } else {
                            return Err(std::fmt::Error);
                        }
                    }
                }
            }
        }
        Ok(list)
    }
    fn tile_name(t: &Tile) -> Option<String> {
        let t = *t;
        if t < 16 {
            return Some(format!("{}m", t));
        } else if t < 32 {
            return Some(format!("{}p", t - 16));
        } else if t < 48 {
            return Some(format!("{}s", t - 32));
        } else if t < 64 {
            return Some("ESWNHRG".chars().nth((t - 48) as usize)?.to_string());
        } else if t == 65 {
            return Some("F".to_string());
        } else {
            return None;
        }
    }
    pub fn from_str(s: &str) -> Result<Self, std::fmt::Error> {
        Ok(Self::from_list(Self::parse(s)?))
    }
}

impl FromStr for TilesInfo {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, std::fmt::Error> {
        Self::from_str(s)
    }
}
impl Display for TilesInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let list = self.get_list();
        let it = list.iter().map(Self::tile_name);
        let res: Option<Vec<String>> = it.collect();
        if let Some(res) = res {
            write!(f, "{}", res.join(""))
        } else {
            Err(std::fmt::Error)
        }
    }
}

pub type Tile = u8;
pub type TileFreq = Vec<Tile>;

pub static MAN: usize = 0;
pub static PIN: usize = 16;
pub static SOU: usize = 32;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Group {
    ConcChow(u8),
    ConcPong(u8),
    KnitChow(u8),
    Pair(u8),
    Chow(u8),
    Pong(u8),
    ConcKong(u8),
    ApKong(u8),
    MinKong(u8),
}

impl Group {
    pub fn components(self) -> Vec<Tile> {
        match self {
            Self::Chow(head) => vec![head, head + 1, head + 2],
            Self::Pong(head) => vec![head, head, head],
            Self::ConcKong(head) => vec![head, head, head, head],
            Self::ApKong(head) => vec![head, head, head, head],
            Self::MinKong(head) => vec![head, head, head, head],
            Self::ConcChow(head) => vec![head, head + 1, head + 2],
            Self::ConcPong(head) => vec![head, head, head],
            Self::Pair(head) => vec![head, head],
            Self::KnitChow(head) => vec![head, head + 3, head + 6],
        }
    }

    pub fn consealed(self) -> bool {
        match self {
            Self::Chow(_) => false,
            Self::Pong(_) => false,
            Self::ApKong(_) => false,
            Self::MinKong(_) => false,
            Self::ConcKong(_) => true,
            Self::ConcChow(_) => true,
            Self::ConcPong(_) => true,
            Self::Pair(_) => true,
            Self::KnitChow(_) => true,
        }
    }
}

mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            TilesInfo::from_str("123m6m8m9mW")?.tiles[1..=9],
            [1, 1, 1, 0, 0, 1, 0, 1, 1]
        );
        Ok(())
    }
}