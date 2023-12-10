use anyhow::{Context, Result};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as Set;
use itertools::Itertools;

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
    let places_char = map_iterator(input)
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

fn visit_graph(start: Coord2, pipes: &Pipes) -> (Set<Coord2>, i32) {
    let path = graph_path(start, pipes);
    let path_len = path.len();
    (path.into_iter().collect(), (path_len / 2) as i32)
}

fn graph_path(start: Coord2, pipes: &Pipes) -> Vec<Coord2> {
    // We have a nice invariant; since all the pipes have 2 connections, we know that we can always
    // add one of the connections that isn't already in `path`, until we can't anymore, at which time
    // we have formed the loop
    let mut path = vec![start, connects_to(start, pipes)[0]];
    let mut place = path.last().copied().unwrap();
    let mut cache: Set<_> = path.iter().copied().collect();
    while let Some(possible) = pipes
        .get(&place)
        .unwrap_or(&vec![])
        .iter()
        .find(|n| !cache.contains(*n))
    {
        place = *possible;
        path.push(place);
        cache.insert(place);
    }
    path
}

fn map_iterator(input: &str) -> impl Iterator<Item = (Coord2, char)> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ([x as i32, y as i32], ch))
        })
}
pub fn part_1(input: &str) -> Result<String> {
    let (start, pipes) = parse_pipes(input)?;
    Ok(visit_graph(start, &pipes).1.to_string())
}

fn shoelace_area(polygon: &[Coord2]) -> i32 {
    let mut s1 = 0;
    let mut s2 = 0;

    for i in 0..polygon.len() {
        let next = (i + 1) % polygon.len();

        s1 += polygon[i][0] * polygon[next][1];
        s2 += polygon[i][1] * polygon[next][0];
    }

    (s2 - s1).abs() / 2
}

pub fn part_2(input: &str) -> Result<String> {
    let (start, pipes) = parse_pipes(input)?;
    // In order vertices of the path
    let the_loop = graph_path(start, &pipes);
    let area_including_loop = shoelace_area(&the_loop);
    let interior_points_by_picks_theorem = area_including_loop - ((the_loop.len() / 2) as i32 - 1);
    Ok(interior_points_by_picks_theorem.to_string())
}
