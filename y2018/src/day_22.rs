use anyhow::{Context, Result};
use fxhash::{FxHashMap, FxHashSet};
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use Equipment::*;
use Kind::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
struct Pos {
    pos: u32,
}

impl Pos {
    #[inline(always)]
    fn new(x: u32, y: u32) -> Self {
        Self { pos: x | (y << 16) }
    }
    #[inline(always)]
    fn x(&self) -> u32 {
        self.pos & 0xffff
    }
    #[inline(always)]
    fn y(&self) -> u32 {
        (self.pos >> 16) & 0xffff
    }
}

const BASE: u32 = 20183;
const X_MUL: u32 = 16807;
const Y_MUL: u32 = 48271;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Kind {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd, Hash)]
enum Equipment {
    Torch,
    Climbing,
    Empty,
}

struct Map {
    origin: Pos,
    target: Pos,
    depth: u32,
    geologic_index: FxHashMap<Pos, u32>,
}

impl Map {
    fn new(depth: u32, target: (u32, u32)) -> Self {
        let (x, y) = target;
        Self {
            origin: Pos::new(0, 0),
            target: Pos::new(x, y),
            depth,
            geologic_index: FxHashMap::default(),
        }
    }

    #[inline]
    fn heuristic(&self, pos: Pos) -> u32 {
        let tx = self.target.x();
        let ty = self.target.y();
        let x = pos.x();
        let y = pos.y();
        tx.max(x) - tx.min(x) + ty.max(y) - ty.min(y)
    }

    #[inline]
    fn neighbours(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        let x = pos.x();
        let y = pos.y();
        let out = [
            if x > 0 {
                Some(Pos::new(x - 1, y))
            } else {
                None
            },
            if y > 0 {
                Some(Pos::new(x, y - 1))
            } else {
                None
            },
            Some(Pos::new(x + 1, y)),
            Some(Pos::new(x, y + 1)),
        ];
        out.into_iter().filter_map(|p| p)
    }
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let pat = Regex::new(
            r"depth: (\d+)
target: (\d+),(\d+)",
        )?;
        let m = pat
            .captures(value)
            .with_context(|| format!("Unexpected input: {value}"))?;
        let depth = m.get(1).unwrap().as_str().parse()?;
        let x = m.get(2).unwrap().as_str().parse()?;
        let y = m.get(3).unwrap().as_str().parse()?;
        Ok(Self::new(depth, (x, y)))
    }
}

fn geologic_index(pos: Pos, map: &mut Map) -> u32 {
    if let Some(index) = map.geologic_index.get(&pos) {
        *index
    } else if pos == map.origin || pos == map.target {
        map.geologic_index.insert(pos, 0);
        0
    } else {
        let x = pos.x();
        let y = pos.y();
        if x == 0 {
            map.geologic_index.insert(pos, y * Y_MUL);
            y * Y_MUL
        } else if y == 0 {
            map.geologic_index.insert(pos, x * X_MUL);
            x * X_MUL
        } else {
            let left = erosion_level(Pos::new(x - 1, y), map);
            let right = erosion_level(Pos::new(x, y - 1), map);
            map.geologic_index.insert(pos, left * right);
            left * right
        }
    }
}

fn erosion_level(pos: Pos, map: &mut Map) -> u32 {
    let index = geologic_index(pos, map);
    (index + map.depth).rem_euclid(BASE)
}

fn kind(pos: Pos, map: &mut Map) -> Kind {
    match erosion_level(pos, map).rem_euclid(3) {
        0 => Rocky,
        1 => Wet,
        2 => Narrow,
        _ => unreachable!(),
    }
}

pub fn part_1(s: &str) -> Result<String> {
    let mut map: Map = s.try_into()?;
    let mut risk_level = 0;
    for x in 0..=map.target.x() {
        for y in 0..=map.target.y() {
            risk_level += kind(Pos::new(x, y), &mut map) as u32;
        }
    }
    Ok(risk_level.to_string())
}

fn shortest_path(map: &mut Map) -> u32 {
    let mut work = BinaryHeap::new();
    let mut seen = FxHashSet::default();
    work.push(Reverse((map.heuristic(map.origin), 0, map.origin, Torch)));

    while let Some(Reverse((_, cost, pos, equipment))) = work.pop() {
        if pos == map.target && equipment == Torch {
            return cost;
        }
        // First time going here with this equipment
        if seen.insert((pos, equipment)) {
            let k = kind(pos, map);
            // We can try any other equipment, it costs 7 minutes
            for eq in [Torch, Climbing, Empty] {
                if eq != equipment
                    && matches!(
                        (k, eq),
                        (Rocky, Climbing | Torch)
                            | (Wet, Climbing | Empty)
                            | (Narrow, Torch | Empty)
                    )
                {
                    work.push(Reverse((map.heuristic(pos) + cost + 7, cost + 7, pos, eq)));
                }
            }
            for neighbour in map.neighbours(pos) {
                let k = kind(neighbour, map);
                let can_go = matches!(
                    (k, equipment),
                    (Rocky, Climbing | Torch) | (Wet, Climbing | Empty) | (Narrow, Torch | Empty)
                );
                if can_go {
                    work.push(Reverse((
                        map.heuristic(neighbour) + cost + 1,
                        cost + 1,
                        neighbour,
                        equipment,
                    )));
                }
            }
        }
    }
    unreachable!()
}

pub fn part_2(s: &str) -> Result<String> {
    let mut map: Map = s.try_into()?;
    let time = shortest_path(&mut map);
    Ok(time.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut map = Map::new(510, (10, 10));
        assert_eq!(kind(map.origin, &mut map), Rocky);
        assert_eq!(erosion_level(map.origin, &mut map), 510);
        assert_eq!(geologic_index(Pos::new(1, 0), &mut map), 16807);
        assert_eq!(erosion_level(Pos::new(1, 0), &mut map), 17317);
        assert_eq!(kind(Pos::new(1, 0), &mut map), Wet);
        assert_eq!(geologic_index(Pos::new(0, 1), &mut map), 48271);
        assert_eq!(erosion_level(Pos::new(0, 1), &mut map), 8415);
        assert_eq!(kind(Pos::new(0, 1), &mut map), Rocky);
        assert_eq!(geologic_index(Pos::new(1, 1), &mut map), 145722555);
        assert_eq!(erosion_level(Pos::new(1, 1), &mut map), 1805);
        assert_eq!(kind(Pos::new(1, 1), &mut map), Narrow);
    }

    #[test]
    fn test_input() {
        let s = "depth: 510
target: 10,10";
        let m: Map = s.try_into().unwrap();
        assert_eq!(m.depth, 510);
        assert_eq!(part_1(s).unwrap().as_str(), "114");
    }

    #[test]
    fn test_part2() {
        let s = "depth: 510
target: 10,10";
        let mut m: Map = s.try_into().unwrap();
        let path = shortest_path(&mut m);
        assert_eq!(path, 45);
    }
}
