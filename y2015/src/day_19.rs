use anyhow::{anyhow, Context};
use fxhash::FxHashSet;
use std::cmp::Reverse;

fn parse(s: &str) -> anyhow::Result<(Vec<(&str, &str)>, &str)> {
    let (replacements, molecule) = s.split_once("\n\n").context("Invalid input")?;

    let replacements: anyhow::Result<Vec<_>> = replacements
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").context("Invalid input")?;
            Ok((from, to))
        })
        .collect();

    Ok((replacements?, molecule.trim()))
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let mut seen = FxHashSet::default();
    let (replacements, molecule) = parse(s)?;

    for (from, to) in replacements {
        for (start_at, _) in molecule.match_indices(from) {
            let mut after = String::with_capacity(molecule.len() + to.len() - from.len());
            after.push_str(&molecule[..start_at]);
            after.push_str(to);
            after.push_str(&molecule[(start_at + from.len())..]);
            seen.insert(after);
        }
    }

    Ok(seen.len().to_string())
}

fn greedy_backtracking_search(
    molecule: &str,
    transformations: &[(&str, &str)],
    steps: usize,
) -> Option<usize> {
    if molecule == "e" {
        Some(steps)
    } else {
        for (from, to) in transformations {
            for (start_at, _) in molecule.match_indices(to) {
                let mut new_molecule = String::with_capacity(molecule.len());
                new_molecule.push_str(&molecule[..start_at]);
                new_molecule.push_str(from);
                new_molecule.push_str(&molecule[(start_at + to.len())..]);
                if let Some(answer) =
                    greedy_backtracking_search(new_molecule.as_str(), transformations, steps + 1)
                {
                    return Some(answer);
                }
            }
        }
        None
    }
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let (mut replacements, molecule) = parse(s)?;
    replacements.sort_by_key(|(from, to)| Reverse(to.len() - from.len()));
    if let Some(answer) = greedy_backtracking_search(molecule, &replacements, 0) {
        Ok(answer.to_string())
    } else {
        Err(anyhow!("Solution not found"))
    }
}
