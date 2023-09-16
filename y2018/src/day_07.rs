use anyhow::{anyhow, Result};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Step = char;
type Depends = (char, char);

fn parse_depends(i: &str) -> IResult<&str, Depends> {
    separated_pair(
        preceded(tag("Step "), anychar),
        tag(" must be finished before step "),
        terminated(anychar, tag(" can begin.")),
    )(i)
}

struct DependencyGraph {
    /// edges: values resolve before key, this is flipped
    edges: HashMap<Step, HashSet<Step>>,
}

impl DependencyGraph {
    fn new(depends: &[Depends]) -> Self {
        let mut edges: HashMap<Step, HashSet<Step>> = HashMap::default();
        for (to, from) in depends {
            edges.entry(*from).or_default().insert(*to);
        }
        DependencyGraph { edges }
    }
    fn nodes(&self) -> HashSet<Step> {
        self.edges
            .keys()
            .chain(self.edges.values().flatten())
            .copied()
            .collect()
    }

    fn parse(input: &str) -> Result<Self> {
        let (_, depends) = separated_list1(tag("\n"), parse_depends)(input)
            .map_err(|e| anyhow!("Parse error: {e}"))?;
        Ok(Self::new(&depends))
    }
}

fn topsort_alphabetical(graph: &DependencyGraph) -> Vec<Step> {
    let mut out = vec![];
    let mut blocked = graph.edges.clone();
    let mut queue = BinaryHeap::new();
    let mut expanded = HashSet::default();
    // graph is from blocked -> blocked by
    // all nodes that are not blocked are candidates at first
    let roots = graph
        .nodes()
        .into_iter()
        .filter(|node| !blocked.contains_key(node));
    for root in roots {
        queue.push(Reverse(root));
        expanded.insert(root);
    }
    while let Some(Reverse(current)) = queue.pop() {
        out.push(current);
        // Unblock and expand
        blocked.iter_mut().for_each(|(blocked, blocked_by)| {
            blocked_by.remove(&current);
            if blocked_by.is_empty() && !expanded.contains(blocked) {
                expanded.insert(*blocked);
                queue.push(Reverse(*blocked));
            }
        });
    }
    out
}

pub fn part_1(input: &str) -> Result<String> {
    DependencyGraph::parse(input).map(|graph| {
        let order = topsort_alphabetical(&graph);
        order.into_iter().collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, vec};

    #[test]
    fn test_parse_depends() {
        assert_eq!(
            ("", ('C', 'A')),
            parse_depends("Step C must be finished before step A can begin.").unwrap()
        );
    }

    #[test]
    fn test_topsort_ex() {
        let ex: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
        let graph = DependencyGraph::parse(ex).unwrap();
        let order = topsort_alphabetical(&graph);
        assert_eq!(order, vec!['C', 'A', 'B', 'D', 'F', 'E']);
    }
}
