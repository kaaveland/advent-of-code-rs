use anyhow::{Context, Result};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::max;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Line {
    source: (i32, i32),
    dest: (i32, i32),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").context("Missing delim ->")?;
        let (xl, yl) = left.split_once(',').context("Missing delim ,")?;
        let (xr, yr) = right.split_once(',').context("Missing delim ,")?;
        let xl = xl.parse()?;
        let yl = yl.parse()?;
        let xr = xr.parse()?;
        let yr = yr.parse()?;
        Ok(Line {
            source: (xl, yl),
            dest: (xr, yr),
        })
    }
}

impl<'a> Line {
    fn straight(&self) -> bool {
        self.source.0 == self.dest.0 || self.source.1 == self.dest.1
    }
    fn dir(&self) -> (i32, i32) {
        let v = (self.dest.0 - self.source.0, self.dest.1 - self.source.1);
        (v.0 / max(1, v.0.abs()), v.1 / max(1, v.1.abs()))
    }
    fn iter(&'a self) -> LineIterator<'a> {
        LineIterator {
            line: self,
            point: self.source,
            dir: self.dir(),
            exhausted: false,
        }
    }
}

struct LineIterator<'a> {
    line: &'a Line,
    point: (i32, i32),
    dir: (i32, i32),
    exhausted: bool,
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else {
            let out = self.point;
            if out == self.line.dest {
                self.exhausted = true;
            }
            self.point = (self.point.0 + self.dir.0, self.point.1 + self.dir.1);
            Some(out)
        }
    }
}

fn parse_lines(input: &str) -> Result<Vec<Line>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Line::from_str)
        .collect()
}

fn solve(lines: &Vec<Line>) -> usize {
    let sets: Vec<_> = lines
        .into_par_iter()
        .enumerate()
        .map(|(i, line)| {
            let mut points: HashSet<(i32, i32)> = HashSet::default();
            let pointset: HashSet<_> = line.iter().collect();
            for other in &lines[i + 1..] {
                let next: HashSet<_> = other.iter().collect();
                points.extend(pointset.intersection(&next));
            }
            points
        })
        .collect();
    let mut points: HashSet<(i32, i32)> = HashSet::default();
    for pointset in sets {
        points.extend(pointset.iter())
    }
    points.len()
}

pub fn part_1(input: &str) -> Result<()> {
    let lines = parse_lines(input)?;
    let lines = lines
        .into_iter()
        .filter(|line| line.straight())
        .collect_vec();
    let sol = solve(&lines);
    println!("{sol}");
    Ok(())
}

pub fn part_2(input: &str) -> Result<()> {
    let lines = parse_lines(input)?;
    let sol = solve(&lines);
    println!("{sol}");
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let lines = parse_lines(EXAMPLE).unwrap();
        assert_eq!(solve(&lines), 12);
    }

    #[test]
    fn test_part1() {
        let lines = parse_lines(EXAMPLE).unwrap();
        let lines = lines
            .into_iter()
            .filter(|line| line.straight())
            .collect_vec();
        assert_eq!(solve(&lines), 5);
    }

    #[test]
    fn test_parse() {
        let lines = parse_lines(EXAMPLE).unwrap();
        assert_eq!(
            lines[0],
            Line {
                source: (0, 9),
                dest: (5, 9)
            }
        );
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn test_iterator() {
        let line = Line {
            source: (0, 9),
            dest: (5, 9),
        };
        let points: Vec<_> = line.iter().collect();
        assert_eq!(points, vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)]);
    }

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
}
