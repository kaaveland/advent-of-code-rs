use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::iter::repeat;

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn pattern(repeats: usize) -> impl Iterator<Item = i32> {
    let base = BASE_PATTERN
        .iter()
        .copied()
        .flat_map(move |n| repeat(n).take(repeats))
        .cycle();
    base.skip(1)
}

fn apply_fft(inputs: &[i32]) -> Vec<i32> {
    (1..=inputs.len())
        .into_par_iter()
        .map(pattern)
        .map(|pat| {
            let n = inputs
                .iter()
                .copied()
                .zip(pat)
                .fold(0, |a, (lhs, rhs)| lhs * rhs + a);
            (n % 10).abs()
        })
        .collect()
}

pub fn part_1(input: &str) -> Result<String> {
    let line = input.lines().next().context("Empty input")?;
    let mut inputs = line
        .as_bytes()
        .iter()
        .map(|ch| (*ch - b'0') as i32)
        .collect_vec();
    for _ in 0..100 {
        inputs = apply_fft(&inputs);
    }
    let out = inputs
        .into_iter()
        .take(8)
        .map(|dig| ((dig as u8) + b'0') as char)
        .join("");
    Ok(out)
}

pub fn part_2(input: &str) -> Result<String> {
    let line = input.lines().next().context("Empty input")?;
    let mut inputs = line
        .as_bytes()
        .iter()
        .map(|ch| (*ch - b'0') as i32)
        .cycle()
        .take(10000 * line.chars().count())
        .collect_vec();
    let skip = inputs[..7]
        .iter()
        .copied()
        .fold(0, |acc, dig| acc * 10 + dig);
    inputs = inputs.into_iter().skip(skip as usize).collect_vec();
    let mut sums = Vec::with_capacity(inputs.len());
    let len = inputs.len();
    for _ in 0..100 {
        sums.clear();
        let mut total = 0;
        sums.push(0);
        for n in inputs.iter().copied() {
            total += n;
            sums.push(total);
        }
        let last = sums.len() - 1;
        inputs.clear();
        inputs.extend((0..len).map(|i| (sums[last] - sums[i]) % 10));
    }
    let n = inputs[..8].iter().copied().fold(0, |acc, n| acc * 10 + n);
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, vec};

    #[test]
    fn test_pattern() {
        let expect = vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1];
        let p = pattern(2).take(expect.len()).collect_vec();
        assert_eq!(p, expect);
    }

    #[test]
    fn test_fft() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let output = apply_fft(&input);
        assert_eq!(output, vec![4, 8, 2, 2, 6, 1, 5, 8]);
    }
}
