use anyhow::Result;
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn part_1(input: &str) -> Result<String> {
    let n: usize = input
        .split("\n\n")
        .map(|block| {
            let hm: HashSet<_> = block.lines().flat_map(|line| line.chars()).collect();
            hm.len()
        })
        .sum();
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let n: usize = input
        .split("\n\n")
        .map(|block| {
            let mut hm: HashMap<_, usize> = HashMap::default();
            block
                .lines()
                .for_each(|line| line.chars().for_each(|ch| *hm.entry(ch).or_default() += 1));
            let len = block.lines().count();
            hm.iter().filter(|(_, &v)| v == len).count()
        })
        .sum();
    Ok(format!("{n}"))
}
