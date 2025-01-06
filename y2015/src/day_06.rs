use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use rayon::prelude::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Command {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct CommandSegment {
    command: Command,
    xmin: u16,
    xmax: u16,
    ymin: u16,
    ymax: u16,
}

fn parse_command(s: &str) -> IResult<&str, Command> {
    alt((
        map(tag("toggle"), |_| Command::Toggle),
        map(tag("turn on"), |_| Command::On),
        map(tag("turn off"), |_| Command::Off),
    ))(s)
}

fn posint(s: &str) -> IResult<&str, u16> {
    map_res(digit1, |s: &str| s.parse())(s)
}

fn parse_command_segment(s: &str) -> IResult<&str, CommandSegment> {
    let (s, command) = parse_command(s)?;
    let (s, xmin) = preceded(space1, posint)(s)?;
    let (s, ymin) = preceded(char(','), posint)(s)?;
    let (s, xmax) = preceded(tag(" through "), posint)(s)?;
    let (s, ymax) = preceded(char(','), posint)(s)?;
    Ok((
        s,
        CommandSegment {
            command,
            xmin,
            xmax: xmax + 1,
            ymin,
            ymax: ymax + 1,
        },
    ))
}

fn parse(s: &str) -> anyhow::Result<Vec<CommandSegment>> {
    separated_list1(char('\n'), parse_command_segment)(s)
        .map_err(|err| anyhow!("{err}"))
        .map(|(_, s)| s)
}

fn apply_all<'a, S: Copy + Default, F: Fn(S, Command) -> S>(
    commands: impl Iterator<Item = &'a CommandSegment>,
    f: F,
) -> Vec<S> {
    let mut work = vec![S::default(); 1000];
    for cmd in commands {
        for x in cmd.xmin..cmd.xmax {
            work[x as usize] = f(work[x as usize], cmd.command);
        }
    }
    work
}

fn grade_segments<S: Copy + Default, F: Fn(S, Command) -> S + Sync, G: Fn(&[S]) -> usize + Sync>(
    f: &F,
    score: G,
    commands: &[CommandSegment],
) -> usize {
    (0..1000)
        .into_par_iter()
        .map(|y| {
            let cmds = commands
                .iter()
                .filter(|cmd| (cmd.ymin..cmd.ymax).contains(&y));
            let r = apply_all(cmds, f);
            score(&r)
        })
        .sum()
}

fn p1_state_change(state: bool, command: Command) -> bool {
    match command {
        Command::On => true,
        Command::Off => false,
        Command::Toggle => !state,
    }
}

fn p1_score(seg: &[bool]) -> usize {
    seg.iter().filter(|b| **b).count()
}

fn p2_state_change(state: u16, command: Command) -> u16 {
    match command {
        Command::On => state + 1,
        Command::Off => state.saturating_sub(1),
        Command::Toggle => state + 2,
    }
}

fn p2_score(seg: &[u16]) -> usize {
    let mut s = 0usize;
    for state in seg {
        s += *state as usize;
    }
    s
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let commands = parse(s)?;
    Ok(grade_segments(&p1_state_change, p1_score, &commands).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let commands = parse(s)?;
    Ok(grade_segments(&p2_state_change, p2_score, &commands).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(
            part_1("turn on 499,499 through 500,500").unwrap().as_str(),
            "4"
        );
    }
}
