use anyhow::{anyhow, Error, Result};
use fxhash::FxHashMap;
use itertools::Itertools;
use std::mem;
use Tile::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
enum Tile {
    Open,
    Tree,
    Lumberyard,
}

impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Open),
            '|' => Ok(Tree),
            '#' => Ok(Lumberyard),
            _ => Err(anyhow!("invalid tile: {c}")),
        }
    }
}

fn parse(s: &str) -> Result<(usize, Vec<Tile>)> {
    let mut width = 0;
    let mut map = Vec::with_capacity(250);
    for line in s.lines() {
        for (x, ch) in line.chars().enumerate() {
            map.push(ch.try_into()?);
            width = width.max(x + 1);
        }
    }
    assert_eq!(map.len(), (map.len() / width) * width);
    Ok((width, map))
}

#[inline]
fn neighbours(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter_map(move |(dx, dy)| {
            if (dx, dy) != (0, 0) {
                Some((x + dx, y + dy))
            } else {
                None
            }
        })
}

fn get(map: &[Tile], width: usize, x: i32, y: i32) -> Option<Tile> {
    let height = (map.len() / width) as i32;
    let width = width as i32;
    if (0..width).contains(&x) && (0..height).contains(&y) {
        Some(map[(x + width * y) as usize])
    } else {
        None
    }
}

fn next_cells(map: &[Tile], width: usize) -> impl Iterator<Item = Tile> + use<'_> {
    map.iter().enumerate().map(move |(i, tile)| {
        let y = (i / width) as i32;
        let x = (i % width) as i32;
        let mut counts = [0; 3];
        for n in neighbours(x, y).filter_map(|(nx, ny)| get(map, width, nx, ny)) {
            counts[n as usize] += 1;
        }
        match (tile, counts) {
            (Open, [_, trees, _]) if trees >= 3 => Tree,
            (Open, _) => Open,
            (Tree, [_, _, lumberyards]) if lumberyards >= 3 => Lumberyard,
            (Tree, _) => Tree,
            (Lumberyard, [_, trees, lumberyard]) if lumberyard >= 1 && trees >= 1 => Lumberyard,
            (Lumberyard, _) => Open,
        }
    })
}

fn fingerprint(map: &[Tile], width: usize) -> Vec<u128> {
    let height = map.len() / width;
    let mut fp = Vec::with_capacity(height);
    for y in 0..height {
        let row = &map[(y * width)..((y + 1) * width)];
        fp.push(row.iter().fold(0, |acc, t| acc * 3 + (*t as u128)));
    }
    fp
}

fn resource_value(s: &str, turns: usize) -> Result<usize> {
    let (width, mut map) = parse(s)?;
    let mut buf = Vec::with_capacity(250);
    let mut cache = FxHashMap::default();
    let mut i = 0;
    while i < turns {
        buf.extend(next_cells(&map, width));
        mem::swap(&mut buf, &mut map);
        buf.clear();
        let k = fingerprint(&map, width);

        if let Some(previous) = cache.get(&k) {
            // The map was seen in this state earlier, so we can skip ahead
            let step = i - *previous;
            i += step;
        } else {
            cache.insert(k, i);
        }

        i += 1;
    }

    let mut counts = [0; 3];
    for t in map {
        counts[t as usize] += 1;
    }
    Ok(counts[1] * counts[2])
}

pub fn part_1(s: &str) -> Result<String> {
    Ok(resource_value(s, 10)?).map(|n| n.to_string())
}

pub fn part_2(s: &str) -> Result<String> {
    Ok(resource_value(s, 1000000000)?).map(|n| n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn p1_test() -> Result<()> {
        assert_eq!(resource_value(EX, 10)?, 1147);
        Ok(())
    }
}
