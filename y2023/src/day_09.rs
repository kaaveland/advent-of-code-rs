use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::IResult;
use std::str::FromStr;

use nom::sequence::preceded;

fn parse(s: &str) -> IResult<&str, Vec<Vec<i64>>> {
    fn posint(s: &str) -> IResult<&str, i64> {
        map_res(digit1, FromStr::from_str)(s)
    }
    let negint = preceded(tag("-"), map(posint, |n: i64| -n));
    let int = alt((negint, posint));
    separated_list1(tag("\n"), separated_list1(space1, int))(s)
}

fn extrapolate(row: &[i64]) -> i64 {
    if row.iter().all(|n| *n == 0) {
        0
    } else {
        let diffs = row
            .iter()
            .zip(row.iter().skip(1))
            .map(|(l, r)| r - l)
            .collect_vec();
        extrapolate(&diffs) + row.last().unwrap()
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let (_, rows) = parse(input).map_err(|err| anyhow!("{err}"))?;
    let s = rows.iter().map(|r| extrapolate(r)).sum::<i64>();
    Ok(s.to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let (_, rows) = parse(input).map_err(|err| anyhow!("{err}"))?;
    let s = rows
        .iter()
        .map(|r| {
            let r = r.iter().rev().copied().collect_vec();
            extrapolate(&r)
        })
        .sum::<i64>();
    Ok(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_extrapolate() {
        let inp = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(extrapolate(&inp), 18);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap(), "114".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EX).unwrap(), "2".to_string());
    }
}
