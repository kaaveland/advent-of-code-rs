use anyhow::Result;
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|n| n.parse().ok())
        .sorted()
        .collect()
}

fn solve_1(input: &str) -> i32 {
    let adapters = parse(input);
    let mut diffs = HashMap::default();
    let mut current_joltage = 0;
    for joltage in adapters {
        let diff = joltage - current_joltage;
        if (1..=3).contains(&diff) {
            *diffs.entry(joltage - current_joltage).or_default() += 1;
            current_joltage = joltage;
        } else {
            panic!("Can't connect {joltage} to {current_joltage}")
        }
    }
    *diffs.entry(3).or_default() += 1;
    *diffs.get(&3).unwrap_or(&0) * diffs.get(&1).unwrap_or(&0)
}

pub fn part_1(input: &str) -> Result<String> {
    Ok(format!("{}", solve_1(input)))
}

fn solve_2(input: &str) -> i64 {
    let mut adapters: Vec<i32> = input
        .lines()
        .filter_map(|n| n.parse().ok())
        .sorted()
        .collect();
    let max_val = adapters[adapters.len() - 1] + 3;
    adapters.push(max_val);
    let mut connect_count = vec![0; max_val as usize + 1];
    connect_count[0] = 1;
    for adapter in adapters {
        connect_count[adapter as usize] = (1..=3)
            .map(|c| adapter - c)
            .filter_map(|c| connect_count.get(c as usize))
            .sum();
    }
    connect_count[max_val as usize]
}

pub fn part_2(input: &str) -> Result<String> {
    Ok(format!("{}", solve_2(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2_example() {
        let n = solve_2(EX_1);
        assert_eq!(n, 8);

        let n = solve_2(EX_2);
        assert_eq!(n, 19208);
    }

    #[test]
    fn test_example() {
        let n = solve_1(EX_1);
        assert_eq!(7 * 5, n);
        let n = solve_1(EX_2);
        assert_eq!(n, 22 * 10);
    }
    const EX_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
    const EX_1: &str = "16
10
15
5
1
11
7
19
6
12
4
";
}
