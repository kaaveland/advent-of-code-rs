use anyhow::{Context, Result};
use fxhash::FxHashMap as Map;
use fxhash::FxHashSet as Set;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    West,
    East,
    South,
}
const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::East,
    Direction::South,
];

impl Direction {
    fn dxdy(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (0, -1),
            West => (-1, 0),
            East => (1, 0),
            South => (0, 1),
        }
    }
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct VecGrid {
    tiles: Vec<Tile>,
    height: i32,
    width: i32,
    slippery: bool,
}

impl VecGrid {
    fn new(input: &str, slippery: bool) -> Self {
        use Direction::*;
        use Tile::*;
        let tiles = input
            .chars()
            .filter_map(|ch| match ch {
                '.' => Some(Path),
                '#' => Some(Forest),
                '>' => Some(Slope(East)),
                '<' => Some(Slope(West)),
                '^' => Some(Slope(North)),
                'v' => Some(Slope(South)),
                _ => None,
            })
            .collect_vec();
        let height = input.lines().filter(|l| !l.is_empty()).count() as i32;
        let width = tiles.len() as i32 / height;
        assert_eq!(height * width, tiles.len() as i32);
        Self {
            tiles,
            height,
            width,
            slippery,
        }
    }
    fn at(&self, x: i32, y: i32) -> Option<Tile> {
        if (0..self.height).contains(&y) && (0..self.width).contains(&x) {
            self.tiles.get((x + self.width * y) as usize).copied()
        } else {
            None
        }
    }
    fn next_possible(&self, xy: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
        use Tile::*;
        DIRECTIONS
            .iter()
            .filter(move |&dir| match self.at(xy.0, xy.1) {
                Some(Slope(next)) => !self.slippery || *dir == next,
                _ => true,
            })
            .map(move |dir| add(xy, dir.dxdy()))
            .filter(|next| self.at(next.0, next.1).map(|tile| tile != Forest) == Some(true))
    }
}

fn add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn find_start(grid: &VecGrid) -> Option<(i32, i32)> {
    (0..grid.width)
        .find(|x| grid.at(*x, 0) == Some(Tile::Path))
        .map(|x| (x, 0))
}

fn find_end(grid: &VecGrid) -> Option<(i32, i32)> {
    (0..grid.width)
        .find(|x| grid.at(*x, grid.height - 1) == Some(Tile::Path))
        .map(|x| (x, grid.height - 1))
}

fn find_junctions(grid: &VecGrid) -> Map<(i32, i32), usize> {
    find_start(grid)
        .into_iter()
        .chain(
            (0..grid.width)
                .cartesian_product(0..grid.height)
                .filter(|(x, y)| grid.at(*x, *y) != Some(Tile::Forest))
                .filter(|(x, y)| grid.next_possible((*x, *y)).count() > 2)
                .chain(find_end(grid)),
        )
        .enumerate()
        .map(|(i, loc)| (loc, i + 1))
        .collect()
}

fn bfs(grid: &VecGrid, start: (i32, i32), junctions: &Map<(i32, i32), usize>) -> Map<usize, usize> {
    let mut cache = Set::default();
    let mut work = VecDeque::new();
    cache.insert(start);
    work.push_back((0, start));
    let mut distances = Map::default();

    while let Some((dist, loc)) = work.pop_back() {
        if loc != start && junctions.contains_key(&loc) {
            let junction_name = junctions.get(&loc).unwrap();
            distances.insert(*junction_name, dist);
        } else {
            for n in grid.next_possible(loc) {
                if cache.insert(n) {
                    work.push_back((dist + 1, n));
                }
            }
        }
    }
    distances
}

fn make_graph(grid: &VecGrid) -> Vec<Vec<(usize, usize)>> {
    let junctions = find_junctions(grid);
    let mut graph = vec![vec![]; junctions.len() + 1];
    for (source, source_name) in junctions.iter() {
        for (dest_name, distance) in bfs(grid, *source, &junctions) {
            graph[*source_name].push((dest_name, distance));
        }
    }
    graph
}
fn longest_path_using_graph(graph: &[Vec<(usize, usize)>]) -> usize {
    let start = 1usize;
    let end = graph.len() - 1;
    let mut work = vec![(start, 1usize, 0)];
    let mut max_dist = 0;
    while let Some((loc, visited, len)) = work.pop() {
        if loc == end {
            max_dist = max_dist.max(len);
        } else {
            work.extend(
                graph[loc]
                    .iter()
                    .filter(|(next, dist)| *dist != 0 && (1 << next) & visited != (1 << next))
                    .map(|(dest, dist)| (*dest, visited | (1 << dest), len + *dist)),
            );
        }
    }
    max_dist
}

fn longest_path(grid: &VecGrid) -> Option<usize> {
    let start = find_start(grid)?;
    let end = find_end(grid)?;
    let mut cache = Set::default();
    cache.insert(start);
    let mut work = vec![(start, cache)];
    let mut max_path = 0;
    while let Some((loc, mut cache)) = work.pop() {
        if loc == end {
            max_path = max_path.max(cache.len() - 1);
        } else {
            let next = grid
                .next_possible(loc)
                .filter(|xy| !cache.contains(xy))
                .collect_vec();
            if next.len() == 1 {
                cache.insert(next[0]);
                work.push((next[0], cache));
            } else {
                for n in next {
                    let mut copy = cache.clone();
                    copy.insert(n);
                    work.push((n, copy));
                }
            }
        }
    }
    Some(max_path)
}

pub fn part_1(s: &str) -> Result<String> {
    let v = VecGrid::new(s, true);
    longest_path(&v)
        .context("Unable to find start/end")
        .map(|n| n.to_string())
}

pub fn part_2(s: &str) -> Result<String> {
    let v = VecGrid::new(s, false);
    let map = make_graph(&v);
    Ok(longest_path_using_graph(&map).to_string())
}

#[cfg(test)]
mod tests {
    const EX: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";
    use super::*;
    #[test]
    fn test_longest_path() {
        let g = VecGrid::new(EX, true);
        assert_eq!(longest_path(&g), Some(94));
    }

    #[test]
    fn test_longest_nonslippery_path() {
        let g = VecGrid::new(EX, false);
        assert_eq!(longest_path(&g), Some(154));
    }

    #[test]
    fn test_longest_path_using_graph() {
        let v = VecGrid::new(EX, false);
        let map = make_graph(&v);
        assert_eq!(longest_path_using_graph(&map), 154);
    }

    #[test]
    fn test_next_possible() {
        let v = VecGrid::new(".>.\nv..\n...", true);
        assert_eq!(v.next_possible((0, 0)).collect_vec(), vec![(1, 0), (0, 1)]);
        assert_eq!(v.next_possible((1, 0)).collect_vec(), vec![(2, 0)]);
        assert_eq!(v.next_possible((0, 1)).collect_vec(), vec![(0, 2)]);
    }
}
