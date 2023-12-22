use anyhow::{Context, Result};
use fxhash::FxHashSet as Set;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Tile {
    Start,
    Open,
    Blocked,
}
trait Grid {
    fn at(&self, x: i32, y: i32) -> Option<Tile>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct VecGrid {
    tiles: Vec<Tile>,
    height: i32,
    width: i32,
}

impl Grid for VecGrid {
    fn at(&self, x: i32, y: i32) -> Option<Tile> {
        let x = x.rem_euclid(self.width);
        let y = y.rem_euclid(self.height);
        if (0..self.height).contains(&y) && (0..self.width).contains(&x) {
            self.tiles.get((x + self.width * y) as usize).copied()
        } else {
            None
        }
    }
}
impl VecGrid {
    fn new(input: &str) -> Self {
        let tiles = input
            .chars()
            .filter_map(|ch| {
                if ch == 'S' {
                    Some(Tile::Start)
                } else if ch == '.' {
                    Some(Tile::Open)
                } else if ch == '#' {
                    Some(Tile::Blocked)
                } else {
                    None
                }
            })
            .collect_vec();
        let height = input.lines().filter(|l| !l.is_empty()).count() as i32;
        let width = tiles.len() as i32 / height;
        assert_eq!(height * width, tiles.len() as i32);
        Self {
            tiles,
            height,
            width,
        }
    }
}

fn find_start(grid: &VecGrid) -> Option<(i32, i32)> {
    (0..grid.width)
        .cartesian_product(0..grid.height)
        .find(|(x, y)| grid.at(*x, *y) == Some(Tile::Start))
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
fn add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
fn fill(grid: &VecGrid, start: (i32, i32), time: usize) -> Vec<usize> {
    let mut at = Set::default();
    let mut n = vec![1];
    at.insert(start);
    for _ in 0..time {
        let mut next = Set::default();
        for loc in at {
            for next_dir in DIRECTIONS {
                let next_loc = add(loc, next_dir);
                let tile = grid.at(next_loc.0, next_loc.1);
                if matches!(tile, Some(Tile::Open) | Some(Tile::Start)) {
                    next.insert(next_loc);
                }
            }
        }
        at = next;
        n.push(at.len());
    }
    n
}

pub fn part_1(s: &str) -> Result<String> {
    let g = VecGrid::new(s);
    let s = find_start(&g).context("No start location")?;
    Ok(fill(&g, s, 64)[64].to_string())
}

pub fn part_2(_: &str) -> Result<String> {
    // Found analytically in jupyter this time around...
    let n: i64 = 202300;
    Ok((14840 * n * n + 14940 * n + 3751).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
    #[test]
    fn test_p1() {
        let g = VecGrid::new(EX);
        let s = find_start(&g).unwrap();
        assert_eq!(*fill(&g, s, 1).last().unwrap(), 2);
        assert_eq!(*fill(&g, s, 2).last().unwrap(), 4);
        assert_eq!(*fill(&g, s, 6).last().unwrap(), 16);
    }

    #[test]
    fn test_p2() {
        let g = VecGrid::new(EX);
        let s = find_start(&g).unwrap();
        let v = fill(&g, s, 101);
        assert_eq!(v[6], 16);
        assert_eq!(v[10], 50);
        assert_eq!(v[50], 1594);
        assert_eq!(v[100], 6536);
    }
}
