use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

pub fn solve_1(input: &str) -> Result<i32> {
    let mut lines = input.lines();
    let now: i32 = lines
        .next()
        .with_context(|| anyhow!("Empty input"))?
        .parse()?;
    let bus_lines = lines
        .next()
        .with_context(|| anyhow!("Missing bus routes"))?
        .split(',')
        .filter_map(|n| n.parse().ok());

    bus_lines
        .map(|n| (n - now.rem_euclid(n), n))
        .min()
        .with_context(|| anyhow!("No valid bus route"))
        .map(|(id, wait)| id * wait)
}

pub fn part_1(input: &str) -> Result<String> {
    solve_1(input).map(|n| format!("{n}"))
}

fn solve_2(input: &str) -> Result<i64> {
    let offset_busid: Vec<(i64, i64)> = input
        .lines()
        .nth(1)
        .with_context(|| anyhow!("Missing bus routes"))?
        .split(',')
        .enumerate()
        .filter_map(|(i, n)| n.parse().ok().map(|n| (i as i64, n)))
        .collect_vec();

    let ans =
        offset_busid
            .into_iter()
            .fold((1, 0), |(mut lcm, mut current_time), (idx, bus_id)| {
                // For example iteration 1 where idx = 0 and bus_id = 13:
                // lcm becomes 7 and we find time = 77 where 78 % 13 == 0
                while (current_time + idx) % bus_id != 0 {
                    current_time += lcm;
                }
                // Then set lcm = 7 * 13 which is 91, because the next solution to t % 7 == 0 and (t + 1) % 13 is that far away
                // and we need the next t to solve that -- this should be linear in the number of buses
                lcm *= bus_id;

                (lcm, current_time)
            });

    Ok(ans.1)
}

pub fn part_2(input: &str) -> Result<String> {
    solve_2(input).map(|n| format!("{n}"))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let n = solve_2(
            "939
7,13,x,x,59,x,31,19
",
        )
        .unwrap();
        assert_eq!(n, 1068781);
    }
    #[test]
    fn test_1() {
        let n = solve_1(
            "939
7,13,x,x,59,x,31,19
",
        )
        .unwrap();
        assert_eq!(n, 295);
    }
}
