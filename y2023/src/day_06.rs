use anyhow::{anyhow, Result};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use rayon::prelude::*;
use std::str::FromStr;

fn posint(s: &str) -> IResult<&str, u64> {
    map_res(digit1, FromStr::from_str)(s)
}

fn intlist(s: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, posint)(s)
}

fn parse(s: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    tuple((
        preceded(preceded(tag("Time:"), space1), intlist),
        preceded(preceded(tag("\nDistance:"), space1), intlist),
    ))(s)
}

pub fn part_1(s: &str) -> Result<String> {
    let (_, (time, dist)) = parse(s).map_err(|err| anyhow!("{err}"))?;
    let r = time
        .into_iter()
        .zip(dist)
        .map(|(time, dist)| {
            (0..=time)
                .map(|hold| (time - hold) * hold)
                .filter(|r| r > &dist)
                .count()
        })
        .product::<usize>();
    Ok(r.to_string())
}

fn concat_nums(nums: &[u64]) -> u64 {
    nums.iter().fold(0, |acc, n| {
        let mut digs = 1;
        while *n / digs > 0 {
            digs *= 10;
        }
        acc * digs + *n
    })
}

pub fn part_2(s: &str) -> Result<String> {
    let (_, (times, distances)) = parse(s).map_err(|err| anyhow!("{err}"))?;
    let time = concat_nums(&times);
    let dist = concat_nums(&distances);
    let r = (0..=time)
        .into_par_iter()
        .map(|hold| (time - hold) * hold)
        .filter(|r| *r > dist)
        .count();
    Ok(format!("{r}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "Time:      7  15   30
Distance:  9  40  200
";
    #[test]
    fn test_p1() {
        assert_eq!(part_1(EX).unwrap(), "288".to_string());
    }
    #[test]
    fn test_p2() {
        assert_eq!(part_2(EX).unwrap(), "71503".to_string());
    }
    #[test]
    fn test_parse() {
        let (_, ex) = parse(EX).unwrap();
        assert_eq!(ex, (vec![7, 15, 30], vec![9, 40, 200]));
    }
}
