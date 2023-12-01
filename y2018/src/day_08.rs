use anyhow::{Context, Result};
use std::num::ParseIntError;

fn parse(input: &str) -> Result<Vec<u32>> {
    let v = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>())
        .collect::<Result<_, ParseIntError>>()?;
    Ok(v)
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<Metadata>,
}
type Metadata = u32;

fn parse_nodes(input: &mut impl Iterator<Item = u32>) -> Option<Node> {
    let num_children = input.next()?;
    let num_metadata = input.next()?;
    let mut children = Vec::with_capacity(num_children as usize);
    for _ in 0..num_children {
        children.push(parse_nodes(input)?);
    }
    let mut metadata = Vec::with_capacity(num_metadata as usize);
    for _ in 0..num_metadata {
        metadata.push(input.next()?);
    }
    Some(Node { children, metadata })
}

fn sum_metadata(node: &Node) -> u32 {
    node.metadata.iter().sum::<u32>() + node.children.iter().map(sum_metadata).sum::<u32>()
}
pub fn part_1(input: &str) -> Result<String> {
    let v = parse(input)?;
    let node = parse_nodes(&mut v.iter().copied()).context("Parse error")?;
    Ok(sum_metadata(&node).to_string())
}

fn node_value(node: &Node) -> u32 {
    if node.children.is_empty() {
        node.metadata.iter().sum::<u32>()
    } else {
        node.metadata
            .iter()
            .copied()
            .filter(|&meta| meta > 0 && meta <= node.children.len() as u32)
            .map(|meta| node_value(&node.children[(meta - 1) as usize]))
            .sum::<u32>()
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;

    const EX: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_parse_nodes() {
        let numbers = parse(EX).unwrap();
        let mut stream = numbers.iter().copied();
        let n = parse_nodes(&mut stream);
        assert_eq!(
            n.unwrap(),
            Node {
                children: vec![
                    Node {
                        children: vec![],
                        metadata: vec![10, 11, 12]
                    },
                    Node {
                        children: vec![Node {
                            children: vec![],
                            metadata: vec![99]
                        }],
                        metadata: vec![2]
                    }
                ],
                metadata: vec![1, 1, 2]
            }
        )
    }

    #[test]
    fn test_sum_metadata() {
        let numbers = parse(EX).unwrap();
        let mut stream = numbers.iter().copied();
        let n = parse_nodes(&mut stream);
        assert_eq!(sum_metadata(&n.unwrap()), 138);
    }

    #[test]
    fn test_node_value() {
        let numbers = parse(EX).unwrap();
        let mut stream = numbers.iter().copied();
        let n = parse_nodes(&mut stream);
        assert_eq!(node_value(&n.unwrap()), 66);
    }
}
