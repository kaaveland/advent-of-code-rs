use anyhow::Context;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

fn parse(inp: &str) -> anyhow::Result<FxHashMap<&str, FxHashSet<&str>>> {
    let mut m: FxHashMap<_, FxHashSet<_>> = FxHashMap::default();
    for l in inp.lines() {
        let (a, b) = l.split_once('-').context("Bad input")?;
        m.entry(a).or_default().insert(b);
        m.entry(b).or_default().insert(a);
    }
    Ok(m)
}

fn max_clique<'a>(
    nodes: &FxHashSet<&'a str>,
    graph: &FxHashMap<&'a str, FxHashSet<&'a str>>,
) -> FxHashSet<&'a str> {
    if nodes.is_empty() || nodes.len() == 1 {
        nodes.clone()
    } else {
        let mut temp = nodes.clone();
        let node = *temp.iter().next().unwrap();
        temp.remove(node);
        let without_node = max_clique(&temp, graph);
        let intersection: FxHashSet<_> = graph
            .get(node)
            .unwrap()
            .clone()
            .intersection(&temp)
            .copied()
            .collect();
        let mut with_node = max_clique(&intersection, graph);
        with_node.insert(node);
        if without_node.len() > with_node.len() {
            without_node
        } else {
            with_node
        }
    }
}

pub fn part_1(inp: &str) -> anyhow::Result<String> {
    let graph = parse(inp)?;
    let mut found = FxHashSet::default();
    for (&k, v) in graph.iter() {
        if k.starts_with("t") {
            for c in v.iter().combinations(2) {
                let (a, b) = (*c[0], *c[1]);
                if graph.get(a).unwrap().contains(&b) {
                    found.insert([k, a, b].into_iter().sorted().collect::<Vec<&str>>());
                }
            }
        }
    }
    Ok(format!("{}", found.len()))
}

pub fn part_2(inp: &str) -> anyhow::Result<String> {
    let graph = parse(inp)?;
    let clique = max_clique(&graph.keys().copied().collect::<FxHashSet<_>>(), &graph);
    let s = clique.into_iter().sorted().join(",");
    Ok(s)
}
