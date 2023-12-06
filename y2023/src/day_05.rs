use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use std::ops::Range;
use std::str::FromStr;

fn posint(s: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(s)
}
fn seeds(s: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), separated_list1(space1, posint))(s)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Conversion {
    source: i64,
    dest: i64,
    length: i64,
}

#[derive(Eq, PartialEq, Debug)]
struct Mapping<'a> {
    source: &'a str,
    dest: &'a str,
    converters: Vec<Conversion>,
}
fn parse_conversion(s: &str) -> IResult<&str, Conversion> {
    let (s, (dest, source, length)) = tuple((
        terminated(posint, space1),
        terminated(posint, space1),
        posint,
    ))(s)?;
    Ok((
        s,
        Conversion {
            source,
            dest,
            length,
        },
    ))
}

fn parse_map(s: &str) -> IResult<&str, Mapping> {
    let name_line = terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n"));
    let (s, ((source, dest), converters)) =
        pair(name_line, separated_list1(tag("\n"), parse_conversion))(s)?;
    Ok((
        s,
        Mapping {
            source,
            dest,
            converters,
        },
    ))
}

#[derive(Eq, PartialEq, Debug)]
struct Task<'a> {
    seeds: Vec<i64>,
    mappings: Vec<Mapping<'a>>,
}

fn parse(s: &str) -> IResult<&str, Task> {
    let (s, seeds) = terminated(seeds, tag("\n\n"))(s)?;
    let (s, mut mappings) = separated_list1(tag("\n\n"), parse_map)(s)?;
    mappings
        .iter_mut()
        .for_each(|m| m.converters.sort_by_key(|c| c.source));
    Ok((s, Task { seeds, mappings }))
}

impl Mapping<'_> {
    fn convert_range(&self, mut range: Range<i64>) -> Vec<Range<i64>> {
        // Assumptions: 1) no converters in a mapping overlap 2) converters are in order of lowest/leftmost source
        let mut converted = vec![];
        for cv in self.converters.iter() {
            let cv_range = cv.source..cv.source + cv.length;
            let delta = cv.dest - cv.source;
            let intersect = cv_range.start.max(range.start)..cv_range.end.min(range.end);
            if intersect.end > intersect.start {
                // Nothing can match what's left of `intersect` anymore.
                if range.start < intersect.start {
                    let leftover = range.start..intersect.start;
                    converted.push(leftover);
                }
                // Translate the intersect
                let mapped = (intersect.start + delta)..(intersect.end + delta);
                converted.push(mapped);
                // Everything left of intersect is already removed, or translated
                range.start = intersect.end;
            }
        }
        // There is a case now where range could be to the right of the last cv:
        if range.end > range.start {
            converted.push(range);
        }
        converted
    }
}

fn map_seed(seed: i64, mappings: &[Mapping]) -> i64 {
    map_seed_range(&(seed..seed + 1), mappings)[0].start
}

fn map_seed_range(range: &Range<i64>, mappings: &[Mapping]) -> Vec<Range<i64>> {
    let mut stage = "seed";
    let mut vecs = vec![range.clone()];
    for mapping in mappings {
        assert_eq!(stage, mapping.source);
        stage = mapping.dest;
        vecs = vecs
            .into_iter()
            .flat_map(|r| mapping.convert_range(r))
            .collect();
    }
    vecs
}

pub fn part_1(input: &str) -> Result<String> {
    let (_, task) = parse(input).map_err(|err| anyhow!("{err}"))?;
    let n = task
        .seeds
        .iter()
        .map(|seed| map_seed(*seed, &task.mappings))
        .min()
        .unwrap_or(0);
    Ok(n.to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let (_, task) = parse(input).map_err(|err| anyhow!("{err}"))?;
    let results: Result<Vec<_>> = task
        .seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let fst = *chunk.next().context("Empty chunk")?;
            let len = *chunk.next().context("Empty chunk")?;
            map_seed_range(&(fst..fst + len), &task.mappings)
                .into_iter()
                .map(|r| r.start)
                .min()
                .context("Empty output")
        })
        .collect();
    results
        .map(|n| n.iter().min().copied().unwrap_or(0))
        .map(|n| n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_p1() {
        let ans = part_1(EX).unwrap();
        assert_eq!(ans, "35".to_string());
    }

    #[test]
    fn test_p2() {
        let ans = part_2(EX).unwrap();
        assert_eq!(ans, "46".to_string());
    }

    #[test]
    fn test_parse() {
        let (_, task) = parse(EX).unwrap();
        assert_eq!(task.seeds[0], 79);
        assert_eq!(task.mappings[6].source, "humidity");
    }
    #[test]
    fn test_parse_mapping() {
        let ex = "seed-to-soil map:
50 98 2
52 50 48";
        let (_, mapping) = parse_map(ex).unwrap();
        assert_eq!(
            mapping,
            Mapping {
                source: "seed",
                dest: "soil",
                converters: vec![
                    Conversion {
                        dest: 50,
                        source: 98,
                        length: 2
                    },
                    Conversion {
                        dest: 52,
                        source: 50,
                        length: 48
                    }
                ]
            }
        );
    }
}
