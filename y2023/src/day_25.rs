use anyhow::{Context, Result};
use fxhash::{FxHashMap as Map, FxHashSet as Set};
use itertools::Itertools;use rand::{thread_rng, Rng};
use std::cmp::Reverse;
use std::collections::VecDeque;

type Graph<'a> = Map<&'a str, Vec<&'a str>>;

fn parse_graph(s: &str) -> Option<Graph> {
    let mut graph = Graph::default();
    for line in s.lines().filter(|line| !line.is_empty()) {
        let (source, destinations) = line.split_once(':')?;
        let mut edges = Vec::new();
        for dest in destinations.split(' ').filter(|vtx| !vtx.is_empty()) {
            graph.entry(dest).or_default().push(source);
            edges.push(dest);
        }
        graph.entry(source).or_default().extend(edges.into_iter());
    }
    Some(graph)
}

fn shortest_path<'a>(graph: &'a Graph, source: &'a str, dest: &'a str) -> Vec<&'a str> {
    let mut work = VecDeque::new();
    let mut cache = Set::default();
    work.push_back(vec![source]);

    while let Some(path) = work.pop_front() {
        if let Some(current) = path.last() {
            if *current == dest {
                return path;
            } else {
                for next in graph.get(*current).unwrap() {
                    if cache.insert(next) {
                        let mut next_path = path.clone();
                        next_path.push(*next);
                        work.push_back(next_path);
                    }
                }
            }
        }
    }
    unreachable!()
}

fn shortest_path_stats<'a>(graph: &'a Graph, simulations: u32) -> Map<(&'a str, &'a str), usize> {
    let mut rng = thread_rng();
    let vertices = graph.keys().collect_vec();
    let mut counters: Map<_, usize> = Map::default();

    for _ in 0..simulations {
        let start = rng.gen::<usize>() % vertices.len();
        let end = rng.gen::<usize>() % vertices.len();
        let path = shortest_path(graph, vertices[start], vertices[end]);
        for (&from, &to) in path.iter().zip(path.iter().skip(1)) {
            let source_name = if from < to { from } else { to };
            let dest_name = if from != source_name { from } else { to };
            *counters.entry((source_name, dest_name)).or_default() += 1;
        }
    }
    counters
}

fn reachable_vertices<'a>(graph: &Graph, start: &str) -> usize {
    let mut cache = Set::default();
    let mut work = VecDeque::new();
    work.push_back(start);
    cache.insert(start);
    while let Some(loc) = work.pop_back() {
        for next in graph.get(loc).unwrap() {
            if cache.insert(*next) {
                work.push_back(*next);
            }
        }
    }
    cache.len()
}

pub fn part_1(s: &str) -> Result<String> {
    let graph = parse_graph(s).context("Unparseable graph")?;
    let stats = shortest_path_stats(&graph, 200)
        .into_iter()
        .sorted_by_key(|(_, usage_count)| Reverse(*usage_count))
        .map(|(vtx, _)| vtx)
        .take(3)
        .collect_vec();
    let mut split_graph = Graph::default();
    for (from, tos) in graph.iter() {
        for to in tos {
            if !(stats.contains(&(from, to)) || stats.contains(&(to, from))) {
                split_graph.entry(from).or_default().push(to);
                split_graph.entry(to).or_default().push(from);
            }
        }
    }
    let left_side = reachable_vertices(&split_graph, stats[0].0);
    let right_side = reachable_vertices(&split_graph, stats[0].1);
    Ok(format!("{}", left_side * right_side))
}

pub fn part_2(_: &str) -> Result<String> {
    Ok("Enter the solutions, collect stars".to_string())
}
