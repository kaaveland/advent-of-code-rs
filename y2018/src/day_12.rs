use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as Map;
use fxhash::FxHashSet as Set;

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    pots: Map<isize, bool>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rule {
    pattern: Vec<bool>,
    result: bool,
}

fn parse_state(s: &str) -> State {
    let mut pots = Map::default();
    for (i, c) in s.split_once(": ").unwrap().1.chars().enumerate() {
        pots.insert(i as isize, c == '#');
    }
    State { pots }
}
fn parse_rule(s: &str) -> Option<Rule> {
    let (pattern, result) = s.split_once(" => ")?;
    let pattern = pattern.chars().map(|c| c == '#').collect();
    let result = result.chars().next()? == '#';
    Some(Rule { pattern, result })
}

fn parse(s: &str) -> Option<(State, Vec<Rule>)> {
    let mut lines = s.lines();
    let state = parse_state(lines.next()?);
    let rules = lines.filter_map(parse_rule).collect();
    Some((state, rules))
}

fn next_state(state: &State, rules: &[Rule]) -> State {
    let mut pots = Map::default();
    let active: Set<_> = state
        .pots
        .keys()
        .copied()
        .flat_map(|i| i - 2..=i + 2)
        .collect();
    for i in active {
        let pattern = (i - 2..=i + 2)
            .map(|j| state.pots.get(&j).copied().unwrap_or(false))
            .collect::<Vec<_>>();
        for rule in rules {
            if rule.pattern == pattern {
                pots.insert(i, rule.result);
                break;
            }
        }
    }
    State { pots }
}

fn iterate(s: &str, times: usize) -> Option<State> {
    let (mut state, rules) = parse(s)?;
    for _ in 0..times {
        state = next_state(&state, &rules);
    }
    Some(state)
}

fn sum_pots(state: &State) -> isize {
    state.pots.iter().filter(|(_, &v)| v).map(|(&i, _)| i).sum()
}

pub fn part_1(s: &str) -> Result<String> {
    let state = iterate(s, 20).with_context(|| anyhow!("Unable to parse and iterate"))?;
    Ok(sum_pots(&state).to_string())
}

pub fn part_2(s: &str) -> Result<String> {
    let (mut state, rules) = parse(s).with_context(|| anyhow!("Unable to parse"))?;
    let mut diffs = Vec::new();
    for i in 0.. {
        let next = next_state(&state, &rules);
        let diff = sum_pots(&next) - sum_pots(&state);
        diffs.push(diff);
        if diffs.len() > 10 {
            let last_diffs = diffs[i - 10..i].iter().collect::<Set<_>>();
            if last_diffs.len() == 1 {
                let diff = diffs[i];
                let sum = sum_pots(&state) + (50_000_000_000 - i) as isize * diff;
                return Ok(sum.to_string());
            }
        }
        state = next;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

    #[test]
    fn test_example() {
        let next = iterate(EX, 1).unwrap();
        assert_eq!(next.pots.values().filter(|&&v| v).count(), 7);
    }
}
