use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Eq, PartialEq, Clone, Debug)]
struct ColumnRanges<'a> {
    labels: HashMap<&'a str, usize>,
    ranges: Vec<[RangeInclusive<u16>; 2]>,
}

type Ticket = Vec<u16>;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Problem<'a> {
    rules: ColumnRanges<'a>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn parse_range(segment: &str) -> Result<RangeInclusive<u16>> {
    let (start, end) = segment
        .split_once('-')
        .with_context(|| anyhow!("Missing delimiter - in {segment}"))?;
    Ok(start.parse()?..=end.parse()?)
}

fn parse_column_range(line: &str) -> Result<(&str, [RangeInclusive<u16>; 2])> {
    let (name, value) = line
        .split_once(": ")
        .with_context(|| anyhow!("Missing delimiter : in {line}"))?;
    let (first, second) = value
        .split_once(" or ")
        .with_context(|| anyhow!("Missing delimiter or in {value}"))?;
    Ok((name, [parse_range(first)?, parse_range(second)?]))
}

fn parse_column_ranges(block: &str) -> Result<ColumnRanges<'_>> {
    let input = block
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_column_range);
    let mut labels = HashMap::default();
    let mut ranges = Vec::default();

    for (label, r) in input.enumerate() {
        let (name, r) = r?;
        labels.insert(name, label);
        ranges.push(r);
    }

    Ok(ColumnRanges { labels, ranges })
}

fn parse_ticket(line: &str) -> Result<Ticket> {
    line.split(',').map(str::parse).map(|r| Ok(r?)).collect()
}

fn parse_ticket_block(block: &str) -> Result<Vec<Ticket>> {
    block
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(parse_ticket)
        .collect()
}

fn parse_input(input: &str) -> Result<Problem<'_>> {
    let ctx = || anyhow!("Input too short: {input}");
    let mut input = input.split("\n\n");
    let ranges = input
        .next()
        .with_context(ctx)
        .and_then(parse_column_ranges)?;
    let my_ticket = input
        .next()
        .with_context(ctx)
        .and_then(parse_ticket_block)?;
    let other_tickets = input
        .next()
        .with_context(ctx)
        .and_then(parse_ticket_block)?;

    Ok(Problem {
        rules: ranges,
        my_ticket: my_ticket[0].clone(),
        other_tickets,
    })
}

fn solve_1(input: &str) -> Result<u64> {
    let problem = parse_input(input)?;
    let invalid_numbers: u64 = problem
        .other_tickets
        .iter()
        .flat_map(|ticket| {
            ticket.iter().filter_map(|v| {
                if !problem
                    .rules
                    .ranges
                    .iter()
                    .any(|rule| rule[0].contains(v) || rule[1].contains(v))
                {
                    Some(*v as u64)
                } else {
                    None
                }
            })
        })
        .sum();
    Ok(invalid_numbers)
}

pub fn part_1(input: &str) -> Result<String> {
    solve_1(input).map(|n| format!("{n}"))
}

fn remove_possibility(possibilites: &mut Vec<Vec<bool>>, column: usize, rule_column: usize) {
    // First time we remove this possible choice
    if possibilites[column][rule_column] {
        possibilites[column][rule_column] = false;
        if possibilites[column]
            .iter()
            .filter(|possible| **possible)
            .count()
            == 1
        {
            // There's only one remaining possible choice, so we found the only valid choice
            // for our column now and it's this one:
            let remaining = possibilites[column]
                .iter()
                .enumerate()
                .find(|(_, status)| **status)
                .map(|(choice, _)| choice)
                .unwrap();

            // Since `remaining` was right in `column`, it must be wrong everywhere else and we can
            // eliminate it elsewhere
            for i in 0..possibilites.len() {
                if i != column {
                    remove_possibility(possibilites, i, remaining);
                }
            }
        }
    }
}

fn solve_2(input: &str) -> Result<u64> {
    let problem = parse_input(input)?;
    let valid_tickets = problem.other_tickets.iter().filter(|ticket| {
        ticket.iter().all(|v| {
            problem
                .rules
                .ranges
                .iter()
                .any(|rule| rule[0].contains(v) || rule[1].contains(v))
        })
    });
    let mut possible = vec![vec![true; problem.rules.ranges.len()]; problem.rules.ranges.len()];

    for ticket in valid_tickets {
        for (column, value) in ticket.iter().enumerate() {
            for (rule_column, rule) in problem.rules.ranges.iter().enumerate() {
                if !(rule[0].contains(value) || rule[1].contains(value)) {
                    remove_possibility(&mut possible, column, rule_column);
                }
            }
        }
    }

    if !possible
        .iter()
        .all(|choices| 1 == choices.iter().filter(|choice| **choice).count())
    {
        Err(anyhow!(
            "Unable to propagate constraints; multiple candidate labels possible for some columns: {possible:?}"
        ))
    } else {
        // We know now which rule that goes with which column:
        let cols_to_rules = possible.iter().map(|choices| {
            choices
                .iter()
                .enumerate()
                .find(|(_, is_right)| **is_right)
                .map(|(choice, _)| choice)
                .unwrap()
        });
        let rules_to_use = problem
            .rules
            .labels
            .iter()
            .filter(|(name, _)| name.starts_with("departure"))
            .map(|(_, rule_no)| rule_no)
            .collect_vec();
        let ans = cols_to_rules
            .enumerate()
            .filter(|(_, rule_no)| rules_to_use.contains(&rule_no))
            .map(|(col, _)| problem.my_ticket[col])
            .fold(1, |acc, n| acc * (n as u64));

        Ok(ans)
    }
}

pub fn part_2(input: &str) -> Result<String> {
    solve_2(input).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex() {
        assert_eq!(solve_1(EXAMPLE).unwrap(), 71);
    }

    const EXAMPLE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
}
