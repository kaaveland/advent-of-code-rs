use anyhow::{anyhow, Context};
use fxhash::{FxHashMap, FxHashSet};
use regex::Regex;

fn parse(s: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    let re = Regex::new(
        r"([A-Za-z]+) would (gain|lose) ([0-9]+) happiness units by sitting next to ([A-Za-z]+)",
    )
    .unwrap();
    let mut interned = FxHashMap::default();
    for line in s.lines() {
        let m = re
            .captures(line)
            .with_context(|| anyhow!("No match in: {line}"))?;
        let ix = interned.len();
        let lhs = m.get(1).unwrap().as_str();
        interned.entry(lhs).or_insert(ix);
        let ix = interned.len();
        let rhs = m.get(4).unwrap().as_str();
        interned.entry(rhs).or_insert(ix);
    }
    let mut out = vec![vec![0; interned.len()]; interned.len()];
    for line in s.lines() {
        let m = re
            .captures(line)
            .with_context(|| anyhow!("No match in: {line}"))?;
        let mul = if m.get(2).unwrap().as_str() == "lose" {
            -1
        } else {
            1
        };
        let size: i32 = m.get(3).unwrap().as_str().parse()?;
        let lhs = *interned.get(m.get(1).unwrap().as_str()).unwrap();
        let rhs = *interned.get(m.get(4).unwrap().as_str()).unwrap();
        out[lhs][rhs] = mul * size;
    }
    Ok(out)
}

fn gen_perm_scores(
    costs: &[Vec<i32>],
    permutation: &mut Vec<usize>,
    used: usize,
    scores_found: &mut FxHashSet<i32>,
) {
    if used.count_ones() as usize == costs.len() {
        // permutation is complete
        let mut score = 0;
        for ix in 0..costs.len() {
            let me = permutation[ix];
            let left = if ix == 0 { costs.len() - 1 } else { ix - 1 };
            let left = permutation[left];
            score += costs[me][left];
            let right = if ix == costs.len() - 1 { 0 } else { ix + 1 };
            let right = permutation[right];
            score += costs[me][right];
        }
        scores_found.insert(score);
    } else {
        for candidate in 0..costs.len() {
            let bit = (1 << candidate) as usize;
            if used & bit == 0 {
                permutation.push(candidate);
                gen_perm_scores(costs, permutation, used | bit, scores_found);
                permutation.pop();
            }
        }
    }
}

fn all_scores(costs: &[Vec<i32>]) -> FxHashSet<i32> {
    let mut scores = FxHashSet::default();
    let mut perm = Vec::with_capacity(costs.len());
    gen_perm_scores(costs, &mut perm, 0, &mut scores);
    scores
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let costs = parse(s)?;
    let score = all_scores(&costs).into_iter().max().context("No guests")?;
    Ok(score.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let mut costs = parse(s)?;
    let guest_count = costs.len() + 1;
    for guest in costs.iter_mut() {
        guest.push(0);
    }
    costs.push(vec![0; guest_count]);
    let score = all_scores(&costs).into_iter().max().context("No guests")?;
    Ok(score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";

    #[test]
    fn test_ex() {
        let costs = parse(EX).unwrap();
        let scores = all_scores(&costs);
        assert_eq!(scores.into_iter().max(), Some(330));
    }
}
