use anyhow::Context;
use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::*;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

type PathCache = FxHashMap<((i32, i32), usize), ((i32, i32), usize)>;
type Segment = Vec<((i32, i32), usize)>;

struct Grid {
    height: i32,
    width: i32,
    obstacles: FxHashSet<(i32, i32)>,
}

fn find_in_grid(grid: &str, ch: char) -> impl Iterator<Item = (i32, i32)> + '_ {
    grid.lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == ch {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
}

fn positions_in_grid<'a>(
    cache: &'a PathCache,
    grid: &'a Grid,
) -> impl Iterator<Item = (i32, i32)> + 'a {
    cache
        .keys()
        .map(|(pos, _)| *pos)
        .filter(|pos| grid.contains(pos))
}

impl Grid {
    fn contains(&self, pos: &(i32, i32)) -> bool {
        let (x, y) = pos;
        (0..self.height).contains(y) && (0..self.width).contains(x)
    }
    fn blocks(&self, pos: (i32, i32)) -> bool {
        self.obstacles.contains(&pos)
    }
    fn from(input: &str) -> Self {
        let height = input.lines().filter(|line| !line.is_empty()).count() as i32;
        let width = input.lines().next().map(|line| line.len()).unwrap_or(0) as i32;
        let obstacles = find_in_grid(input, '#').collect();
        Self {
            height,
            width,
            obstacles,
        }
    }
}
fn resolve_segment(cache: &mut PathCache, segment: &Segment, end_pos: (i32, i32), end_dir: usize) {
    for (pos, dir) in segment {
        cache.insert((*pos, *dir), (end_pos, end_dir));
    }
}

fn build_path_cache(grid: &Grid, mut pos: (i32, i32)) -> PathCache {
    let mut cache = PathCache::default();
    let mut segment = Segment::new();
    let mut dir = 0;

    loop {
        segment.push((pos, dir));
        let (dx, dy) = DIRS[dir];
        let (x, y) = (pos.0 + dx, pos.1 + dy);
        if grid.contains(&(x, y)) {
            if !grid.blocks((x, y)) {
                pos = (x, y);
            } else {
                dir = (dir + 1) % 4;
                resolve_segment(&mut cache, &segment, pos, dir);
                segment.clear();
            }
        } else {
            resolve_segment(&mut cache, &segment, (x, y), dir);
            return cache;
        }
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let grid = Grid::from(input);
    let origin = find_in_grid(input, '^')
        .next()
        .context("Origin not found")?;
    let cache = build_path_cache(&grid, origin);
    let unique_positions: FxHashSet<_> = positions_in_grid(&cache, &grid).collect();
    Ok(format!("{}", unique_positions.len()))
}

fn would_loop(grid: &Grid, mut pos: (i32, i32), cache: &PathCache, obs: (i32, i32)) -> bool {
    let mut dir = 0;
    let mut visited = FxHashSet::default();
    loop {
        if visited.insert((pos, dir)) {
            if cache.contains_key(&(pos, dir)) && (obs.0 != pos.0 && obs.1 != pos.1) {
                let (next_pos, next_dir) = cache.get(&(pos, dir)).unwrap();
                pos = *next_pos;
                dir = *next_dir;
            } else {
                let (dx, dy) = DIRS[dir];
                let (x, y) = (pos.0 + dx, pos.1 + dy);
                if !grid.contains(&(x, y)) {
                    return false;
                } else if grid.blocks((x, y)) || (x, y) == obs {
                    dir = (dir + 1) % 4;
                } else {
                    pos = (x, y);
                }
            }
        } else {
            return true;
        }
    }
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let grid = Grid::from(input);
    let origin = find_in_grid(input, '^')
        .next()
        .context("Origin not found")?;
    let cache = build_path_cache(&grid, origin);
    let unique_positions: FxHashSet<_> = positions_in_grid(&cache, &grid)
        .filter(|pos| *pos != origin)
        .collect();
    let p2 = unique_positions
        .par_iter()
        .filter(|obs| would_loop(&grid, origin, &cache, **obs))
        .count();
    Ok(format!("{}", p2))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_p1() {
        assert_eq!(part_1(EXAMPLE).unwrap().as_str(), "41");
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(EXAMPLE).unwrap().as_str(), "6");
    }
}
