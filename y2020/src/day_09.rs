use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

fn parse(input: &str) -> Vec<u64> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn xmas(input: &[u64], preamble: usize) -> Option<u64> {
    for i in preamble..input.len() - preamble {
        let pre = &input[i - preamble..i];
        let mut sums = pre
            .iter()
            .cartesian_product(pre.iter())
            .filter(|(x, y)| x != y)
            .map(|(x, y)| x + y);
        if !sums.any(|s| s == input[i]) {
            return Some(input[i]);
        }
    }
    None
}

pub fn part_1(input: &str) -> Result<String> {
    let input = parse(input);
    xmas(&input, 25)
        .with_context(|| anyhow!("Unable to solve"))
        .map(|n| format!("{n}"))
}

fn sum_spans(input: &[u64], target: u64, nums: usize) -> Option<u64> {
    (0..input.len() - nums)
        .map(|i| &input[i..i + nums])
        .filter(|v| v.iter().sum::<u64>() == target)
        .filter_map(|v| {
            let min = v.iter().min()?;
            let max = v.iter().max()?;
            Some(*min + *max)
        })
        .next()
}

fn find_weakness(input: &[u64], preamble: usize) -> Option<u64> {
    let target = xmas(input, preamble)?;
    (2..input.len())
        .filter_map(|n| sum_spans(input, target, n))
        .next()
}

pub fn part_2(input: &str) -> Result<String> {
    let input = parse(input);
    find_weakness(&input, 25)
        .with_context(|| anyhow!("Unable to solve"))
        .map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas() {
        let input = parse(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
",
        );
        assert_eq!(xmas(&input, 5), Some(127));
        assert_eq!(find_weakness(&input, 5), Some(62));
    }
}
