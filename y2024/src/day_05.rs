use anyhow::Context;
use fxhash::FxHashSet;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug)]
struct Rules<'a> {
    rules: FxHashSet<&'a str>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Update<'a> {
    id: &'a str,
}

fn parse(input: &str) -> anyhow::Result<(Rules, Vec<Vec<Update>>)> {
    let (rules, updates) = input.split_once("\n\n").context("Malformed input)")?;
    let rules = Rules {
        rules: rules.lines().map(|l| l.trim()).collect(),
    };
    let updates = updates
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.split(',').map(|id| Update { id }).collect())
        .collect();
    Ok((rules, updates))
}

fn sorted_ids<'a>(rules: &'a Rules, updates: &'a [Update]) -> Vec<Update<'a>> {
    updates
        .iter()
        .copied()
        .sorted_by(|a, b| {
            if rules.rules.contains(&format!("{}|{}", a.id, b.id).as_str()) {
                Ordering::Less
            } else if rules.rules.contains(&format!("{}|{}", b.id, a.id).as_str()) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
        .collect()
}

fn middle(updates: &[Update]) -> anyhow::Result<usize> {
    let middle = updates.get(updates.len() / 2).context("Empty updates")?;
    Ok(middle.id.parse::<usize>()?)
}

enum Part {
    Part1,
    Part2,
}

fn solve(input: &str, part: Part) -> anyhow::Result<usize> {
    let (rules, updates) = parse(input)?;
    let mut total = 0;
    for update in updates {
        let sorted = sorted_ids(&rules, &update);
        let eq = sorted == update;
        match part {
            Part::Part1 if eq => total += middle(&sorted)?,
            Part::Part2 if !eq => total += middle(&sorted)?,
            _ => {}
        }
    }
    Ok(total)
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    Ok(format!("{}", solve(input, Part::Part1)?))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    Ok(format!("{}", solve(input, Part::Part2)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    #[test]
    fn test_p1() {
        assert_eq!(solve(EXAMPLE, Part::Part1).unwrap(), 143);
    }

    #[test]
    fn test_p2() {
        assert_eq!(solve(EXAMPLE, Part::Part2).unwrap(), 123);
    }
}
