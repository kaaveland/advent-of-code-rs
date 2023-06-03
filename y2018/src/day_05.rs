use anyhow::Result;
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use rayon::prelude::*;

fn step_polymer(polymer: &[char]) -> Vec<char> {
    let mut stack: Vec<char> = vec![];
    for &c in polymer {
        if let Some(&last) = stack.last() {
            if last != c && last.eq_ignore_ascii_case(&c) {
                stack.pop();
                continue;
            }
        }
        stack.push(c);
    }
    stack
}

pub fn part_1(input: &str) -> Result<String> {
    let polymer: Vec<_> = input.trim_end().chars().collect();
    let reacted = step_polymer(&polymer);
    Ok(reacted.len().to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let polymer: Vec<_> = input.trim_end().chars().collect();
    let units: HashSet<_> = polymer.iter().map(|c| c.to_ascii_lowercase()).collect();
    units
        .into_par_iter()
        .map(|unit| {
            let candidate = polymer
                .iter()
                .filter(|c| !c.eq_ignore_ascii_case(&unit))
                .copied()
                .collect_vec();
            step_polymer(&candidate).len()
        })
        .min()
        .map(|n| n.to_string())
        .ok_or_else(|| anyhow::anyhow!("No solution found!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input: Vec<_> = "dabAcCaCBAcCcaDA".chars().collect();
        let expect: Vec<_> = "dabCBAcaDA".chars().collect();
        assert_eq!(step_polymer(&input), expect);
    }
}
