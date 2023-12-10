use anyhow::{Context, Result};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as Set;
use itertools::Itertools;
use std::collections::VecDeque;

type Coord2 = [i32; 2];
const NORTH: Coord2 = [0, -1];
const WEST: Coord2 = [-1, 0];
const SOUTH: Coord2 = [0, 1];
const EAST: Coord2 = [1, 0];
const CONNECTORS: [(char, [Coord2; 2]); 6] = [
    ('|', [NORTH, SOUTH]),
    ('-', [WEST, EAST]),
    ('L', [NORTH, EAST]),
    ('J', [NORTH, WEST]),
    ('7', [SOUTH, WEST]),
    ('F', [SOUTH, EAST]),
];

type Pipes = HashMap<Coord2, Vec<Coord2>>;

fn add_coords(lhs: Coord2, rhs: Coord2) -> Coord2 {
    [lhs[0] + rhs[0], lhs[1] + rhs[1]]
}

fn parse_pipes(input: &str) -> Result<(Coord2, Pipes)> {
    let places_char = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ([x as i32, y as i32], ch))
        })
        .filter(|(_, ch)| *ch != '.')
        .collect_vec();

    let start = places_char
        .iter()
        .find(|(_, ch)| *ch == 'S')
        .map(|(xy, _)| *xy)
        .context("Unable to find start tile")?;

    let conns: HashMap<_, _> = CONNECTORS.iter().copied().collect();

    let place_by_connections = places_char
        .into_iter()
        .filter(|(_, ch)| conns.contains_key(ch))
        .map(|(xy, ch)| {
            // Safe because `conns.contains_key(ch)`
            let [dxdy1, dxdy2] = *conns.get(&ch).unwrap();
            (xy, vec![add_coords(xy, dxdy1), add_coords(xy, dxdy2)])
        })
        .collect();

    Ok((start, place_by_connections))
}
fn connects_to(place: Coord2, pipes: &Pipes) -> Vec<Coord2> {
    pipes
        .iter()
        .filter(|(_, conns)| conns.contains(&place))
        .map(|(xy, _)| *xy)
        .collect()
}

pub fn part_1(input: &str) -> Result<String> {
    let (start, pipes) = parse_pipes(input)?;

    let mut visited = Set::default();
    visited.insert(start);
    let mut work = VecDeque::new();
    work.push_front((0, start));
    let mut max_distance = 0;

    while let Some((time, place)) = work.pop_front() {
        max_distance = max_distance.max(time);
        for next in connects_to(place, &pipes) {
            if visited.insert(next) {
                work.push_back((time + 1, next));
            }
        }
    }

    Ok(max_distance.to_string())
}
