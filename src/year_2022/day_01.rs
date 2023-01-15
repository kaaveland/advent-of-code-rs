use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .split("\n\n")
        .map(|block| {
            let r: Result<Vec<_>, ParseIntError> = block.lines().map(str::parse::<i32>).collect();
            Ok(r?)
        })
        .collect()
}

fn largest_group(groups: &[Vec<i32>]) -> Result<i32> {
    groups
        .iter()
        .map(|v| v.iter().sum())
        .max()
        .context("Empty groups")
}

fn top_n(groups: &[Vec<i32>], n: usize) -> Result<i32> {
    let top_n: Vec<i32> = groups
        .iter()
        .map(|v| v.iter().sum())
        .sorted()
        .rev()
        .collect();

    if top_n.len() < n {
        Err(Error::msg("Too few groups"))
    } else {
        Ok(top_n.iter().take(n).sum())
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let groups = parse_input(input)?;
    let sol = largest_group(&groups)?;
    Ok(format!("Solution: {sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let groups = parse_input(input)?;
    let sol = top_n(&groups, 3)?;
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_parse_input() {
        let groups = parse_input(EXAMPLE).expect("Unable to parse");
        assert_eq!(groups[0], vec![1000, 2000, 3000]);
        assert_eq!(groups[1], vec![4000]);
    }

    #[test]
    fn test_largest_group() {
        let groups = parse_input(EXAMPLE).expect("Unable to parse");
        assert_eq!(largest_group(&groups).expect("Unable to sum"), 24000);
    }

    #[test]
    fn test_top_n() {
        let groups = parse_input(EXAMPLE).expect("Unable to parse");
        assert_eq!(top_n(&groups, 3).expect("Too few n"), 45000);
    }
}
