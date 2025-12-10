use anyhow::anyhow;
use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, digit1, one_of, space0};
use nom::combinator::{map_res, recognize};
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::IResult;
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

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
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
        let s = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let m = parse_machine(s).unwrap().1;
        assert_eq!(configure_machine(&m), 3);
        let s = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let m = parse_machine(s).unwrap().1;
        assert_eq!(configure_machine(&m), 2);
    }
}
