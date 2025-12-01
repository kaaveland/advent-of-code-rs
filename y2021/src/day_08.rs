use anyhow::{Context, Result};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn part_1(input: &str) -> Result<String> {
    let examples = parse(input)?;
    let sol: Vec<u8> = examples.iter().flat_map(solve).collect();
    let n = sol.iter().filter(|n| [1, 4, 7, 8].contains(n)).count();
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let examples = parse(input)?;
    let sol: Vec<Vec<u8>> = examples.iter().map(solve).collect();
    let n = sol.iter().fold(0, |s, n| {
        s + n
            .iter()
            .fold((1000, 0u32), |(f, s), n| (f / 10, (*n as u32) * f + s))
            .1
    });
    Ok(format!("{n}"))
}

type Example<'a> = (Vec<&'a [u8]>, Vec<&'a [u8]>);

fn parse(input: &str) -> Result<Vec<Example<'_>>> {
    let lines = input.lines().filter(|line| !line.is_empty());
    lines
        .map(|line| {
            let (left, right) = line.split_once(" | ").context("Missing delim |")?;
            Ok((
                left.split_ascii_whitespace()
                    .map(|s| s.as_bytes())
                    .collect(),
                right
                    .split_ascii_whitespace()
                    .map(|s| s.as_bytes())
                    .collect(),
            ))
        })
        .collect()
}

fn solve(example: &Example) -> Vec<u8> {
    let digit_sets: HashMap<_, _> = example
        .0
        .iter()
        .map(|dig| {
            let digset = dig.iter().cloned().collect::<HashSet<u8>>();
            (digset.len(), digset)
        })
        .collect();
    example
        .1
        .iter()
        .map(|dig| {
            let digset = dig.iter().cloned().collect::<HashSet<u8>>();
            let share_with_4 = digset.intersection(digit_sets.get(&4).unwrap()).count();
            let share_with_1 = digset.intersection(digit_sets.get(&2).unwrap()).count();
            match (digset.len(), share_with_4, share_with_1) {
                (2, _, _) => 1u8,
                (3, _, _) => 7u8,
                (4, _, _) => 4u8,
                (7, _, _) => 8u8,
                (5, 2, _) => 2u8,
                (5, 3, 1) => 5u8,
                (5, 3, 2) => 3u8,
                (6, 4, _) => 9u8,
                (6, 3, 1) => 6u8,
                (6, 3, 2) => 0u8,
                _ => panic!("Unhandled case for {dig:?} {digset:?} {share_with_4} {share_with_1} {digit_sets:?}"),
            }
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let examples = parse(EXAMPLE).unwrap();
        assert_eq!(solve(&examples[0]), vec![8u8, 3u8, 9u8, 4u8]);
    }

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
}
