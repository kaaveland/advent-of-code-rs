use anyhow::anyhow;
use fxhash::FxHashSet;
use highs::{RowProblem, Sense};
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::{map_res, recognize};
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::IResult;
use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Machine {
    // bitmask
    indicator_lights: u16,
    // bitmasks
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}

fn parse_indicator(s: &str) -> IResult<&str, u16> {
    let (s, indicator) = delimited(tag("["), recognize(many1(one_of(".#"))), tag("]"))(s)?;
    let indicator = indicator
        .chars()
        .rev()
        .map(|ch| if ch == '#' { 1 } else { 0 })
        .fold(0, |acc, ch| (acc << 1) | ch);
    Ok((s, indicator))
}

fn parse_buttons(s: &str) -> IResult<&str, Vec<u16>> {
    fn parse_button(s: &str) -> IResult<&str, u16> {
        let (s, button) = delimited(
            tag("("),
            separated_list1(tag(","), map_res(digit1, |n: &str| n.parse::<u16>())),
            tag(")"),
        )(s)?;
        let button = button.into_iter().map(|on| 1 << on).sum();
        Ok((s, button))
    }
    separated_list1(tag(" "), parse_button)(s)
}

fn parse_joltage(s: &str) -> IResult<&str, Vec<u16>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), map_res(digit1, |n: &str| n.parse::<u16>())),
        tag("}"),
    )(s)
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (s, indicators) = parse_indicator(s)?;
    let (s, _) = char(' ')(s)?;
    let (s, buttons) = parse_buttons(s)?;
    let (s, _) = char(' ')(s)?;
    let (s, joltage) = parse_joltage(s)?;
    Ok((
        s,
        Machine {
            indicator_lights: indicators,
            buttons,
            joltage,
        },
    ))
}

fn parse(s: &str) -> anyhow::Result<Vec<Machine>> {
    separated_list1(tag("\n"), parse_machine)(s)
        .map_err(|err| anyhow!("{err}"))
        .map(|(_, m)| m)
}

fn configure_machine(m: &Machine) -> usize {
    let mut work = VecDeque::from([(0, 0)]);
    let mut cache = FxHashSet::default();

    while let Some((steps, state)) = work.pop_front() {
        if state == m.indicator_lights {
            return steps;
        }
        for &button in m.buttons.iter() {
            let next = state ^ button;
            if cache.insert(next) {
                work.push_back((steps + 1, state ^ button));
            }
        }
    }
    unreachable!()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let machines = parse(s)?;
    let presses: usize = machines.into_iter().map(|m| configure_machine(&m)).sum();
    Ok(format!("{presses}"))
}

fn configure_machine_joltage(m: &Machine) -> usize {
    // This is a system of equations. Taking each joltage, for example:
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // Let us name the buttons a through f then we have:
    // 0th joltage: 3 = e + f
    // 1st joltage: 5 = b + f
    // 2nd joltage: 4 = c + d + e
    // 3rd joltage: 7 = a + b + d
    // This can be set up as a matrix, and we can use gaussian elimination to solve some
    // variables which should make the state space smaller. Let's first set up a matrix and
    // perform gaussian elimination. This may not fix _all_ variables, but it should fix some
    // of them, and we can use search to find the rest. Recall that we must find the _minimum_
    // number of presses that achieves the set joltage. This means each button must be pressed
    // a positive integer number of times.
    let mut pb = RowProblem::default();
    let mut vars = vec![];
    for _ in 0..m.buttons.len() {
        vars.push(pb.add_integer_column(1.0, 0..));
    }
    for (i, &joltage) in m.joltage.iter().enumerate() {
        let mut constraint = vec![];
        for (j, &button) in m.buttons.iter().enumerate() {
            if button & (1 << i) > 0 {
                constraint.push((vars[j], 1.0));
            }
        }
        pb.add_row(joltage..=joltage, constraint.into_iter());
    }
    let model = pb.try_optimise(Sense::Minimise).expect("Unable to solve");
    let solution = model.solve().get_solution();
    solution.columns().iter().map(|n| n.round() as usize).sum()
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let machines = parse(s)?;
    let sum: usize = machines
        .into_par_iter()
        .map(|m| configure_machine_joltage(&m))
        .sum();
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indicator() {
        let (_, i) = parse_indicator("[.##.]").unwrap();
        assert_eq!(i, 6);
        let (_, i) = parse_indicator("[...#.]").unwrap();
        assert_eq!(i, 8);
    }
    #[test]
    fn test_parse_buttons() {
        let s = "(3) (1,3) (2) (2,3) (0,2) (0,1)";
        let r = parse_buttons(s).unwrap().1;
        assert_eq!(r, vec![8, 10, 4, 12, 5, 3]);
    }
    #[test]
    fn test_parse_machine() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let m = parse_machine(s).unwrap().1;
        assert_eq!(
            m,
            Machine {
                indicator_lights: 6,
                buttons: vec![8, 10, 4, 12, 5, 3],
                joltage: vec![3, 5, 4, 7]
            }
        );
    }

    #[test]
    fn test_configure_machine() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let m = parse_machine(s).unwrap().1;
        assert_eq!(configure_machine(&m), 2);
        assert_eq!(configure_machine_joltage(&m), 10);
        let s = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let m = parse_machine(s).unwrap().1;
        assert_eq!(configure_machine(&m), 3);
        assert_eq!(configure_machine_joltage(&m), 12);
        let s = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let m = parse_machine(s).unwrap().1;
        assert_eq!(configure_machine(&m), 2);
        assert_eq!(configure_machine_joltage(&m), 11);
    }
}
