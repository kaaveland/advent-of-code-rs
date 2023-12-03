use anyhow::{anyhow, Result};
use fxhash::FxHashSet as HashSet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use std::str::FromStr;

type Vec2 = [i64; 2];
#[derive(Eq, PartialEq, Debug)]
struct Point2 {
    pos: Vec2,
    vel: Vec2,
}

impl Point2 {
    fn step(&self, steps: i64) -> Self {
        Point2 {
            pos: [
                self.pos[0] + self.vel[0] * steps,
                self.pos[1] + self.vel[1] * steps,
            ],
            vel: self.vel,
        }
    }
}

fn parse_num(s: &str) -> IResult<&str, i64> {
    let posint = map_res(digit1, FromStr::from_str);
    let neg = map(posint, |n: i64| -n);
    alt((map_res(digit1, FromStr::from_str), preceded(tag("-"), neg)))(s)
}

fn parse_vec2(s: &str) -> IResult<&str, Vec2> {
    let inner = separated_pair(
        delimited(space0, parse_num, space0),
        tag(","),
        delimited(space0, parse_num, space0),
    );
    let (s, p) = delimited(tag("<"), inner, tag(">"))(s)?;
    Ok((s, [p.0, p.1]))
}

fn parse_point2(s: &str) -> IResult<&str, Point2> {
    let pos = preceded(tag("position="), parse_vec2);
    let vel = preceded(tag("velocity="), parse_vec2);
    map(separated_pair(pos, space0, vel), |(pos, vel)| Point2 {
        pos,
        vel,
    })(s)
}

fn parse_points(s: &str) -> Result<Vec<Point2>> {
    let (_, points) =
        separated_list1(tag("\n"), parse_point2)(s).map_err(|err| anyhow!("{err}"))?;
    Ok(points)
}

fn step(points: &[Point2], steps: i64) -> Vec<Point2> {
    points.iter().map(|p| p.step(steps)).collect()
}

fn bounds(points: &[Point2]) -> [[i64; 2]; 2] {
    let [xmin, ymin, xmax, ymax] = points.iter().map(|p| p.pos).fold(
        [i64::MAX, i64::MAX, i64::MIN, i64::MIN],
        |[xmin, ymin, xmax, ymax], [x, y]| [xmin.min(x), ymin.min(y), xmax.max(x), ymax.max(y)],
    );
    [[xmin, xmax], [ymin, ymax]]
}
fn area(points: &[Point2]) -> i64 {
    let [[xmin, xmax], [ymin, ymax]] = bounds(points);
    (xmax - xmin) * (ymax - ymin)
}

fn render(points: &[Point2]) -> String {
    let [[xmin, xmax], [ymin, ymax]] = bounds(points);
    let p: HashSet<_> = points.iter().map(|p| p.pos).collect();
    let mut out = String::new();
    for row in ymin..=ymax {
        for col in xmin..=xmax {
            out.push(if p.contains(&[col, row]) { '#' } else { ' ' });
        }
        out.push('\n');
    }
    out
}

fn solve<F, R>(input: &str, f: F) -> Result<R>
where
    F: Fn(i32, &[Point2]) -> Result<R>,
{
    let mut points = parse_points(input)?;
    let mut min_area = area(&points);

    for i in 0.. {
        let next = step(&points, 1);
        let next_area = area(&next);
        if next_area < min_area {
            min_area = next_area;
            points = next;
        } else {
            return f(i, &points);
        }
    }
    unreachable!()
}

pub fn part_1(input: &str) -> Result<String> {
    solve(input, |_, points| Ok(render(points)))
}

pub fn part_2(input: &str) -> Result<String> {
    solve(input, |time, _| Ok(time.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let ex = "position=< 2, -4> velocity=< 2,  2>";
        let p = parse_point2(ex).unwrap().1;
        assert_eq!(
            p,
            Point2 {
                pos: [2, -4],
                vel: [2, 2]
            }
        );
    }
}
