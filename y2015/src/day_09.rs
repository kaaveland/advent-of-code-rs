use anyhow::Context;
use fxhash::{FxHashMap, FxHashSet};
use regex::Regex;

fn parse(s: &str) -> anyhow::Result<Vec<Vec<Option<usize>>>> {
    let re = Regex::new(r"^([^ ]+) to ([^ ]+) = ([0-9]+)")?;
    let mut interned = FxHashMap::default();
    for line in s.lines() {
        let m = re.captures(line).context("No match")?;
        let next_ix = interned.len();
        interned
            .entry(m.get(1).unwrap().as_str())
            .or_insert(next_ix);
        let next_ix = interned.len();
        interned
            .entry(m.get(2).unwrap().as_str())
            .or_insert(next_ix);
    }
    let mut distances = vec![vec![None; interned.len()]; interned.len()];
    for line in s.lines() {
        let m = re.captures(line).context("No match")?;
        let from = *interned.get(m.get(1).unwrap().as_str()).unwrap();
        let to = *interned.get(m.get(2).unwrap().as_str()).unwrap();
        let distance = m.get(3).unwrap().as_str().parse::<usize>()?;
        distances[from][to] = Some(distance);
        distances[to][from] = Some(distance);
    }
    Ok(distances)
}

fn all_distances(
    graph: &[Vec<Option<usize>>],
    visited: u32,
    current_location: usize,
    current_distance: usize,
    distances: &mut FxHashSet<usize>,
) {
    if visited.count_ones() as usize == graph.len() {
        distances.insert(current_distance);
    } else {
        for ix in 0..graph.len() {
            let bit = 1 << ix;
            // haven't been
            if visited & bit == 0 {
                if let Some(next_distance) = graph[current_location][ix] {
                    all_distances(
                        graph,
                        visited | bit,
                        ix,
                        current_distance + next_distance,
                        distances,
                    );
                }
            }
        }
    }
}

fn distances(graph: &[Vec<Option<usize>>]) -> FxHashSet<usize> {
    let mut found = FxHashSet::default();
    for start in 0..graph.len() {
        all_distances(graph, 1 << start, start, 0, &mut found);
    }
    found
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let graph = parse(s)?;
    let distances = distances(&graph);
    let min = distances
        .into_iter()
        .min()
        .context("Unable to find a tour")?;
    Ok(min.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let graph = parse(s)?;
    let distances = distances(&graph);
    let max = distances
        .into_iter()
        .max()
        .context("Unable to find a tour")?;
    Ok(max.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
    #[test]
    fn test_p1() {
        let graph = parse(EX).unwrap();
        let distances = distances(&graph);
        assert!(distances.contains(&605));
        assert!(distances.contains(&982));
        assert!(distances.contains(&659));
    }
}
