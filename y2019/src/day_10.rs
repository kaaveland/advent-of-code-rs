use anyhow::{Context, Result};
use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use itertools::Itertools;
use std::cmp::Ordering::Equal;
use std::cmp::{Ordering, Reverse};
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};

type Asteroid = [i32; 2];

fn parse(input: &str) -> HashSet<Asteroid> {
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
#[derive(Copy, Clone, Debug)]
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

impl PartialOrd<Self> for Float64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let Float64(lhs) = self;
        let Float64(rhs) = other;
        lhs.partial_cmp(rhs)
    }
}

impl Ord for Float64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Less)
    }
}

/// This corrects for our upside down coordinate reference system to make 0.0 up
/// even though the y axis grows down
fn detection_angle(source: Asteroid, target: Asteroid) -> Option<Float64> {
    let v = [source[0] - target[0], target[1] - source[1]];
    if v != [0, 0] {
        let dx = v[0] as f64;
        let dy = v[1] as f64;
        let radians = dy.atan2(dx) + PI / 2.0;
        Some(Float64(radians))
    } else {
        None
    }
}

fn most_detection_angles(asteroids: &HashSet<Asteroid>) -> Option<(Asteroid, usize)> {
    asteroids
        .iter()
        .copied()
        .map(|source| {
            let angles: HashSet<_> = asteroids
                .iter()
                .copied()
                .filter_map(|target| detection_angle(source, target))
                .collect();
            (source, angles.len())
        })
        .max_by_key(|(_, angles)| *angles)
}

pub fn part_1(input: &str) -> Result<String> {
    let asteroids = parse(input);
    let winner = most_detection_angles(&asteroids);
    winner
        .context("Unable to find any asteroid")
        .map(|(_, n)| format!("{n}"))
}

fn angles_to_others(
    source: Asteroid,
    asteroids: &HashSet<Asteroid>,
) -> HashMap<Float64, Vec<&Asteroid>> {
    let mut to_others: HashMap<Float64, Vec<&Asteroid>> = HashMap::default();
    let add = asteroids
        .iter()
        .filter_map(|asteroid| detection_angle(source, *asteroid).map(|angle| (angle, asteroid)));
    for (angle, asteroid) in add {
        to_others.entry(angle).or_default().push(asteroid);
    }
    let distance_to_source = |asteroid: &&Asteroid| {
        let dx = (source[0] - asteroid[0]) as f64;
        let dy = (source[1] - asteroid[1]) as f64;
        Reverse(Float64((dx * dx + dy * dy).sqrt()))
    };
    to_others
        .iter_mut()
        .for_each(|(_, asteroids)| asteroids.sort_by_key(distance_to_source));
    to_others
}

fn fire_laser(
    targets_by_angle: &mut HashMap<Float64, Vec<&Asteroid>>,
    times: usize,
) -> Option<Asteroid> {
    // Laser starts pointing up and shoots clockwise; but our coordinate system is upside-down
    // since y increases downwards
    let target_angles = targets_by_angle
        .keys()
        .copied()
        .sorted()
        .rev() // compensate for upside-down y again
        .collect_vec();
    let mut target_idx: usize = 0;
    while target_angles[target_idx] > Float64(0.0) {
        target_idx += 1;
    }
    let mut shot = 0;

    while shot < times {
        let target_angle = target_angles[target_idx % target_angles.len()];
        if let Some(target) = targets_by_angle.get_mut(&target_angle).unwrap().pop() {
            shot += 1;
            if shot == times {
                return Some(*target);
            }
        }
        target_idx += 1;
    }

    None
}

pub fn part_2(input: &str) -> Result<String> {
    let asteroids = parse(input);
    let (source, _) = most_detection_angles(&asteroids).context("Unable to find source")?;
    let mut source_angles = angles_to_others(source, &asteroids);
    let winner = fire_laser(&mut source_angles, 200).context("Unable to shoot 200 times")?;
    let n = winner[0] * 100 + winner[1];
    Ok(format!("{n}"))
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
        assert_eq!(most_detection_angles(&asteroids).map(|(_, a)| a), Some(33));
    }

    #[test]
    fn test_ex_4() {
        let asteroids = parse(EX_4);
        assert_eq!(most_detection_angles(&asteroids).map(|(_, a)| a), Some(210));
    }

    #[test]
    fn test_atan2_behaviour() {
        // Assume we are the origin (0, 0), up is (0, 1), but we're upside down
        println!("{}", (0.0f64).atan2(-1.0));
    }

    #[test]
    fn test_part_2() {
        let asteroids = parse(EX_4);
        let (source, _) = most_detection_angles(&asteroids).unwrap();
        let mut targets = angles_to_others(source, &asteroids);
        let n_shot = fire_laser(&mut targets, 200);
        assert_eq!(n_shot, Some([8, 2]));
    }

    const EX_4: &str = ".#..##.###...#######
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
###.##.####.##.#..##";
}
