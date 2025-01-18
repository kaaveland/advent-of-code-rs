use anyhow::{anyhow, Context};
use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Destination {
    Output(u32),
    Bot(u32),
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Line {
    Give(u32, Destination),
    CompareAndGive(u32, Destination, Destination),
}

fn posint(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |n: &str| n.parse())(s)
}

fn parse_value(s: &str) -> IResult<&str, Line> {
    let (s, v) = preceded(tag("value "), posint)(s)?;
    let (s, b) = preceded(tag(" goes to "), parse_destination)(s)?;
    Ok((s, Line::Give(v, b)))
}

fn parse_destination(s: &str) -> IResult<&str, Destination> {
    alt((
        map(preceded(tag("bot "), posint), Destination::Bot),
        map(preceded(tag("output "), posint), Destination::Output),
    ))(s)
}

fn parse_bot(s: &str) -> IResult<&str, Line> {
    let (s, b) = preceded(tag("bot "), posint)(s)?;
    let (s, low) = preceded(tag(" gives low to "), parse_destination)(s)?;
    let (s, hi) = preceded(tag(" and high to "), parse_destination)(s)?;
    Ok((s, Line::CompareAndGive(b, low, hi)))
}

fn parse(s: &str) -> anyhow::Result<Vec<Line>> {
    separated_list1(tag("\n"), alt((parse_value, parse_bot)))(s)
        .map_err(|err| anyhow!("{err}"))
        .map(|(_, l)| l)
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Slot {
    Empty,
    One(u32),
    Two(u32, u32),
}

impl Slot {
    fn give(&mut self, v: u32) -> Self {
        *self = match self {
            Slot::Empty => Slot::One(v),
            Slot::One(found) => {
                let found = *found;
                Slot::Two(found.min(v), found.max(v))
            }
            _ => panic!("{v}"),
        };
        *self
    }
}

fn eval_tree(prog: &[Line]) -> FxHashMap<Destination, Slot> {
    let mut bindings = FxHashMap::default();
    let bots: FxHashMap<u32, _> = prog
        .iter()
        .filter_map(|line| match line {
            Line::CompareAndGive(bot, _, _) => Some((*bot, line)),
            _ => None,
        })
        .collect();
    let mut work = VecDeque::new();

    for line in prog.iter() {
        if let Line::Give(value, dest) = line {
            work.push_back((*value, *dest))
        }
    }

    while let Some((value, destination)) = work.pop_front() {
        if let Slot::Two(lesser, greater) = bindings
            .entry(destination)
            .or_insert(Slot::Empty)
            .give(value)
        {
            if let Destination::Bot(bot) = destination {
                if let Some(Line::CompareAndGive(_, receive_lesser, receive_greater)) =
                    bots.get(&bot)
                {
                    work.push_back((lesser, *receive_lesser));
                    work.push_back((greater, *receive_greater));
                }
            }
        }
    }

    bindings
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let prog = parse(s)?;
    let tree = eval_tree(&prog);

    if let Some((Destination::Bot(bot), _)) = tree
        .into_iter()
        .find(|(bot, slot)| matches!(bot, Destination::Bot(_)) && matches!(slot, Slot::Two(17, 61)))
    {
        return Ok(bot.to_string());
    }
    Err(anyhow!("Unable to solve"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let prog = parse(s)?;
    let tree = eval_tree(&prog);
    let mut product = 1;
    for i in 0..3u32 {
        let slot = tree
            .get(&Destination::Output(i))
            .context("Unable to solve")?;
        if let Slot::One(p) = slot {
            product *= *p;
        } else {
            return Err(anyhow!("Got {slot:?} for {i}"));
        }
    }
    Ok(product.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";

    #[test]
    fn check_parse_bot() {
        let (s, b) = parse_bot("bot 0 gives low to output 2 and high to output 0").unwrap();
        assert!(s.is_empty());
    }

    #[test]
    fn check_parse() {
        assert_eq!(
            parse(EX).unwrap(),
            vec![
                Line::Give(5, Destination::Bot(2)),
                Line::CompareAndGive(2, Destination::Bot(1), Destination::Bot(0)),
                Line::Give(3, Destination::Bot(1)),
                Line::CompareAndGive(1, Destination::Output(1), Destination::Bot(0)),
                Line::CompareAndGive(0, Destination::Output(2), Destination::Output(0)),
                Line::Give(2, Destination::Bot(2))
            ]
        );
    }
}
