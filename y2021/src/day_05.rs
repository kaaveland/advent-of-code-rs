use anyhow::{Context, Result};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use std::cmp::{max, min};
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
            source: if xl <= xr { (xl, yl) } else { (xr, yr) },
            dest: if xl <= xr { (xr, yr) } else { (xl, yl) },
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
enum LineType {
    Vertical,
    Horizontal,
    Sloped,
}

impl Line {
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
    fn len(&self) -> i32 {
        let dir = self.dir();
        if dir.0 != 0 {
            // x_dest = x_source + N * dir_x => (x_dest - x_source) / dir_x = N
            (self.dest.0 - self.source.0) / dir.0
        } else {
            // y_dest = y_source + N * dir_y => (y_dest - y_source) / dir_y = N
            (self.dest.1 - self.source.1) / dir.1
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
                let x_s = max(
                    min(self.source.0, self.dest.0),
                    min(other.source.0, other.dest.0),
                );
                let x_d = min(
                    max(self.source.0, self.dest.0),
                    max(other.source.0, other.dest.0),
                );
                if x_s <= x_d {
                    Points::Horizontally(y, x_s..=x_d)
                } else {
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
            (Sloped, Horizontal) => {
                let y = other.source.1;
                let xr = (
                    min(other.source.0, other.dest.0),
                    max(other.source.0, other.dest.0),
                );
                let xr = (xr.0)..=(xr.1);
                let t = (y - self.source.1) / self.dir().1;
                let x = self.source.0 + t * self.dir().0;
                if t >= 0 && t <= self.len() && xr.contains(&x) {
                    Points::Single(x, y)
                } else {
                    Points::Empty
                }
            }
            (Horizontal, Sloped) => other.intersection(self),
            (Sloped, Vertical) => {
                let x = other.source.0;
                let yr = (
                    min(other.source.1, other.dest.1),
                    max(other.source.1, other.dest.1),
                );
                let yr = (yr.0)..=(yr.1);
                let t = (x - self.source.0) / self.dir().0;
                let y = self.source.1 + t * self.dir().1;
                if t >= 0 && t <= self.len() && yr.contains(&y) {
                    Points::Single(x, y)
                } else {
                    Points::Empty
                }
            }
            (Vertical, Sloped) => other.intersection(self),
            (Sloped, Sloped) => {
                let dir = self.dir();
                let their_dir = other.dir();
                if dir == their_dir {
                    // Me first
                    let y_slope = dir.1 / dir.0;
                    let y_0 = self.source.1;
                    let b = y_0 - y_slope * self.source.0;
                    let y_0_them = other.source.1;
                    let b_them = y_0_them - other.source.0 * y_slope;
                    // b is y-intersect and must be the same or the lines are parallel
                    if b == b_them {
                        let left_x = max(
                            min(self.source.0, self.dest.0),
                            min(other.source.0, other.dest.0),
                        );
                        let right_x = min(
                            max(self.source.0, self.dest.0),
                            max(other.source.0, other.dest.0),
                        );

                        // Use this to calculuate t from Xt = X_0 + t * dir_x => Xt - X_0 = t * dir_x => t = (Xt - X_0) / dir_x
                        let t_me = (left_x - self.source.0) / dir.0;
                        if t_me < 0 || t_me > self.len() {
                            return Points::Empty;
                        }
                        let left_y = self.source.1 + t_me * dir.1;
                        let t_them = (left_x - other.source.0) / dir.0;
                        if t_them < 0 || t_them > other.len() {
                            return Points::Empty;
                        }
                        let points = right_x - left_x;

                        Points::Diagonally(left_x, left_y, points, dir)
                    } else {
                        Points::Empty
                    }
                } else {
                    let t_me = -self.source.1 / dir.1;
                    let a_me = dir.1 / dir.0; // -1 or 1
                    let b_me = -(self.source.0 + t_me * dir.0) * a_me;
                    let t_them = -other.source.1 / their_dir.1;
                    let a_them = their_dir.1 / their_dir.0; // -1 or 1
                    let b_them = -(other.source.0 + t_them * their_dir.0) * a_them;

                    // We know that one slope is negative and the other positive, so adding the
                    // equations simplifies to:
                    // 2y = b_them + b_me => y = (b_them + b_me) / 2
                    let b_total = b_them + b_me;
                    if b_total % 2 == 1 {
                        // If this happens, it's because of this scenario:
                        // #..#
                        // .##.
                        // .##.
                        // #..#
                        // Where the lines don't actually intersect
                        return Points::Empty;
                    }
                    let y = b_total / 2;
                    let top = min(
                        max(self.source.1, self.dest.1),
                        max(other.source.1, other.dest.1),
                    );
                    let bot = max(
                        min(self.source.1, self.dest.1),
                        min(other.source.1, other.dest.1),
                    );
                    let left = max(
                        min(self.source.0, self.dest.0),
                        min(other.source.0, other.dest.0),
                    );
                    let right = min(
                        max(self.source.0, self.dest.0),
                        max(other.source.0, other.dest.0),
                    );
                    let t = (y - self.source.1) / dir.1;
                    let x = self.source.0 + t * dir.0;

                    if (bot..=top).contains(&y) && (left..=right).contains(&x) {
                        Points::Single(x, y)
                    } else {
                        Points::Empty
                    }
                }
            }
            _ => Points::Empty,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Points {
    Horizontally(i32, RangeInclusive<i32>),
    Vertically(i32, RangeInclusive<i32>),
    Single(i32, i32),
    Diagonally(i32, i32, i32, (i32, i32)),
    Empty,
}

impl Points {
    #[cfg(test)]
    fn count(&mut self) -> usize {
        use Points::*;
        match self {
            Horizontally(_, r) => r.count(),
            Vertically(_, r) => r.count(),
            Single(_, _) => 1,
            Diagonally(_, _, steps, _) => (0..=*steps).count(),
            Empty => 0,
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

fn points_intersection(lines: &[Line]) -> HashSet<(i32, i32)> {
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
                Points::Diagonally(x0, y0, steps, (dx, dy)) => {
                    pointset.extend((0..=steps).map(|t| (x0 + dx * t, y0 + dy * t)))
                }
                Points::Empty => {}
            }
        }
    }
    pointset
}

fn solve(lines: &Vec<Line>) -> usize {
    let ps = points_intersection(lines);
    ps.len()
}

pub fn part_1(input: &str) -> Result<String> {
    let lines = parse_lines(input)?;
    let lines = lines
        .into_iter()
        .filter(|line| line.straight())
        .collect_vec();
    let sol = solve(&lines);
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
    fn test_diag_millionth_case() {
        let left = Line {
            source: (324, 221),
            dest: (911, 808),
        };
        let right = Line {
            source: (161, 890),
            dest: (808, 243),
        };
        assert_eq!(left.intersection(&right), Points::Single(577, 474));
    }

    #[test]
    fn test_diagonals() {
        let left = Line {
            source: (324, 221),
            dest: (911, 808),
        };
        let right = Line {
            source: (66, 936),
            dest: (941, 61),
        };
        assert_eq!(left.intersection(&right).count(), 0);
        let left = Line {
            source: (2, 2),
            dest: (2, 1),
        };
        let right = Line {
            source: (0, 0),
            dest: (8, 8),
        };
        assert_eq!(left.intersection(&right).count(), 1);
        let left = Line {
            source: (7, 0),
            dest: (7, 4),
        };
        let right = Line {
            source: (5, 5),
            dest: (8, 2),
        };
        assert_eq!(left.intersection(&right).count(), 1);
        let left = Line {
            source: (10, 10),
            dest: (30, 30),
        };
        let right = Line {
            source: (11, 11),
            dest: (15, 15),
        };
        assert_eq!(left.intersection(&right).count(), 5);
        assert_eq!(right.intersection(&left).count(), 5);
        let points = left.intersection(&right);
        assert_eq!(points, Points::Diagonally(11, 11, 4, (1, 1)));
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
    fn test_another_hard_case() {
        let left = Line {
            source: (62, 949),
            dest: (973, 38),
        };
        let right = Line {
            source: (55, 956),
            dest: (849, 162),
        };
        assert_eq!(left.intersection(&right).count(), 788);
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
