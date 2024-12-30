use anyhow::anyhow;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{map_res, recognize};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

type Point = [i32; 4];

fn parse(s: &str) -> anyhow::Result<Vec<Point>> {
    fn int(s: &str) -> IResult<&str, i32> {
        map_res(
            alt((recognize(preceded(char('-'), digit1)), digit1)),
            |n: &str| n.parse(),
        )(s)
    }

    separated_list1(
        tag("\n"),
        map_res(
            preceded(space0, separated_list1(tag(","), int)),
            |v: Vec<i32>| v.try_into(),
        ),
    )(s)
    .map_err(|err| anyhow!("{err}"))
    .map(|(_, v)| v)
}

#[inline]
fn manhattan<const N: usize>(left: &[i32; N], right: &[i32; N]) -> i32 {
    (0..N).map(|i| (left[i] - right[i]).abs()).sum()
}

fn constellations(points: &[Point]) -> usize {
    let mut unassigned = points.iter().collect_vec();
    let mut found = 0;
    // Choose any arbitrary point to be start of a new constellation
    while let Some(current) = unassigned.pop() {
        found += 1;
        let mut neighbours = vec![current];
        // Now we just need to remove all neighbours from unassigned
        while let Some(check) = neighbours.pop() {
            // Grab all the points that are close to `check`
            let friends = unassigned
                .iter()
                .filter(|&&friend| manhattan(check, friend) <= 3)
                .copied()
                .collect_vec();
            unassigned.retain(|point| !friends.contains(point));
            neighbours.extend(friends);
        }
    }
    found
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let v = parse(s)?;
    Ok(constellations(&v).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let v = parse(
            "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
",
        )
        .unwrap();
        assert_eq!(constellations(&v), 4);
    }

    #[test]
    fn test_parse() {
        let v = parse(
            " 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0
",
        )
        .unwrap();
        assert_eq!(v.len(), 8);
    }
}
