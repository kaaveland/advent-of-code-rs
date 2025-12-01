use anyhow::Result;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum Cave {
    Small(usize),
    Large(usize),
}

impl Cave {
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
    fn bitpattern(&self) -> u64 {
        assert!(self.idx() <= 63);
        1 << (self.idx() as u64)
    }
    fn is_in(&self, path: u64) -> bool {
        let pat = self.bitpattern();
        (pat & path) == pat
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

fn parse_graph(input: &str) -> Search<'_> {
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
            assert!(left <= 63 && right <= 63);
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

fn dfs_count(search: &Search, allow_dup: bool) -> usize {
    let mut completed_paths = 0;
    let mut stack = vec![(!allow_dup, &search.start, 0u64)];
    while let Some((has_dup, cave, path)) = stack.pop() {
        if cave == &search.end {
            completed_paths += 1;
        } else {
            let graph = &search.graph;
            for (would_dup, next_place) in graph[cave.idx()]
                .iter()
                .map(|cave| (cave.is_in(path) && cave.is_small(), cave))
            {
                if next_place == &search.start || (would_dup && has_dup) {
                    continue;
                } else {
                    stack.push((would_dup || has_dup, next_place, path | cave.bitpattern()));
                }
            }
        }
    }
    completed_paths
}

pub fn part_1(input: &str) -> Result<String> {
    let search = parse_graph(input);
    let paths = dfs_count(&search, false);
    Ok(format!("{paths}"))
}
pub fn part_2(input: &str) -> Result<String> {
    let search = parse_graph(input);
    let paths = dfs_count(&search, true);
    Ok(format!("{paths}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn small_example_p1() {
        let search = parse_graph(SMALL_EXAMPLE);
        assert_eq!(dfs_count(&search, false), 10);
    }

    #[test]
    fn small_example_p2() {
        let search = parse_graph(SMALL_EXAMPLE);
        assert_eq!(dfs_count(&search, true), 36);
    }

    #[test]
    fn large_example_p1() {
        let search = parse_graph(EXAMPLE);
        assert_eq!(dfs_count(&search, false), 226);
    }

    #[test]
    fn large_example_p2() {
        let search = parse_graph(EXAMPLE);
        assert_eq!(dfs_count(&search, true), 3509);
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
