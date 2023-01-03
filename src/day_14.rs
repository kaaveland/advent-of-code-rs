use anyhow::{Context, Result};
use fxhash::FxHashMap as HashMap;

type Pair = (char, char);
type PairMap = HashMap<Pair, usize>;
type InsertionRules = HashMap<Pair, (Pair, Pair)>;

#[derive(Eq, PartialEq, Debug)]
struct PolymerTask {
    initial: PairMap,
    count: HashMap<char, usize>,
    rules: InsertionRules,
}

fn step_polymers(
    pairmap: &PairMap,
    rules: &InsertionRules,
    counter: &mut HashMap<char, usize>,
) -> PairMap {
    let mut result = PairMap::default();
    for (lookfor, insert) in rules.iter() {
        let count = *pairmap.get(lookfor).unwrap_or(&0);
        if count > 0 {
            *result.entry(insert.0).or_default() += count;
            *result.entry(insert.1).or_default() += count;
            *counter.entry(insert.0 .1).or_default() += count;
        }
    }
    result
}

fn iterate_polymers(
    initial: &PairMap,
    rules: &InsertionRules,
    iterations: usize,
    out: &mut HashMap<char, usize>,
) -> PairMap {
    let mut pairmap = initial.clone();
    for _ in 0..iterations {
        pairmap = step_polymers(&pairmap, rules, out);
    }
    pairmap
}

fn parse(input: &str) -> Result<PolymerTask> {
    let mut lines = input.lines();
    let first = lines.next().context("Empty input")?;
    let mut count = HashMap::default();
    for ch in first.chars() {
        *count.entry(ch).or_default() += 1;
    }
    let mut initial: PairMap = PairMap::default();
    for (first, second) in first.chars().zip(first.chars().skip(1)) {
        *initial.entry((first, second)).or_default() += 1;
    }
    let mut rules = HashMap::default();
    for line in lines.filter(|line| !line.is_empty()) {
        let (pair, rule) = line.split_once(" -> ").context("Missing rule")?;
        assert_eq!(pair.chars().count(), 2);
        assert_eq!(rule.chars().count(), 1);
        let pair = pair.chars().take(2).collect::<Vec<_>>();
        let pair = (pair[0], pair[1]);
        let rule = rule.chars().next().unwrap();
        let generated = ((pair.0, rule), (rule, pair.1));
        rules.insert(pair, generated);
    }
    Ok(PolymerTask {
        initial,
        count,
        rules,
    })
}

fn solve(input: &str, iterations: usize) -> Result<usize> {
    let mut input = parse(input)?;
    let _pairmap = iterate_polymers(&input.initial, &input.rules, iterations, &mut input.count);
    let max = *input.count.values().max().unwrap_or(&0);
    let min = *input.count.values().min().unwrap_or(&0);
    let score = max - min;
    Ok(score)
}

pub fn part_1(input: &str) -> Result<String> {
    solve(input, 10).map(|score| format!("{score}"))
}

pub fn part_2(input: &str) -> Result<String> {
    solve(input, 40).map(|score| format!("{score}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let score = solve(EXAMPLE, 10).unwrap();
        assert_eq!(score, 1588);
    }

    #[test]
    fn test_iterate_once() {
        let mut result = parse(EXAMPLE).unwrap();
        let step_1 = step_polymers(&result.initial, &result.rules, &mut result.count);
        let expect: PairMap = [
            (('N', 'C'), 1),
            (('C', 'N'), 1),
            (('N', 'B'), 1),
            (('B', 'C'), 1),
            (('C', 'H'), 1),
            (('H', 'B'), 1),
        ]
        .into_iter()
        .collect();
        assert_eq!(step_1, expect);
    }

    #[test]
    fn test_iterate_twice() {
        let mut result = parse(EXAMPLE).unwrap();
        let step_1 = step_polymers(&result.initial, &result.rules, &mut result.count);
        let step_2 = step_polymers(&step_1, &result.rules, &mut result.count);
        let expect: PairMap = [
            (('N', 'B'), 2),
            (('B', 'C'), 2),
            (('C', 'C'), 1),
            (('C', 'N'), 1),
            (('B', 'B'), 2),
            (('C', 'B'), 2),
            (('B', 'H'), 1),
            (('H', 'C'), 1),
        ]
        .into_iter()
        .collect();
        assert_eq!(step_2, expect);
    }
    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result.initial.get(&('N', 'N')), Some(&1));
        assert_eq!(result.initial.get(&('N', 'C')), Some(&1));
        assert_eq!(result.initial.get(&('C', 'B')), Some(&1));
        assert_eq!(
            result.rules.get(&('C', 'H')),
            Some(&(('C', 'B'), ('B', 'H')))
        );
        assert_eq!(
            result.rules.get(&('N', 'N')),
            Some(&(('N', 'C'), ('C', 'N')))
        );
    }

    const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
}
