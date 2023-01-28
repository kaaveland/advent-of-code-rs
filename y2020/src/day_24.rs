use anyhow::Result;
use fxhash::FxHashSet as HashSet;

// https://en.wikipedia.org/wiki/Hexagonal_Efficient_Coordinate_System#/media/File:HECS_Nearest_Neighbors.png
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct HexCoord {
    a: i32,
    r: i32,
    c: i32,
}

impl HexCoord {
    fn east(&self) -> Self {
        HexCoord {
            a: self.a,
            r: self.r,
            c: self.c + 1,
        }
    }
    fn west(&self) -> Self {
        HexCoord {
            a: self.a,
            r: self.r,
            c: self.c - 1,
        }
    }
    fn northwest(&self) -> Self {
        let a_neg = 1 - self.a;
        HexCoord {
            a: a_neg,
            r: self.r - a_neg,
            c: self.c - a_neg,
        }
    }
    fn northeast(&self) -> Self {
        let a_neg = 1 - self.a;
        HexCoord {
            a: a_neg,
            r: self.r - a_neg,
            c: self.c + self.a,
        }
    }
    fn southwest(&self) -> Self {
        let a_neg = 1 - self.a;
        HexCoord {
            a: a_neg,
            r: self.r + self.a,
            c: self.c - a_neg,
        }
    }
    fn southeast(&self) -> Self {
        HexCoord {
            a: 1 - self.a,
            r: self.r + self.a,
            c: self.c + self.a,
        }
    }
}
type HexNeighour = fn(&HexCoord) -> HexCoord;
const PREFIX_MAP: [(&str, HexNeighour); 6] = [
    ("nw", HexCoord::northwest),
    ("ne", HexCoord::northeast),
    ("w", HexCoord::west),
    ("e", HexCoord::east),
    ("sw", HexCoord::southwest),
    ("se", HexCoord::southeast),
];

fn walk(mut path: &str) -> HexCoord {
    let mut pos = HexCoord { a: 0, r: 0, c: 0 };
    while !path.is_empty() {
        let mut found = false;
        for (prefix, neighbour) in PREFIX_MAP {
            if path.starts_with(prefix) {
                found = true;
                pos = neighbour(&pos);
                path = &path[prefix.len()..];
                break;
            }
        }
        if !found {
            panic!("Illegal prefix in {path}");
        }
    }
    pos
}

fn lay_floor(input: &str) -> HashSet<HexCoord> {
    let mut flipped_tiles = HashSet::default();
    for path in input.lines().filter(|line| !line.is_empty()) {
        let pos = walk(path);
        if !flipped_tiles.insert(pos) {
            flipped_tiles.remove(&pos);
        }
    }
    flipped_tiles
}

pub fn part_1(input: &str) -> Result<String> {
    let n = lay_floor(input).len();
    Ok(format!("{n}"))
}
