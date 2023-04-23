// intellij keeps putting this one back even though rustc says it's unused
#[allow(unused_imports)]
use crate::point_2d::Rem;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::VecDeque;

fn parse(input: &str) -> Result<VecDeque<(usize, i64)>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(idx, line)| {
            let val = line.parse::<i64>()?;
            Ok((idx, val))
        })
        .collect()
}

fn mix(deq: &mut VecDeque<(usize, i64)>) {
    for i in 0..deq.len() {
        let (now_location, _) = deq
            .iter()
            .find_position(|(old_loc, _)| *old_loc == i)
            .unwrap();
        let (idx, val) = deq.remove(now_location).unwrap();
        let new_loc = (now_location as i64 + val).rem_euclid(deq.len() as i64) as usize;
        deq.insert(new_loc, (idx, val));
    }
}

fn solve_1(input: &str) -> Result<i64> {
    let mut deq = parse(input)?;
    mix(&mut deq);
    let (zero_loc, _) = deq
        .iter()
        .find_position(|&(_, val)| *val == 0)
        .context("Lost 0")?;
    let n = (1..=3)
        .map(|idx| (idx * 1000 + zero_loc).rem_euclid(deq.len()))
        .map(|idx| deq.get(idx).unwrap().1)
        .sum();
    Ok(n)
}

fn solve_2(input: &str) -> Result<i64> {
    let mut deq = parse(input)?
        .into_iter()
        .map(|(idx, val)| (idx, val * 811589153))
        .collect();
    for _ in 0..10 {
        mix(&mut deq);
    }
    let (zero_loc, _) = deq
        .iter()
        .find_position(|&(_, val)| *val == 0)
        .context("Lost 0")?;
    let n = (1..=3)
        .map(|idx| (idx * 1000 + zero_loc).rem_euclid(deq.len()))
        .map(|idx| deq.get(idx).unwrap().1)
        .sum();
    Ok(n)
}

pub fn part_1(input: &str) -> Result<String> {
    let n = solve_1(input)?;
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let n = solve_2(input)?;
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "1
2
-3
3
-2
0
4
";

    #[test]
    fn test_parse() {
        let deq = parse(EXAMPLE).unwrap();
        assert_eq!(
            deq,
            vec![1, 2, -3, 3, -2, 0, 4]
                .into_iter()
                .enumerate()
                .collect::<VecDeque<_>>()
        )
    }

    #[test]
    fn test_mix() {
        let mut deq = parse(EXAMPLE).unwrap();
        mix(&mut deq);
        let mixed: Vec<_> = deq.iter().map(|(_, val)| *val).collect();
        assert_eq!(mixed, vec![-2, 1, 2, -3, 4, 0, 3]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_1(EXAMPLE).unwrap(), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_2(EXAMPLE).unwrap(), 1623178306);
    }
}
