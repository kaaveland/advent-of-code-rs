use anyhow::{Context, Result};
use fxhash::FxHashSet as HashSet;
use std::cmp::Ordering::Equal;
use std::hash::{Hash, Hasher};

type Asteriod = [i32; 2];

fn parse(input: &str) -> HashSet<Asteriod> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some([x as i32, y as i32])
                } else {
                    None
                }
            })
        })
        .collect()
}

/// Can't put f64 into any Set without promising that we can hash/compare them nicely
struct Float64(f64);

impl PartialEq<Self> for Float64 {
    fn eq(&self, other: &Self) -> bool {
        let (Float64(lhs), Float64(rhs)) = (self, other);
        lhs.partial_cmp(rhs) == Some(Equal)
    }
}

impl Eq for Float64 {}

impl Hash for Float64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Float64(f) = *self;
        let h = (f * 1e18) as i64; // 18 significant digits ought to be enough for anybody
        h.hash(state);
    }
}

fn detection_angle(source: Asteriod, target: Asteriod) -> Option<Float64> {
    let v = [target[0] - source[0], target[1] - source[1]];
    if v != [0, 0] {
        let dx = v[0] as f64;
        let dy = v[1] as f64;
        Some(Float64(dy.atan2(dx)))
    } else {
        None
    }
}

fn most_detection_angles(asteroids: &HashSet<Asteriod>) -> Option<usize> {
    asteroids
        .iter()
        .copied()
        .map(|source| {
            let angles: HashSet<_> = asteroids
                .iter()
                .copied()
                .filter_map(|target| detection_angle(source, target))
                .collect();
            angles.len()
        })
        .max()
}

pub fn part_1(input: &str) -> Result<String> {
    let asteroids = parse(input);
    let winner = most_detection_angles(&asteroids);
    winner
        .context("Unable to find any asteroid")
        .map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_1() {
        let asteroids = parse(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        );
        assert_eq!(most_detection_angles(&asteroids), Some(33));
    }

    #[test]
    fn test_ex_4() {
        let asteroids = parse(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );
        assert_eq!(most_detection_angles(&asteroids), Some(210));
    }
}
