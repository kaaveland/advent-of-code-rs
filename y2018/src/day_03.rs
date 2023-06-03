use anyhow::{anyhow, Result};
use fxhash::FxHashSet as HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Rect {
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
}

impl From<Claim> for Rect {
    fn from(value: Claim) -> Self {
        Rect {
            min_x: value.x,
            max_x: value.x + value.width,
            min_y: value.y,
            max_y: value.y + value.height,
        }
    }
}

impl Rect {
    fn points(self) -> impl Iterator<Item = (u32, u32)> {
        (self.min_x..self.max_x).flat_map(move |x| (self.min_y..self.max_y).map(move |y| (x, y)))
    }
}

fn intersect<U, T>(left: U, right: T) -> Rect
where
    T: Into<Rect>,
    U: Into<Rect>,
{
    let left = left.into();
    let right = right.into();
    Rect {
        min_x: left.min_x.max(right.min_x),
        max_x: left.max_x.min(right.max_x),
        min_y: left.min_y.max(right.min_y),
        max_y: left.max_y.min(right.max_y),
    }
}

fn points_in_intersections<T>(rects: &[T]) -> HashSet<(u32, u32)>
where
    T: Into<Rect> + Copy,
{
    let idx = (0..rects.len()).flat_map(|i| (0..i).map(move |j| (i, j)));
    let rects = idx.flat_map(|(i, j)| intersect(rects[i], rects[j]).points());
    rects.collect()
}

fn parse_claim(input: &str) -> IResult<&str, Claim> {
    let extract = tuple((
        preceded(char('#'), map_res(digit1, str::parse)),
        preceded(tag(" @ "), map_res(digit1, str::parse)),
        preceded(char(','), map_res(digit1, str::parse)),
        preceded(tag(": "), map_res(digit1, str::parse)),
        preceded(char('x'), map_res(digit1, str::parse)),
    ));
    map(extract, |(id, x, y, width, height)| Claim {
        id,
        x,
        y,
        width,
        height,
    })(input)
}

fn parse_claims(input: &str) -> Result<Vec<Claim>> {
    separated_list1(char('\n'), parse_claim)(input)
        .map_err(|e| anyhow!("{e}"))
        .map(|(_, claims)| claims)
}

pub fn part_1(input: &str) -> Result<String> {
    let claims = parse_claims(input)?;
    let points = points_in_intersections(&claims);
    Ok(points.len().to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let claims = parse_claims(input)?;
    let points = points_in_intersections(&claims);
    claims
        .into_iter()
        .find(|claim| {
            let rect = Rect::from(*claim);
            let overlaps = rect.points().any(|point| points.contains(&point));
            !overlaps
        })
        .map(|claim| claim.id.to_string())
        .ok_or_else(|| anyhow!("No claim found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: [Claim; 3] = [
        Claim {
            id: 1,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        },
        Claim {
            id: 2,
            x: 3,
            y: 1,
            width: 4,
            height: 4,
        },
        Claim {
            id: 3,
            x: 5,
            y: 5,
            width: 2,
            height: 2,
        },
    ];

    #[test]
    fn test_points_in_example_claims_intersections() {
        let points = points_in_intersections(&EXAMPLES);
        assert_eq!(points.len(), 4);
        assert!(points.contains(&(3, 3)));
        assert!(points.contains(&(3, 4)));
        assert!(points.contains(&(4, 3)));
        assert!(points.contains(&(4, 4)));
    }

    #[test]
    fn parse_examples() {
        let x = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";
        let (_, claims) = separated_list1(char('\n'), parse_claim)(x).unwrap();
        assert_eq!(claims, EXAMPLES);
    }
}
