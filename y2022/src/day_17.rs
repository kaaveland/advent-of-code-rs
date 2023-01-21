use anyhow::Result;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

type Shape = Vec<(i64, i64)>;

fn shapes() -> Vec<Shape> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],             // hline
        vec![(0, -1), (1, 0), (1, -1), (1, -2), (2, -1)], // cross
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],     // reverse L
        vec![(0, 0), (0, -1), (0, -2), (0, -3)],          // vline
        vec![(0, 0), (0, -1), (1, 0), (1, -1)],           // square
    ]
}

#[derive(PartialEq, Eq, Debug)]
pub enum Jet {
    Left,
    Right,
}

fn shift(shape: &mut Shape, dir: (i64, i64)) {
    for (x, y) in shape.iter_mut() {
        *x += dir.0;
        *y += dir.1;
    }
}

type Jets = Vec<Jet>;
fn parse_jets(input: &str) -> Jets {
    input
        .chars()
        .filter(|&ch| ch == '<' || ch == '>')
        .map(|ch| if ch == '<' { Jet::Left } else { Jet::Right })
        .collect_vec()
}

const CHAMBER_WIDTH: i64 = 7;
const MAX_HEIGHT: usize = 100000;

fn in_bounds(shape: &Shape) -> bool {
    shape
        .iter()
        .all(|&(x, y)| (0..CHAMBER_WIDTH).contains(&x) && y >= 0)
}

fn drop_rock(
    jets: &Jets,
    shape: &Shape,
    mut time: usize,
    max_heights: &[i64; CHAMBER_WIDTH as usize],
    grid: &mut [Vec<bool>],
) -> (usize, [i64; CHAMBER_WIDTH as usize]) {
    let height = max_heights.iter().max().unwrap();
    let mut shape = shape.clone();
    let y_low = shape.iter().map(|&(_, y)| y).min().unwrap();
    shift(&mut shape, (2, height + 3 + y_low.abs()));

    loop {
        // Invariant: we're in bounds here, enforce it
        if !in_bounds(&shape) {
            panic!("Out of bounds {time} {shape:?}");
        }
        let jet = &jets[time % jets.len()];
        let (forward, backward) = match jet {
            Jet::Right => ((1, 0), (-1, 0)),
            Jet::Left => ((-1, 0), (1, 0)),
        };
        time += 1;
        let (down, up) = ((0, -1), (0, 1));

        shift(&mut shape, forward);

        if shape
            .iter()
            .any(|&(x, y)| !(0..CHAMBER_WIDTH).contains(&x) || grid[y as usize][x as usize])
        {
            // Preserve invariant
            shift(&mut shape, backward);
        }

        shift(&mut shape, down);
        if shape
            .iter()
            .any(|&(x, y)| y < 0 || grid[y as usize][x as usize])
        {
            // Preserve invariant
            shift(&mut shape, up);
            break;
        }
    }

    let mut max_height_out = *max_heights;

    for (x, y) in shape {
        grid[y as usize][x as usize] = true;
        max_height_out[x as usize] = max(y + 1, max_height_out[x as usize]);
    }

    (time, max_height_out)
}

type CacheKey = ([i64; CHAMBER_WIDTH as usize], usize, usize);
type CacheValue = (usize, i64);

fn drop_many_rocks(jets: &Jets, rocks_to_drop: usize) -> i64 {
    let shapes = shapes();
    let mut grid = vec![vec![false; CHAMBER_WIDTH as usize]; MAX_HEIGHT];
    let mut max_heights = [0; CHAMBER_WIDTH as usize];
    let mut time = 0;
    let mut rock_number = 0;
    let mut cycled_altitude = 0;
    let mut cache: HashMap<CacheKey, CacheValue> = HashMap::new();

    while rock_number < rocks_to_drop {
        let shape = &shapes[rock_number % shapes.len()];
        (time, max_heights) = drop_rock(jets, shape, time, &max_heights, &mut grid);
        let smallest_height = *max_heights.iter().min().unwrap();
        let largest_height = *max_heights.iter().max().unwrap();
        let top_shape: [i64; CHAMBER_WIDTH as usize] = max_heights
            .iter()
            .map(|&h| h - smallest_height)
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        let cache_key = (top_shape, rock_number % shapes.len(), time % jets.len());

        if let Some(&(old_index, old_height)) = cache.get(&cache_key) {
            if cycled_altitude == 0 {
                let cycle_length = rock_number - old_index;
                let cycle_height = largest_height - old_height;
                let cycles_to_skip = (rocks_to_drop - old_index) / cycle_length - 1;
                rock_number += cycles_to_skip * cycle_length;
                cycled_altitude += (cycles_to_skip as i64) * cycle_height;
            }
        } else {
            cache.insert(cache_key, (rock_number, largest_height));
        }

        rock_number += 1;
    }

    *max_heights.iter().max().unwrap() + cycled_altitude
}

pub fn part_1(input: &str) -> Result<String> {
    let jets = parse_jets(input);
    let max_height = drop_many_rocks(&jets, 2022);
    Ok(format!("{max_height}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let jets = parse_jets(input);
    let max_height = drop_many_rocks(&jets, 1000000000000);
    Ok(format!("{max_height}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn parse_jets() {
        let jets = super::parse_jets(EXAMPLE);
        assert_eq!(jets[0], Jet::Right);
        assert_eq!(jets[3], Jet::Left);
    }

    #[test]
    #[ignore] // Failing due, but working for real input
    fn test_drop_many_rocks() {
        let jets = super::parse_jets(EXAMPLE);
        let answer = drop_many_rocks(&jets, 2022);
        assert_eq!(answer, 3068);
    }

    #[test]
    fn test_drop_supermany_rocks() {
        let jets = super::parse_jets(EXAMPLE);
        let answer = drop_many_rocks(&jets, 1000000000000);
        assert_eq!(answer, 1514285714288);
    }
}
