use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum Cave {
    Small(usize),
    Large(usize),
}

impl Cave {
    fn is_large(&self) -> bool {
        matches!(self, Cave::Large(_))
    }
    fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }
    fn idx(&self) -> usize {
        match self {
            Cave::Small(idx) => *idx,
            Cave::Large(idx) => *idx,
        }
    }
    fn new(name: &str, label: usize) -> Cave {
        if name.to_ascii_lowercase() == name {
            Cave::Small(label)
        } else {
            Cave::Large(label)
        }
    }
}

type Graph = Vec<Vec<Cave>>;

#[derive(Debug, Eq, PartialEq)]
struct Search<'a> {
    labels: HashMap<&'a str, usize>,
    graph: Graph,
    start: Cave,
    end: Cave,
}

fn options<'a>(graph: &'a Graph, vtx: &'a Cave, path: &[&'a Cave]) -> Vec<&'a Cave> {
    graph[vtx.idx()]
        .iter()
        .filter(|&option: &&Cave| option.is_large() || !path.contains(&option))
        .collect_vec()
}

fn parse_graph(input: &str) -> Search {
    let mut labels: HashMap<_, usize> = HashMap::new();
    let mut idx = 0;
    let mut graph = Graph::new();
    for line in input.lines() {
        if let Some((vtx1, vtx2)) = line.split_once('-') {
            if !labels.contains_key(vtx1) {
                labels.insert(vtx1, idx);
                graph.push(vec![]);
                idx += 1;
            }
            if !labels.contains_key(vtx2) {
                labels.insert(vtx2, idx);
                graph.push(vec![]);
                idx += 1;
            }
            let left = *labels.get(vtx1).unwrap();
            let right = *labels.get(vtx2).unwrap();
            graph[left].push(Cave::new(vtx2, right));
            graph[right].push(Cave::new(vtx1, left));
        }
    }
    let start = *labels.get("start").unwrap();
    let end = *labels.get("end").unwrap();
    Search {
        labels,
        graph,
        start: Cave::new("start", start),
        end: Cave::new("end", end),
    }
}

fn dfs_count(search: &Search) -> usize {
    let mut completed_paths = 0;
    let mut stack = vec![vec![&search.start]];
    while let Some(path) = stack.pop() {
        // Can not fail, path can't be empty
        if let Some(&cave) = path.last() {
            if cave == &search.end {
                completed_paths += 1;
            } else {
                for next_place in options(&search.graph, cave, &path) {
                    let mut next_path = path.iter().copied().collect_vec();
                    next_path.push(next_place);
                    stack.push(next_path);
                }
            }
        }
    }
    completed_paths
}

fn options_dup_once<'a>(
    graph: &'a Graph,
    vtx: &'a Cave,
    path: &[&'a Cave],
) -> Vec<(bool, &'a Cave)> {
    graph[vtx.idx()]
        .iter()
        .map(|cave| (path.contains(&cave) && cave.is_small(), cave))
        .collect_vec()
}

fn dfs_count_dup_once(search: &Search) -> usize {
    let mut completed_paths = 0;
    let mut stack = vec![(false, vec![&search.start])];
    while let Some((has_dup, path)) = stack.pop() {
        if let Some(&cave) = path.last() {
            if cave == &search.end {
                completed_paths += 1;
            } else {
                for (would_dup, next_place) in options_dup_once(&search.graph, cave, &path) {
                    if next_place == &search.start || (would_dup && has_dup) {
                        continue;
                    } else {
                        let mut next_path = path.iter().copied().collect_vec();
                        next_path.push(next_place);
                        stack.push((would_dup || has_dup, next_path));
                    }
                }
            }
        }
    }
    completed_paths
}

pub fn part_1(input: &str) -> Result<()> {
    let search = parse_graph(input);
    let paths = dfs_count(&search);
    println!("{paths}");
    Ok(())
}
pub fn part_2(input: &str) -> Result<()> {
    let search = parse_graph(input);
    let paths = dfs_count_dup_once(&search);
    println!("{paths}");
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn small_example_p1() {
        let search = parse_graph(SMALL_EXAMPLE);
        assert_eq!(dfs_count(&search), 10);
    }

    #[test]
    fn small_example_p2() {
        let search = parse_graph(SMALL_EXAMPLE);
        assert_eq!(dfs_count_dup_once(&search), 36);
    }

    #[test]
    fn large_example_p1() {
        let search = parse_graph(EXAMPLE);
        assert_eq!(dfs_count(&search), 226);
    }

    #[test]
    fn large_example_p2() {
        let search = parse_graph(EXAMPLE);
        assert_eq!(dfs_count_dup_once(&search), 3509);
    }

    const SMALL_EXAMPLE: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EXAMPLE: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
}
