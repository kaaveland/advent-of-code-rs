use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::cmp::Ordering;

#[cfg(test)]
pub mod tests {
    use super::*;
    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(EXAMPLE).unwrap(), 198);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE).unwrap(), 230);
    }
}

fn bitcounts(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let digits = input.lines().next().map(|l| l.len()).unwrap_or(0);

    let mut zeros = vec![0; digits];
    let mut ones = vec![0; digits];

    for line in input.lines().filter(|l| !l.is_empty()) {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '0' => {
                    zeros[i] += 1;
                }
                '1' => {
                    ones[i] += 1;
                }
                _ => return Err(anyhow!("Illegal char: {ch}")),
            }
        }
    }
    Ok((zeros, ones))
}

fn solve_1(input: &str) -> Result<i64> {
    let (zeros, ones) = bitcounts(input)?;

    let (gamma, epsilon) =
        zeros
            .iter()
            .zip(ones.iter())
            .fold((0, 0), |(gamma, eps), (&zcount, &ocount)| {
                let (add_g, add_e) = if zcount > ocount { (0, 1) } else { (1, 0) };
                (gamma * 2 + add_g, eps * 2 + add_e)
            });

    Ok(gamma * epsilon)
}

pub fn part_1(input: &str) -> Result<()> {
    let r = solve_1(input)?;
    println!("{r}");
    Ok(())
}

fn bin2dec(inp: &Vec<char>) -> i64 {
    let mut out = 0;
    for ch in inp {
        out = out * 2 + i64::from(*ch == '1')
    }
    out
}

fn rating(nums: &[Vec<char>], favor: char) -> i64 {
    let mut candidates = nums.iter().collect_vec();
    let mut pos = 0;
    while candidates.len() > 1 {
        let ones_count = candidates.iter().filter(|num| num[pos] == '1').count();
        let z_count = candidates.len() - ones_count;
        let lookfor = match ones_count.cmp(&z_count) {
            Ordering::Less => {
                if favor == '1' {
                    '0'
                } else {
                    '1'
                }
            }
            Ordering::Equal => favor,
            Ordering::Greater => {
                if favor == '1' {
                    '1'
                } else {
                    '0'
                }
            }
        };
        candidates.retain(|num| num[pos] == lookfor);
        pos += 1;
    }
    bin2dec(candidates[0])
}

fn solve_2(input: &str) -> Result<i64> {
    let nums = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    let o2_rating = rating(&nums, '1');
    let co2_rating = rating(&nums, '0');

    Ok(o2_rating * co2_rating)
}

pub fn part_2(input: &str) -> Result<()> {
    let r = solve_2(input)?;
    println!("{r}");
    Ok(())
}
