use anyhow::{anyhow, Context};
use fxhash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Pos = (i32, i32);

struct Maze {
    walls: FxHashSet<Pos>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn clockwise(&self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn counterclockwise(&self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
    fn step(&self, pos: Pos) -> Pos {
        let (x, y) = pos;
        use Direction::*;
        match self {
            North => (x, y - 1),
            South => (x, y + 1),
            East => (x + 1, y),
            West => (x - 1, y),
        }
    }
    fn backward(&self, pos: Pos) -> Pos {
        self.clockwise().clockwise().step(pos)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct State {
    cost: i64,
    pos: Pos,
    dir: Direction,
}

fn parse(input: &str) -> anyhow::Result<(Maze, Pos, Pos)> {
    let by_idx = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, ch)| (x as i32, y as i32, ch))
    });
    let start = by_idx
        .clone()
        .find_map(|(x, y, ch)| if ch == 'S' { Some((x, y)) } else { None })
        .context("Unable to find start loc")?;
    let end = by_idx
        .clone()
        .find_map(|(x, y, ch)| if ch == 'E' { Some((x, y)) } else { None })
        .context("Unable to find start loc")?;
    let walls = by_idx
        .filter_map(|(x, y, ch)| if ch == '#' { Some((x, y)) } else { None })
        .collect();
    Ok((Maze { walls }, start, end))
}

type Cache = FxHashMap<(Pos, Direction), i64>;

fn solve(maze: &Maze, start: Pos, end: Pos) -> anyhow::Result<(i64, Cache)> {
    let mut work = BinaryHeap::default();
    let mut cache: Cache = FxHashMap::default();
    work.push(Reverse(State {
        cost: 0,
        pos: start,
        dir: Direction::East,
    }));

    while let Some(Reverse(State { cost, pos, dir })) = work.pop() {
        if pos == end {
            return Ok((cost, cache));
        }
        let next = [
            State {
                dir,
                cost: cost + 1,
                pos: dir.step(pos),
            },
            State {
                pos,
                dir: dir.clockwise(),
                cost: cost + 1000,
            },
            State {
                pos,
                dir: dir.counterclockwise(),
                cost: cost + 1000,
            },
        ];
        for state in next {
            if &state.cost < cache.get(&(state.pos, state.dir)).unwrap_or(&i64::MAX)
                && !maze.walls.contains(&state.pos)
            {
                work.push(Reverse(state));
                cache.insert((state.pos, state.dir), state.cost);
            }
        }
    }
    Err(anyhow!("Unable to solve maze"))
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let (maze, start, end) = parse(input)?;
    let (cost, _) = solve(&maze, start, end)?;
    Ok(format!("{cost}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    use Direction::*;
    let (maze, start, end) = parse(input)?;
    // costs contain the cheapest way to reach the (pos, dir) pair
    let (cost, costs) = solve(&maze, start, end)?;
    // Locate the ending location / orientation
    let dir = [North, West, East, South]
        .into_iter()
        .find(|dir| costs.contains_key(&(end, *dir)))
        .context("Apparently didn't solve maze?")?;
    let mut visited = FxHashSet::default();
    // We can only push locations that are part of the solution here
    let mut work = vec![(cost, end, dir)];

    while let Some((cost, loc, dir)) = work.pop() {
        visited.insert(loc);
        let possible_parents = [
            (dir.backward(loc), dir, cost - 1),
            (loc, dir.clockwise(), cost - 1000),
            (loc, dir.counterclockwise(), cost - 1000),
        ];
        for (parent_loc, parent_dir, parent_cost) in possible_parents {
            if costs.get(&(parent_loc, parent_dir)) == Some(&parent_cost) {
                // Could've come from here
                work.push((parent_cost, parent_loc, parent_dir));
            }
        }
    }

    Ok(format!("{}", visited.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[test]
    fn test_solve_1() {
        let (maze, start, end) = parse(EXAMPLE).unwrap();
        let (cost, _) = solve(&maze, start, end).unwrap();
        assert_eq!(cost, 7036);
    }

    #[test]
    fn test_solve_2() {
        let ans = part_2(EXAMPLE).unwrap();
        assert_eq!(ans.as_str(), "45");
    }
}
