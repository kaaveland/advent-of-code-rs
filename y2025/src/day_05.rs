use anyhow::Context;
use itertools::Itertools;

#[derive(Debug)]
struct ElfInventorySystem {
    fresh_ranges: Vec<Range>,
    available_ids: Vec<u64>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Range(u64, u64);

impl Range {
    fn contains(&self, x: &u64) -> bool {
        *x <= self.1 && *x >= self.0
    }
    fn len(&self) -> u64 {
        self.1 - self.0 + 1
    }
    fn positive(&self) -> bool {
        self.1 >= self.0
    }
    fn overlap(&self, other: &Self) -> Self {
        Range(self.0.max(other.0), self.1.min(other.1))
    }
}

fn parse(s: &str) -> anyhow::Result<ElfInventorySystem> {
    let (ranges, ids) = s.split_once("\n\n").context("Invalid input")?;
    let fresh_ranges: Result<Vec<_>, anyhow::Error> = ranges
        .lines()
        .filter(|l| !l.is_empty())
        .map(|r| {
            let (start, end) = r.split_once('-').context("Input missing dash")?;
            let start: u64 = start.parse()?;
            let end: u64 = end.parse()?;
            Ok(Range(start, end))
        })
        .collect();
    let ids: Result<Vec<_>, _> = ids
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect();
    Ok(ElfInventorySystem {
        fresh_ranges: fresh_ranges?,
        available_ids: ids?,
    })
}

fn count_fresh_ids(input: &ElfInventorySystem) -> usize {
    input
        .available_ids
        .iter()
        .filter(|id| input.fresh_ranges.iter().any(|range| range.contains(id)))
        .count()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let inventory = parse(s)?;
    Ok(format!("{}", count_fresh_ids(&inventory)))
}

fn merge_adjacent_ranges(ranges: &[Range]) -> Vec<Range> {
    // Invariant: maintain in sorted by start order
    let mut out: Vec<Range> = vec![];
    for new in ranges.iter().sorted().copied() {
        if let Some(existing) = out.pop() {
            let shared = new.overlap(&existing);
            if shared.positive() {
                let merged = Range(existing.0, new.1.max(existing.1));
                out.push(merged);
            } else {
                // no overlap
                out.push(existing);
                out.push(new);
            }
        } else {
            out.push(new);
        }
    }
    out
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let inventory = parse(s)?;
    let ranges = merge_adjacent_ranges(&inventory.fresh_ranges);
    let fresh: u64 = ranges.into_iter().map(|range| range.len()).sum();
    Ok(format!("{fresh}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1() {
        let ex = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let inventory = parse(ex).unwrap();
        assert_eq!(count_fresh_ids(&inventory), 3);
    }
}
