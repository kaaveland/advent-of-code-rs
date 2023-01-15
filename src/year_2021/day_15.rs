use anyhow::{Context, Result};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Location = (i32, i32);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Chiton {
    width: i32,
    height: i32,
    risk: Vec<Vec<u8>>,
}

fn parse(input: &str) -> Result<Chiton> {
    let width = input.lines().next().context("Empty input")?.chars().count() as i32;
    let mut risk = Vec::with_capacity(width as usize);
    for line in input.lines().filter(|line| !line.is_empty()) {
        risk.push(line.as_bytes().iter().map(|ch| ch - b'0').collect_vec());
    }

    Ok(Chiton {
        width,
        height: risk.len() as i32,
        risk,
    })
}

const DIRECTIONS: [Location; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn shortest_path(chiton: &Chiton, start_loc: Location, end_loc: Location) -> Result<i32> {
    let mut tasks = BinaryHeap::new();
    let mut costs = vec![vec![i32::MAX; chiton.width as usize]; chiton.height as usize];
    let (x, y) = start_loc;
    costs[y as usize][x as usize] = 0;
    tasks.push(Reverse((0, start_loc)));

    while let Some(Reverse((cost, location))) = tasks.pop() {
        let (x, y) = location;
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if (0..chiton.width).contains(&nx) && (0..chiton.height).contains(&ny) {
                let next = (nx, ny);
                let next_cost = chiton.risk[ny as usize][nx as usize];
                let opt = (next_cost as i32) + cost;
                let prev_cost = costs[ny as usize][nx as usize];
                let node = (opt, next);
                if opt < prev_cost {
                    costs[ny as usize][nx as usize] = opt;
                    tasks.push(Reverse(node));
                }
            }
        }
    }

    let (ex, ey) = end_loc;
    Ok(costs[ey as usize][ex as usize])
}

pub fn part_1(input: &str) -> Result<String> {
    let chiton = parse(input)?;
    shortest_path(&chiton, (0, 0), (chiton.width - 1, chiton.height - 1))
        .map(|cost| format!("{cost}"))
}

const EXTRA_COST: [[u8; 5]; 5] = [
    [0, 1, 2, 3, 4],
    [1, 2, 3, 4, 5],
    [2, 3, 4, 5, 6],
    [3, 4, 5, 6, 7],
    [4, 5, 6, 7, 8],
];
pub fn part_2(input: &str) -> Result<String> {
    let chiton = parse(input)?;
    let real_width = 5 * chiton.width;
    let real_height = 5 * chiton.height;

    let mut real_map = vec![vec![0; real_width as usize]; real_height as usize];

    for tile_x in 0..5 {
        for tile_y in 0..5 {
            let added_risk = EXTRA_COST[tile_x as usize][tile_y as usize];
            for x in 0..chiton.width {
                for y in 0..chiton.height {
                    let risk = chiton.risk[y as usize][x as usize];
                    let next_risk = (risk - 1 + added_risk) % 9 + 1;
                    let real_x = (x + tile_x * chiton.width) as usize;
                    let real_y = (y + tile_y * chiton.height) as usize;
                    real_map[real_y][real_x] = next_risk;
                }
            }
        }
    }

    let real_chiton = Chiton {
        risk: real_map,
        height: real_height,
        width: real_width,
    };
    shortest_path(
        &real_chiton,
        (0, 0),
        (real_chiton.width - 1, real_chiton.height - 1),
    )
    .map(|cost| format!("{cost}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let chiton = parse(EXAMPLE).unwrap();
        let cost = shortest_path(&chiton, (0, 0), (chiton.width - 1, chiton.height - 1)).unwrap();
        assert_eq!(cost, 40);
    }

    #[test]
    fn test_example_part2() {
        let answer = part_2(EXAMPLE).unwrap();
        assert_eq!(answer.as_str(), "315");
    }

    const EXAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
}
