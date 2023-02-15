use crate::intcode::{Output, Program};
use anyhow::{Context, Result};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;

const OPPOSITE: [i64; 5] = [
    0, // unused
    2, // opposite of 1/north
    1, // opposite of 2/south
    4, // opposite of 3/west
    3, // opposite of 4/east
];

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Tile {
    Empty,
    Oxygen,
    Wall,
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Reachable {
    cost: usize,
    kind: Tile,
}

fn bfs(prog: &Program) -> Result<HashMap<(i32, i32), Reachable>> {
    let mut work = VecDeque::from(vec![
        (prog.clone(), 1, 0, 0),
        (prog.clone(), 2, 0, 0),
        (prog.clone(), 3, 0, 0),
        (prog.clone(), 4, 0, 0),
    ]);
    // Map (x, y) => (steps to here, kind of thing here)
    let mut cache = HashMap::default();
    cache.insert(
        (0, 0),
        Reachable {
            cost: 0,
            kind: Tile::Empty,
        },
    );

    while let Some((mut droid, input, mut x, mut y)) = work.pop_front() {
        (x, y) = update_location(x, y, input);
        if cache.contains_key(&(x, y)) {
            continue;
        }
        droid.input(input);
        if let Output::Value(code) = droid.produce_output()? {
            // Location must be new or we would have continued
            if code == 0 {
                cache.insert(
                    (x, y),
                    Reachable {
                        cost: droid.read_inputs().len(),
                        kind: Tile::Wall,
                    },
                );
            } else if code == 2 {
                cache.insert(
                    (x, y),
                    Reachable {
                        cost: droid.read_inputs().len(),
                        kind: Tile::Oxygen,
                    },
                );
            } else if code == 1 {
                cache.insert(
                    (x, y),
                    Reachable {
                        cost: droid.read_inputs().len(),
                        kind: Tile::Empty,
                    },
                );
                for dir in 1..=4 {
                    if dir != OPPOSITE[input as usize]
                        && !cache.contains_key(&update_location(x, y, dir))
                    {
                        work.push_back((droid.clone(), dir, x, y));
                    }
                }
            }
        }
    }

    Ok(cache)
}

fn update_location(mut x: i32, mut y: i32, input: i64) -> (i32, i32) {
    if input == 1 {
        y -= 1;
    } else if input == 2 {
        y += 1;
    } else if input == 3 {
        x -= 1;
    } else if input == 4 {
        x += 1;
    }
    (x, y)
}

pub fn part_1(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Empty input")?)?;
    let map = bfs(&prog)?;
    map.values()
        .find(|reachable| reachable.kind == Tile::Oxygen)
        .copied()
        .map(|reachable| format!("{}", reachable.cost))
        .context("No oxygen found")
}

fn floodfill(map: &HashMap<(i32, i32), Reachable>, origin: (i32, i32)) -> usize {
    let mut visit: HashSet<_> = map
        .iter()
        .filter(|(_pos, reachable)| reachable.kind == Tile::Empty)
        .map(|(pos, _reachable)| pos)
        .copied()
        .collect();
    let (x, y) = origin;
    let mut max_cost = 0;
    let mut work = VecDeque::new();
    fn go_next(
        x: i32,
        y: i32,
        cost: usize,
        visit: &mut HashSet<(i32, i32)>,
        work: &mut VecDeque<(i32, i32, usize)>,
    ) {
        for dir in 1..=4 {
            let (nx, ny) = update_location(x, y, dir);
            if visit.contains(&(nx, ny)) {
                work.push_back((nx, ny, cost + 1));
            }
        }
    }
    go_next(x, y, 0, &mut visit, &mut work);

    while let Some((x, y, cost)) = work.pop_front() {
        max_cost = max_cost.max(cost);
        if visit.remove(&(x, y)) {
            // First arrival here
            go_next(x, y, cost, &mut visit, &mut work);
        }
    }

    max_cost
}

pub fn part_2(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Empty input")?)?;
    let map = bfs(&prog)?;
    let oxygen = map
        .iter()
        .find(|(_poss, reachable)| reachable.kind == Tile::Oxygen)
        .map(|(pos, _)| pos)
        .copied()
        .context("No oxygen found")?;
    let cost = floodfill(&map, oxygen);
    Ok(format!("{cost}"))
}
