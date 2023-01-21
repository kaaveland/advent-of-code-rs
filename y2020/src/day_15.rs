use anyhow::{anyhow, Context, Result};

fn solve(n: usize, initial: &[u32]) -> u32 {
    // Map from number to last time it was said -- 0 means new
    let mut spoken = vec![0; n];
    // Map from number to the time it was said prior to being said last time -- 0 means new
    let mut previously_spoken = vec![0; n];

    for (i, n) in initial.iter().copied().enumerate() {
        spoken[n as usize] = (i + 1) as u32;
    }
    let mut last = initial[initial.len() - 1];

    for turn in initial.len()..n {
        let as_i = last as usize;
        let last_said = spoken[as_i];
        let prev_said = previously_spoken[as_i];

        if last_said == 0 || prev_said == 0 {
            last = 0;
        } else {
            last = last_said - prev_said;
        }
        let as_i = last as usize;
        previously_spoken[as_i] = spoken[as_i];
        spoken[as_i] = (turn + 1) as u32;
    }

    last
}

fn solve_str(input: &str, n: usize) -> Result<String> {
    let input: Result<Vec<_>> = input
        .lines()
        .next()
        .with_context(|| anyhow!("Empty input"))?
        .split(',')
        .map(|n| {
            let n = n.parse()?;
            Ok(n)
        })
        .collect();
    input.map(|v| format!("{}", solve(n, &v)))
}

pub fn part_1(input: &str) -> Result<String> {
    solve_str(input, 2020)
}

pub fn part_2(input: &str) -> Result<String> {
    solve_str(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn solve_examples() {
        assert_eq!(solve(10, &[0, 3, 6]), 0);
        assert_eq!(solve(2020, &[1, 3, 2]), 1);
        assert_eq!(solve(2020, &[2, 1, 3]), 10);
        assert_eq!(solve(2020, &[1, 2, 3]), 27);
        assert_eq!(solve(2020, &[2, 3, 1]), 78);
        assert_eq!(solve(2020, &[3, 2, 1]), 438);
        assert_eq!(solve(2020, &[3, 1, 2]), 1836);
    }
}
