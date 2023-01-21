use anyhow::{anyhow, Context, Result};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug)]
struct Graph<'a> {
    labels: HashMap<&'a str, usize>,
    edges: Vec<Vec<usize>>,
    costs: Vec<Vec<usize>>,
}

fn parse(input: &str) -> Result<Graph> {
    let mut labels = HashMap::default();
    let mut edges = vec![];
    let mut costs = vec![];
    let mut label = 0;

    for line in input.lines().filter(|line| !line.is_empty()) {
        let (container, rest) = line
            .split_once(" contain ")
            .with_context(|| anyhow!("Invalid: {line}"))?;
        let container = &container[..container.len() - 1]; // drop trailing s
        if !labels.contains_key(container) {
            labels.insert(container, label);
            costs.push(vec![]);
            edges.push(vec![]);
            label += 1;
        }
        let src = *labels.get(container).unwrap();
        let rest = &rest[..rest.len() - 1];
        if rest != "no other bags" {
            for next in rest.split(", ") {
                let (amount, rest) = next
                    .split_once(' ')
                    .with_context(|| anyhow!("Invalid rest: {rest}"))?;
                let amount = amount.parse::<usize>()?;
                let containee = rest.strip_suffix('s').unwrap_or(rest);
                if !labels.contains_key(containee) {
                    labels.insert(containee, label);
                    costs.push(vec![]);
                    edges.push(vec![]);
                    label += 1;
                }
                let dst = *labels.get(containee).unwrap();
                edges[src].push(dst);
                costs[src].push(amount);
            }
        }
    }
    Ok(Graph {
        labels,
        edges,
        costs,
    })
}

fn solve_1(input: &str) -> Result<usize> {
    let graph = parse(input)?;
    let mut visited = HashSet::default();

    let target = *graph
        .labels
        .get("shiny gold bag")
        .with_context(|| anyhow!("{input} contains no shiny gold bags"))?;
    let mut reverse_edges = vec![vec![]; graph.edges.len()];
    for src in 0..reverse_edges.len() {
        for dst in graph.edges[src].iter().copied() {
            reverse_edges[dst].push(src);
        }
    }
    let mut work = VecDeque::from([target]);
    while let Some(node) = work.pop_front() {
        if node != target {
            visited.insert(node);
        }
        for next in reverse_edges[node].iter().copied() {
            if !visited.contains(&next) {
                work.push_back(next);
            }
        }
    }
    Ok(visited.len())
}

pub fn part_1(input: &str) -> Result<String> {
    solve_1(input).map(|sol| format!("{sol}"))
}

fn solve_2(input: &str) -> Result<usize> {
    let graph = parse(input)?;
    let start = *graph
        .labels
        .get("shiny gold bag")
        .with_context(|| anyhow!("{input} contains no shiny gold bags"))?;
    let mut total_cost = 0;
    let mut work = VecDeque::from([(start, 1)]);
    while let Some((node, parents)) = work.pop_front() {
        for (next, cost) in graph.edges[node]
            .iter()
            .copied()
            .zip(graph.costs[node].iter().copied())
        {
            total_cost += cost * parents;
            work.push_back((next, cost * parents));
        }
    }
    Ok(total_cost)
}

pub fn part_2(input: &str) -> Result<String> {
    solve_2(input).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let _graph = parse(EXAMPLE).unwrap();
    }

    #[test]
    fn test_part1() {
        let sol = solve_1(EXAMPLE).unwrap();
        assert_eq!(sol, 4);
    }

    #[test]
    fn test_part2() {
        let sol = solve_2(EXAMPLE).unwrap();
        assert_eq!(sol, 32);
    }

    const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
}
