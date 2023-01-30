use anyhow::{anyhow, Context, Result};
use fxhash::{FxHashMap, FxHashSet};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::{Finish, IResult};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn to_arr(&self) -> [i32; 2] {
        use Direction::*;
        match self {
            Up => [0, -1],
            Down => [0, 1],
            Left => [-1, 0],
            Right => [1, 0],
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Instruction(Direction, u32);

fn parse_direction(i: &str) -> IResult<&str, Direction> {
    use Direction::*;
    map(one_of("RDLU"), |ch| match ch {
        'R' => Right,
        'L' => Left,
        'U' => Up,
        'D' => Down,
        _ => unreachable!(),
    })(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        pair(parse_direction, map_res(digit1, FromStr::from_str)),
        |(dir, amount)| Instruction(dir, amount),
    )(i)
}

fn parse_wire(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(complete::char(','), parse_instruction)(i)
}

fn parse_wires(i: &str) -> Result<(Vec<Instruction>, Vec<Instruction>)> {
    fn inner(i: &str) -> IResult<&str, (Vec<Instruction>, Vec<Instruction>)> {
        let (i, first) = parse_wire(i)?;
        let (i, _) = tag("\n")(i)?;
        let (i, second) = parse_wire(i)?;
        Ok((i, (first, second)))
    }
    let (_, pair) = inner(i)
        .finish()
        .map_err(|err| anyhow!("Parse error: {err:?}"))?;
    Ok(pair)
}

fn wire_points(wire: &Vec<Instruction>) -> FxHashMap<[i32; 2], u32> {
    let mut hs = FxHashMap::default();
    let mut loc = [0, 0];
    let mut step = 0;
    for ins in wire {
        for _ in 0..ins.1 {
            step += 1;
            loc.iter_mut()
                .zip(ins.0.to_arr())
                .for_each(|(coord, delta)| *coord += delta);
            hs.insert(loc, step);
        }
    }
    hs
}

pub fn part_1(input: &str) -> Result<String> {
    let (first, second) = parse_wires(input)?;
    let (first, second) = (wire_points(&first), wire_points(&second));
    let first: FxHashSet<_> = first.keys().into_iter().collect();
    let second: FxHashSet<_> = second.keys().into_iter().collect();
    let intersection = first.intersection(&second);
    intersection
        .into_iter()
        .map(|[x, y]| x.abs() + y.abs())
        .min()
        .context("No internsections")
        .map(|n| format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let (first, second) = parse_wires(input)?;
    let (first, second) = (wire_points(&first), wire_points(&second));
    let fk: FxHashSet<_> = first.keys().collect();
    let sk: FxHashSet<_> = second.keys().collect();
    let intersection = fk.intersection(&sk);
    intersection
        .into_iter()
        .map(|&point| first.get(point).unwrap() + second.get(point).unwrap())
        .min()
        .context("No internsections")
        .map(|n| format!("{n}"))
}
