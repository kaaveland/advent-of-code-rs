use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<char> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Hand::*;
        match value {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err(anyhow!("Illegal hand: {value}")),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        use Hand::*;
        use Ordering::*;
        match (self, other) {
            (Rock, Scissors) => Greater,
            (Paper, Rock) => Greater,
            (Scissors, Paper) => Greater,
            (Rock, Rock) => Equal,
            (Paper, Paper) => Equal,
            (Scissors, Scissors) => Equal,
            (Scissors, Rock) => Less,
            (Paper, Scissors) => Less,
            (Rock, Paper) => Less,
        }
    }
}

impl Hand {
    fn points(&self) -> u32 {
        use Hand::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn points(duel: &(Hand, Hand)) -> u32 {
    use Ordering::*;
    let (theirs, my) = duel;
    my.points()
        + match my.cmp(theirs) {
            Less => 0,
            Equal => 3,
            Greater => 6,
        }
}

fn parse_duel(line: &str) -> Result<(Hand, Hand)> {
    let parts = line.split_ascii_whitespace().collect_vec();
    let my = parts[0].chars().next().context("Empty hand")?.try_into()?;
    let theirs = parts[1].chars().next().context("Empty hand")?.try_into()?;
    Ok((my, theirs))
}

fn solve_1(inp: &str) -> Result<u32> {
    let results: Result<Vec<_>> = inp
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_duel)
        .collect();
    results.map(|duels| duels.iter().map(points).sum())
}

fn solve_2(inp: &str) -> Result<u32> {
    let results: Result<Vec<_>> = inp
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_duel)
        .collect();
    let points = results?
        .iter()
        .map(|duel| {
            use Hand::*;
            use Ordering::*;
            let (their_choice, my_condition) = duel;
            let desired_outcome = match my_condition {
                Rock => Less,        // X
                Paper => Equal,      // Y
                Scissors => Greater, // Z
            };
            let options = [Rock, Paper, Scissors];
            let my_choice = options
                .iter()
                .find(|&hand| hand.cmp(their_choice) == desired_outcome)
                .unwrap();
            points(&(*their_choice, *my_choice))
        })
        .sum();
    Ok(points)
}

pub fn part_1(input: &str) -> Result<String> {
    let sol = solve_1(input)?;
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let sol = solve_2(input)?;
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y
B X
C Z
";

    #[test]
    fn part1_test() {
        assert_eq!(solve_1(EXAMPLE).unwrap(), 15);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_2(EXAMPLE).unwrap(), 12);
    }
}
