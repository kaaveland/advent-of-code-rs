use anyhow::Context;
use fxhash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Debug)]
struct Ducts {
    passable_tiles: FxHashSet<(i32, i32)>,
    locations: FxHashMap<(i32, i32), char>,
}

fn parse_map(s: &str) -> Ducts {
    let by_coords = s
        .trim_end()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| (x, y, ch)));
    let mut passable_tiles = FxHashSet::default();
    let mut locations = FxHashMap::default();
    for (x, y, ch) in by_coords {
        let pos = (x as i32, y as i32);
        if ch != '#' {
            passable_tiles.insert(pos);
            if ch != '.' {
                locations.insert(pos, ch);
            }
        }
    }
    Ducts {
        passable_tiles,
        locations,
    }
}

/// The distance from source to every destination != source
fn bfs(ducts: &Ducts, source: (i32, i32)) -> Option<FxHashMap<(i32, i32), usize>> {
    let mut work = VecDeque::new();
    work.push_back((0, source));
    let mut seen = FxHashSet::default();
    seen.insert(source);
    let mut distances = FxHashMap::default();

    while let Some((dist, pos)) = work.pop_front() {
        let (x, y) = pos;

        if dist > 0 && ducts.locations.contains_key(&pos) {
            distances.insert(pos, dist);
        } else {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let pos = (x + dx, y + dy);
                if ducts.passable_tiles.contains(&pos) && seen.insert(pos) {
                    work.push_back((dist + 1, pos));
                }
            }
        }
    }

    Some(distances)
}

fn adjacency_list(ducts: &Ducts) -> FxHashMap<(i32, i32), FxHashMap<(i32, i32), usize>> {
    let mut adj = FxHashMap::default();
    for loc in ducts.locations.keys() {
        if let Some(next) = bfs(ducts, *loc) {
            adj.insert(*loc, next);
        }
    }
    adj
}

fn djikstra(ducts: &Ducts, back_to_0: bool) -> Option<usize> {
    let adj = adjacency_list(ducts);
    let bit = |pos: &(i32, i32)| 1u32 << ducts.locations[pos].to_digit(10).unwrap();
    let start = *ducts.locations.iter().find(|(_, &c)| c == '0')?.0;
    let all = ducts.locations.keys().fold(0, |m, p| m | bit(p));

    let mut dist: FxHashMap<((i32, i32), u32), usize> = FxHashMap::default();
    let mut heap = BinaryHeap::new();
    dist.insert((start, bit(&start)), 0);
    heap.push(Reverse((0usize, start, bit(&start))));

    while let Some(Reverse((d, pos, mask))) = heap.pop() {
        if mask == all && (!back_to_0 || pos == start) {
            return Some(d);
        }
        // Skip stale heap entries
        if dist.get(&(pos, mask)).is_some_and(|&best| d > best) {
            continue;
        }
        for (&next, &w) in &adj[&pos] {
            let state = (next, mask | bit(&next));
            let nd = d + w;
            if dist.get(&state).is_none_or(|&best| nd < best) {
                dist.insert(state, nd);
                heap.push(Reverse((nd, state.0, state.1)));
            }
        }
    }
    None
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let ducts = parse_map(s);
    let len = djikstra(&ducts, false).context("No path")?;
    Ok(format!("{len}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let ducts = parse_map(s);
    let len = djikstra(&ducts, true).context("No path")?;
    Ok(format!("{len}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########
";

    #[test]
    fn bfs_origin() {
        let map = parse_map(EX);
        let from_0 = bfs(&map, (1, 1));
        assert!(from_0.is_some());
        let from_0 = from_0.unwrap();
        assert_eq!(from_0.len(), 2);
        assert!(from_0.contains_key(&(1, 3)));
        assert!(from_0.contains_key(&(3, 1)));
    }
}
