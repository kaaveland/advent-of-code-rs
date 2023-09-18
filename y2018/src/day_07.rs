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
    schedule(graph, 1, 0).1
}

fn time_required(task: char, base_cost: i32) -> i32 {
    (task as i32) - ('A' as i32) + base_cost + 1
}

fn schedule(graph: &DependencyGraph, workers: usize, base_cost: i32) -> (i32, Vec<Step>) {
    let mut out = vec![];
    let mut blocked = graph.edges.clone();
    let mut queue = BinaryHeap::new();
    let mut scheduled = HashSet::default();
    let mut capacity = workers;
    let mut time_passed: i32 = 0;
    let size = graph.nodes().len();

    while out.len() < size {
        // Schedule as much work as possible
        while capacity > 0 {
            // Identify something we can schedule according to conditions:
            // 1) It must not be scheduled already
            // 2) It must not be blocked by some task that is not completed
            if let Some(next) = graph
                .nodes()
                .iter()
                .filter(|step| {
                    !(scheduled.contains(*step)
                        || (blocked.contains_key(step) && !blocked.get(step).unwrap().is_empty()))
                })
                .min()
            {
                capacity -= 1;
                scheduled.insert(*next);
                queue.push(Reverse((
                    time_passed + time_required(*next, base_cost),
                    *next,
                )))
            } else {
                break;
            }
        }
        // Complete some step. Capacity can't increase until task is complete, so we won't be able
        // to schedule until that happens and can just skip in time.
        if let Some(Reverse((time, step))) = queue.pop() {
            out.push(step);
            time_passed = time;
            capacity += 1;
            blocked.iter_mut().for_each(|(_, blocked_by)| {
                blocked_by.remove(&step);
            })
        }
    }
    (time_passed, out)
}

pub fn part_1(input: &str) -> Result<String> {
    DependencyGraph::parse(input).map(|graph| {
        let order = topsort_alphabetical(&graph);
        order.into_iter().collect()
    })
}

pub fn part_2(input: &str) -> Result<String> {
    DependencyGraph::parse(input).map(|graph| {
        let (time, _) = schedule(&graph, 5, 60);
        time.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, vec};
    const EXAMPLE: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
    #[test]
    fn test_parse_depends() {
        assert_eq!(
            ("", ('C', 'A')),
            parse_depends("Step C must be finished before step A can begin.").unwrap()
        );
    }

    #[test]
    fn test_topsort_ex() {
        let graph = DependencyGraph::parse(EXAMPLE).unwrap();
        let order = topsort_alphabetical(&graph);
        assert_eq!(order, vec!['C', 'A', 'B', 'D', 'F', 'E']);
    }

    #[test]
    fn test_schedule_ex() {
        let graph = DependencyGraph::parse(EXAMPLE).unwrap();
        let (time, order) = schedule(&graph, 2, 0);
        assert_eq!(time, 15);
        assert_eq!(order, vec!['C', 'A', 'B', 'F', 'D', 'E']);
    }
}
