use anyhow::{anyhow, Result};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(s: &str) -> IResult<&str, Point> {
        fn parse_i32(s: &str) -> IResult<&str, i32> {
            map_res(digit1, FromStr::from_str)(s)
        }
        fn parse_neg_i32(s: &str) -> IResult<&str, i32> {
            map(preceded(char('-'), parse_i32), |n| -n)(s)
        }
        fn num(s: &str) -> IResult<&str, i32> {
            alt((parse_neg_i32, parse_i32))(s)
        }
        map(separated_pair(num, tag(", "), num), |(x, y)| Point { x, y })(s)
    }

    fn manhattan(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn parse_input(s: &str) -> Result<Vec<Point>> {
    Ok(separated_list1(char('\n'), Point::parse)(s)
        .map_err(|e| anyhow!("{e}"))?
        .1)
}

fn upper_left(points: &[Point]) -> Point {
    let (x, y) = points.iter().fold((i32::MAX, i32::MAX), |(xmin, ymin), p| {
        (xmin.min(p.x), ymin.min(p.y))
    });
    Point { x, y }
}

fn lower_right(points: &[Point]) -> Point {
    let (x, y) = points.iter().fold((i32::MIN, i32::MIN), |(xmax, ymax), p| {
        (xmax.max(p.x), ymax.max(p.y))
    });
    Point { x, y }
}

fn perimeter(points: &[Point]) -> impl Iterator<Item = Point> {
    let topleft = upper_left(points);
    let botright = lower_right(points);
    let top = (topleft.x..=botright.x).map(move |x| Point { x, y: topleft.y });
    let left = (topleft.y..=botright.y).map(move |y| Point { x: topleft.x, y });
    let bot = (topleft.x..=botright.x).map(move |x| Point { x, y: botright.y });
    let right = (topleft.y..=botright.y).map(move |y| Point { x: botright.x, y });
    top.chain(left).chain(bot).chain(right)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum AssignedTile {
    OwnedBy(Point, i32),
    Draw(i32),
}

impl AssignedTile {
    fn distance(self) -> i32 {
        use AssignedTile::*;
        match self {
            Draw(d) => d,
            OwnedBy(_, d) => d,
        }
    }
}

fn max_finite_area(points: &[Point]) -> i32 {
    use AssignedTile::*;

    let perimeter: HashSet<Point> = perimeter(points).collect();
    let topleft = upper_left(points);
    let botright = lower_right(points);

    let mut assignment = HashMap::default();

    for (x, y) in (topleft.x..=botright.x).cartesian_product(topleft.y..=botright.y) {
        let here = Point { x, y };
        for point in points {
            let dist = point.manhattan(&here);
            if !assignment.contains_key(&here) {
                assignment.insert(here, OwnedBy(*point, dist));
            } else {
                let closest = assignment.get(&here).map(|a| a.distance()).unwrap();
                match dist.cmp(&closest) {
                    Ordering::Less => {
                        assignment.insert(here, OwnedBy(*point, dist));
                    }
                    Ordering::Equal => {
                        assignment.insert(here, Draw(dist));
                    }
                    Ordering::Greater => {}
                };
            }
        }
    }

    let infinite_points: HashSet<_> = perimeter
        .iter()
        .map(|p| assignment.get(p).unwrap())
        .filter_map(|assignment| match assignment {
            OwnedBy(p, _) => Some(p),
            _ => None,
        })
        .collect();

    let valid_assignments = assignment
        .iter()
        .filter_map(|(_, assignment)| match assignment {
            OwnedBy(p, _) if !infinite_points.contains(p) => Some(p),
            _ => None,
        });

    let mut counts = HashMap::default();
    for p in valid_assignments {
        *counts.entry(*p).or_default() += 1;
    }

    *counts.values().max().unwrap()
}

pub fn part_1(input: &str) -> Result<String> {
    parse_input(input).map(|points| max_finite_area(&points).to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let points = parse_input(input)?;
    let topleft = upper_left(&points);
    let botright = lower_right(&points);
    let within = (topleft.x..=botright.x)
        .cartesian_product(topleft.y..=botright.y)
        .map(|(x, y)| {
            points
                .iter()
                .map(|p| (x - p.x).abs() + (y - p.y).abs())
                .sum()
        })
        .filter(|dist: &i32| *dist < 10_000)
        .count();
    Ok(within.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    const EXAMPLE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

    #[test]
    fn parse_point() {
        assert_eq!(Point::parse("3, 1").unwrap().1, Point { x: 3, y: 1 });
        assert_eq!(Point::parse("-3, -1").unwrap().1, Point { x: -3, y: -1 });
    }

    #[test]
    fn parse_example() {
        let ex = parse_input(EXAMPLE).unwrap();
        assert_eq!(ex.len(), 6);
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(Point { x: 0, y: 0 }.manhattan(&Point { x: 3, y: 1 }), 4);
    }

    #[test]
    fn test_upper_left() {
        let points = parse_input(EXAMPLE).unwrap();
        assert_eq!(upper_left(&points), Point { x: 1, y: 1 });
    }

    #[test]
    fn test_upper_right() {
        let points = parse_input(EXAMPLE).unwrap();
        assert_eq!(lower_right(&points), Point { x: 8, y: 9 });
    }

    #[test]
    fn test_max_fin_area() {
        let points = parse_input(EXAMPLE).unwrap();
        assert_eq!(max_finite_area(&points), 17);
    }
}
