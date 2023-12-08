use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, separated_pair, terminated};
use nom::IResult;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Dir {
    L,
    R,
}

fn parse_dirs(s: &str) -> IResult<&str, Vec<Dir>> {
    let l = map(tag("L"), |_| Dir::L);
    let r = map(tag("R"), |_| Dir::R);
    let dir = alt((l, r));
    terminated(many1(dir), tag("\n"))(s)
}

struct Crossroads<'a> {
    at: &'a str,
    left: &'a str,
    right: &'a str,
}
fn parse_crossroad(s: &str) -> IResult<&str, Crossroads> {
    let dirs = separated_pair(alphanumeric1, tag(", "), alphanumeric1);
    let dirs = delimited(tag("("), dirs, tag(")"));
    let both = separated_pair(alphanumeric1, tag(" = "), dirs);
    map(both, |(at, (left, right))| Crossroads { at, left, right })(s)
}

fn parse(s: &str) -> IResult<&str, (Vec<Dir>, Vec<Crossroads>)> {
    let (s, (dirs, roads)) = pair(
        terminated(parse_dirs, tag("\n")),
        separated_list1(tag("\n"), parse_crossroad),
    )(s)?;
    Ok((s, (dirs, roads)))
}
fn assemble_map<'a>(roads: &'a [Crossroads]) -> HashMap<&'a str, (&'a str, &'a str)> {
    roads
        .iter()
        .map(|road| (road.at, (road.left, road.right)))
        .collect()
}

fn solve<'a>(
    mut place: &'a str,
    map: &HashMap<&'a str, (&'a str, &'a str)>,
    dirs: &[Dir],
    end: &dyn Fn(&str) -> bool,
) -> Result<usize> {
    for i in 0usize.. {
        let dir = dirs[i.rem_euclid(dirs.len())];
        let choices = map.get(place).context(anyhow!("{place}"))?;
        if dir == Dir::L {
            place = choices.0
        } else {
            place = choices.1
        };

        if end(place) {
            return Ok(i + 1);
        }
    }
    unreachable!()
}
pub fn part_1(s: &str) -> Result<String> {
    let (_, (dirs, roads)) = parse(s).map_err(|err| anyhow!("{err}"))?;
    let map = assemble_map(&roads);
    solve("AAA", &map, &dirs, &|place| place == "ZZZ").map(|n| n.to_string())
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b))
    }
    a
}

pub fn part_2(s: &str) -> Result<String> {
    let (_, (dirs, roads)) = parse(s).map_err(|err| anyhow!("{err}"))?;
    let map = assemble_map(&roads);
    let sols: Result<Vec<_>> = roads
        .iter()
        .map(|r| r.at)
        .filter(|r| r.ends_with("A"))
        .map(|place| solve(place, &map, &dirs, &|place| place.ends_with('Z')))
        .collect();
    let n = sols?.iter().copied().fold(1usize, |a, b| {
        let g = gcd(a, b);
        a * b / g
    });
    Ok(n.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap(), "6".to_string());
    }

    const EX2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EX2).unwrap(), "6".to_string());
    }
}
