use anyhow::{anyhow, Result};
use fxhash::FxHashSet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::str::FromStr;

fn parse_nat<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(digit1, |n: &str| n.parse::<T>())(input)
}

fn parse(input: &str) -> Result<Vec<i32>> {
    let pos_int = preceded(tag("+"), parse_nat);
    let neg_int = map(preceded(tag("-"), parse_nat), |n: i32| -n);
    let num = alt((pos_int, neg_int));
    let (_, numbers) = separated_list1(tag("\n"), num)(input).map_err(|e| anyhow!("{e}"))?;
    Ok(numbers)
}
pub fn part_1(input: &str) -> Result<String> {
    let numbers = parse(input)?;
    Ok(format!("{}", numbers.iter().sum::<i32>()))
}

pub fn part_2(input: &str) -> Result<String> {
    let numbers = parse(input)?;
    let mut seen = FxHashSet::default();
    let mut sum = 0;
    for n in numbers.into_iter().cycle() {
        sum += n;
        if !seen.insert(sum) {
            return Ok(format!("{}", sum));
        }
    }
    Err(anyhow!("No frequencies seen twice"))
}
