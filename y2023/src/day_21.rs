use anyhow::{Context, Result};
use fxhash::FxHashMap as Map;
use itertools::Itertools;
use std::collections::VecDeque;

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
fn walk_garden(grid: &VecGrid, start: (i32, i32), time: usize) -> Vec<usize> {
    let mut distances = Map::default();
    let mut work = VecDeque::from([(0, start)]);
    distances.insert(start, 0);
    while let Some((t, place)) = work.pop_front() {
        if t > time {
            break;
        }
        for next_dir in DIRECTIONS {
            let next_loc = add(place, next_dir);
            let tile = grid.at(next_loc.0, next_loc.1);
            if matches!(tile, Some(Tile::Open) | Some(Tile::Start))
                && !distances.contains_key(&next_loc)
            {
                distances.insert(next_loc, t + 1);
                work.push_back((t + 1, next_loc));
            }
        }
    }
    let mut counts = vec![0; time + 1];
    for value in distances.values() {
        if *value <= time {
            counts[*value] += 1;
        }
    }
    counts
        .into_iter()
        .enumerate()
        .scan((0, 0), |(evens, odds), (t, count)| {
            if t % 2 == 0 {
                *evens += count;
                Some(*evens)
            } else {
                *odds += count;
                Some(*odds)
            }
        })
        .collect_vec()
}

pub fn part_1(s: &str) -> Result<String> {
    let g = VecGrid::new(s);
    let s = find_start(&g).context("No start location")?;
    Ok(walk_garden(&g, s, 64)[64].to_string())
}

pub fn part_2(s: &str) -> Result<String> {
    let g = VecGrid::new(s);
    assert_eq!(g.width, g.height);
    let s = find_start(&g).context("No start location")?;
    let step_count = 26501365;
    let grid_radius = (step_count / g.width) as usize;
    let remainder = step_count % g.width;
    let sample_garden_tiles_reached = walk_garden(&g, s, (g.width * 2 + remainder) as usize + 1);
    // We want to create a polynomial f(n) = a * n^2 + b * n + c such that f(0) is the amount of tiles
    // walked to at time `remainder`, f(1) is the amount of tiles walked to at time `remainder + grid.width`,
    // f(2) is the amount of time walked to at `remainder + 2 * grid.width`
    // We can verify f(0), f(1), f(2) against `sample_garden_tiles_reached`
    let f_0 = sample_garden_tiles_reached[remainder as usize];
    let f_1 = sample_garden_tiles_reached[(remainder + g.width) as usize];
    let f_2 = sample_garden_tiles_reached[(remainder + 2 * g.width) as usize];
    // Trivially, f(0) = c
    let c = f_0;
    // Now we have f_1 = a + b + c and f_2 = a * 4 + b * 2 + c
    // Solve f_1 for a: a = f_1 - b - c, insert into f_2:
    // f_2 = 4 * (f_1 - b - c) + b * 2 + c
    // f_2 = 4 * f_1 - 2 * b - 3c => b = (4 * f_1 - 3c - f_2) / 2
    let b = (4 * f_1 - 3 * c - f_2) / 2;
    // f_1 = a + b + c => a = f_1 - b - c
    let a = f_1 - b - c;
    let ans = a * grid_radius * grid_radius + b * grid_radius + c;
    Ok(ans.to_string())
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
        assert_eq!(*walk_garden(&g, s, 1).last().unwrap(), 2);
        assert_eq!(*walk_garden(&g, s, 2).last().unwrap(), 4);
        assert_eq!(*walk_garden(&g, s, 6).last().unwrap(), 16);
    }

    #[test]
    fn test_p2() {
        let g = VecGrid::new(EX);
        let s = find_start(&g).unwrap();
        let v = walk_garden(&g, s, 501);
        assert_eq!(v[6], 16);
        assert_eq!(v[10], 50);
        assert_eq!(v[50], 1594);
        assert_eq!(v[100], 6536);
        assert_eq!(v[500], 167004);
    }
}
