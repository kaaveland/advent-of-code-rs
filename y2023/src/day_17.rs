use anyhow::Result;
use fxhash::FxHashMap as Map;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

trait Grid {
    fn at(&self, x: i32, y: i32) -> Option<i8>;
    fn south_east_corner(&self, x: i32, y: i32) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct VecGrid {
    tiles: Vec<i8>,
    height: i32,
    width: i32,
}

impl Grid for VecGrid {
    fn at(&self, x: i32, y: i32) -> Option<i8> {
        if (0..self.height).contains(&y) && (0..self.width).contains(&x) {
            self.tiles.get((x + self.width * y) as usize).copied()
        } else {
            None
        }
    }

    fn south_east_corner(&self, x: i32, y: i32) -> bool {
        x == (self.width - 1) && y == (self.height - 1)
    }
}

impl VecGrid {
    fn new(input: &str) -> Self {
        let tiles = input
            .chars()
            .filter(|ch| ch.is_numeric())
            .map(|ch| (ch as u8 - b'0') as i8)
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

fn add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    fn dxdy(self) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0),
        }
    }
    fn possible(self) -> [Direction; 3] {
        use Direction::*;
        match self {
            North => [West, North, East],
            South => [West, South, East],
            West => [South, West, North],
            East => [South, East, North],
        }
    }
}
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
struct CrucibleState {
    pos: (i32, i32),
    heading: Direction,
    straight_length: u8,
}

fn generate_crucible_states(
    grid: &dyn Grid,
    cost: i32,
    state: CrucibleState,
    buffer: &mut Vec<(i32, CrucibleState)>,
) {
    for next_dir in state.heading.possible() {
        let straight_len = if next_dir == state.heading {
            state.straight_length + 1
        } else {
            1
        };
        if straight_len <= 3 {
            let next_pos = add(state.pos, next_dir.dxdy());
            if let Some(heatloss) = grid.at(next_pos.0, next_pos.1) {
                let next_cost = cost + heatloss as i32;
                let next_state = CrucibleState {
                    pos: next_pos,
                    heading: next_dir,
                    straight_length: straight_len,
                };
                buffer.push((next_cost, next_state))
            }
        }
    }
}

fn generate_ultra_crucible_states(
    grid: &dyn Grid,
    cost: i32,
    state: CrucibleState,
    buffer: &mut Vec<(i32, CrucibleState)>,
) {
    // This can't turn yet
    if (1..4).contains(&state.straight_length) {
        let next_pos = add(state.pos, state.heading.dxdy());
        if let Some(heatloss) = grid.at(next_pos.0, next_pos.1) {
            let next = CrucibleState {
                pos: next_pos,
                heading: state.heading,
                straight_length: state.straight_length + 1,
            };
            buffer.push((heatloss as i32 + cost, next));
        }
    } else {
        for next_dir in state.heading.possible() {
            let straight_len = if next_dir == state.heading {
                state.straight_length + 1
            } else {
                1
            };
            if straight_len <= 10 {
                let next_pos = add(state.pos, next_dir.dxdy());
                if let Some(heatloss) = grid.at(next_pos.0, next_pos.1) {
                    let next_cost = cost + heatloss as i32;
                    let next_state = CrucibleState {
                        pos: next_pos,
                        heading: next_dir,
                        straight_length: straight_len,
                    };
                    buffer.push((next_cost, next_state))
                }
            }
        }
    }
}

fn min_heat_loss(grid: &dyn Grid, is_ultra: bool) -> i32 {
    use Direction::*;
    let mut cache = Map::default();
    cache.insert(
        CrucibleState {
            pos: (0, 0),
            heading: East,
            straight_length: 0,
        },
        0,
    );
    // BinaryHeap is a maxheap, we want to minimize so we'll push Reverse(...)
    let mut work = BinaryHeap::new();
    for k in cache.keys() {
        work.push(Reverse((0, *k)));
    }
    let mut buffer = Vec::with_capacity(3);

    while let Some(Reverse((cost, state))) = work.pop() {
        if grid.south_east_corner(state.pos.0, state.pos.1)
            && (!is_ultra || (4..=10).contains(&state.straight_length))
        {
            return cost;
        }
        buffer.clear();
        if !is_ultra {
            generate_crucible_states(grid, cost, state, &mut buffer);
        } else {
            generate_ultra_crucible_states(grid, cost, state, &mut buffer);
        }
        for (next_cost, next_state) in buffer.iter().copied() {
            let prev_cost = cache.get(&next_state).unwrap_or(&i32::MAX);
            if next_cost < *prev_cost {
                cache.insert(next_state, next_cost);
                work.push(Reverse((next_cost, next_state)));
            }
        }
    }
    panic!("Unable to solve")
}

pub fn part_1(input: &str) -> Result<String> {
    let g = VecGrid::new(input);
    Ok(min_heat_loss(&g, false).to_string())
}
pub fn part_2(input: &str) -> Result<String> {
    let g = VecGrid::new(input);
    Ok(min_heat_loss(&g, true).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn test_example() {
        let g = VecGrid::new(EX);
        assert_eq!(min_heat_loss(&g, false), 102);
    }

    #[test]
    fn test_example_part_2() {
        let g = VecGrid::new(EX);
        assert_eq!(min_heat_loss(&g, true), 94);
    }
}
