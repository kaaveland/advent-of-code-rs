use anyhow::{Context, Result};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::iter::empty;
use std::ops::RangeInclusive;
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

#[derive(Eq, PartialEq, Debug)]
enum LineType {
    Vertical,
    Horizontal,
    Sloped,
}

impl<'a> Line {
    fn straight(&self) -> bool {
        self.source.0 == self.dest.0 || self.source.1 == self.dest.1
    }

    // Line between points source and dest best understood as a collection of N cells
    // at t = 0 we are in source and at t = N we are at dest
    // The dir is what what we add to a point to move 1 step closer to dest
    // So the equation for y is defined for all 0 <= t <= N and is:
    // y_t = y_source + t * dir_y
    // x_t = x_source + t * dir_x
    fn dir(&self) -> (i32, i32) {
        let v = (self.dest.0 - self.source.0, self.dest.1 - self.source.1);
        (v.0 / max(1, v.0.abs()), v.1 / max(1, v.1.abs()))
    }
    // We can use the dir to find N easily: We insert y_dest = y_source + t * dir_y when dir_y != 0
    // and x_dest = x_source + t * dir_x when dir_x != 0 and solve for t
    fn len(&self) -> usize {
        let dir = self.dir();
        if dir.0 != 0 {
            // x_dest = x_source + N * dir_x => (x_dest - x_source) / dir_x = N
            ((self.dest.0 - self.source.0) / dir.0) as usize
        } else {
            // y_dest = y_source + N * dir_y => (y_dest - y_source) / dir_y = N
            ((self.dest.1 - self.source.1) / dir.1) as usize
        }
    }

    fn kind(&self) -> LineType {
        use LineType::*;
        match self.dir() {
            (0, _) => Vertical,
            (_, 0) => Horizontal,
            _ => Sloped,
        }
    }

    fn flip(&self) -> Line {
        Line {
            source: (self.source.1, self.source.0),
            dest: (self.dest.1, self.dest.0),
        }
    }

    fn intersection(&self, other: &Line) -> Points {
        use LineType::*;

        match (self.kind(), other.kind()) {
            (Horizontal, Horizontal) if self.source.1 == other.source.1 => {
                let y = self.source.1;
                // Let's find the leftmost point that could possibly be in both
                let x_s = max(
                    min(self.source.0, self.dest.0),
                    min(other.source.0, other.dest.0),
                );
                // The rightmost point that could possibly be in both
                let x_d = min(
                    max(self.source.0, self.dest.0),
                    max(other.source.0, other.dest.0),
                );
                if x_s <= x_d {
                    Points::Horizontally(y, x_s..=x_d)
                } else {
                    // Non-overlapping
                    Points::Empty
                }
            }
            (Vertical, Vertical) if self.source.0 == other.source.0 => {
                if let Points::Horizontally(x, yr) = self.flip().intersection(&other.flip()) {
                    Points::Vertically(x, yr)
                } else {
                    Points::Empty
                }
            }
            (Vertical, Horizontal) => {
                // Requirement: my x coordinate is in other xmin..=xmax and their y coordinate is in ymin..=ymax
                // If met: intersects in my x, their y
                let vrange = (
                    min(self.source.1, self.dest.1),
                    max(self.source.1, self.dest.1),
                );
                let vrange = (vrange.0)..=(vrange.1);
                let hrange = (
                    min(other.source.0, other.dest.0),
                    max(other.source.0, other.dest.0),
                );
                let hrange = (hrange.0)..=(hrange.1);
                if vrange.contains(&other.source.1) && hrange.contains(&self.source.0) {
                    Points::Single(self.source.0, other.source.1)
                } else {
                    Points::Empty
                }
            }
            (Horizontal, Vertical) => other.intersection(self),
            _ => Points::Empty,
        }
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

enum Points {
    Horizontally(i32, RangeInclusive<i32>),
    Vertically(i32, RangeInclusive<i32>),
    Single(i32, i32),
    Empty,
}

impl Points {
    fn count(&mut self) -> usize {
        use Points::*;
        match self {
            Horizontally(_, r) => r.count(),
            Vertically(_, r) => r.count(),
            Single(_, _) => 1,
            Empty => 0,
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

pub fn part_1(input: &str) -> Result<String> {
    let lines = parse_lines(input)?;
    let lines = lines
        .into_iter()
        .filter(|line| line.straight())
        .collect_vec();
    let mut pointset = HashSet::default();
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let intersection = lines[i].intersection(&lines[j]);
            match intersection {
                Points::Horizontally(y, xr) => pointset.extend(xr.map(|x| (x, y))),
                Points::Vertically(x, yr) => pointset.extend(yr.map(|y| (x, y))),
                Points::Single(x, y) => {
                    pointset.insert((x, y));
                }
                Points::Empty => {}
            }
        }
    }
    let sol = pointset.len();
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let lines = parse_lines(input)?;
    let sol = solve(&lines);
    Ok(format!("{sol}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_horizontal_overlapping() {
        let left = Line {
            source: (4, 0),
            dest: (2, 0),
        };
        let right = Line {
            source: (2, 0),
            dest: (4, 0),
        };
        let mut intersect = left.intersection(&right);
        assert_eq!(intersect.count(), 3);
        assert_eq!(right.intersection(&left).count(), 3);
        let right = Line {
            source: (0, 0),
            dest: (2, 0),
        };
        assert_eq!(right.intersection(&left).count(), 1);
        let left = Line {
            source: (3, 0),
            dest: (9, 0),
        };
        assert_eq!(right.intersection(&left).count(), 0);
        let left = Line {
            source: (0, 1),
            dest: (2, 1),
        };
        assert_eq!(right.intersection(&left).count(), 0);
    }

    #[test]
    fn test_vertical_overlapping() {
        let left = Line {
            source: (0, 4),
            dest: (0, 2),
        };
        let right = Line {
            source: (0, 2),
            dest: (0, 4),
        };
        let mut intersect = left.intersection(&right);
        assert_eq!(intersect.count(), 3);
        assert_eq!(right.intersection(&left).count(), 3);
        let right = Line {
            source: (0, 0),
            dest: (0, 2),
        };
        assert_eq!(right.intersection(&left).count(), 1);
        let left = Line {
            source: (0, 3),
            dest: (0, 9),
        };
        assert_eq!(right.intersection(&left).count(), 0);
        let left = Line {
            source: (1, 0),
            dest: (1, 2),
        };
        assert_eq!(right.intersection(&left).count(), 0);
    }

    #[test]
    fn test_vertical_horizontal() {
        let left = Line {
            source: (0, 0),
            dest: (4, 0),
        };
        let right = Line {
            source: (0, 0),
            dest: (0, 4),
        };
        assert_eq!(left.intersection(&right).count(), 1);
        assert_eq!(right.intersection(&left).count(), 1);
        let right = Line {
            source: (-2, 0),
            dest: (-2, 4),
        };
        assert_eq!(left.intersection(&right).count(), 0);
        assert_eq!(right.intersection(&left).count(), 0);
        let right = Line {
            source: (2, -2),
            dest: (2, 2),
        };
        assert_eq!(left.intersection(&right).count(), 1);
        assert_eq!(right.intersection(&left).count(), 1);
    }

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
