use anyhow::{anyhow, Result};
use fxhash::FxHashSet as Set;
use is_close::is_close;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i64 as parse_i64, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair, terminated, tuple};
use nom::IResult;
use std::ops::Sub;

fn parse_vec3(s: &str) -> IResult<&str, Vec3<i64>> {
    map(
        tuple((
            terminated(parse_i64, pair(char(','), space1)),
            terminated(parse_i64, pair(char(','), space1)),
            parse_i64,
        )),
        |(x, y, z)| Vec3 { x, y, z },
    )(s)
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: Sub<Output = T>> Vec3<T> {
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl From<Vec3<i64>> for Vec3<f64> {
    fn from(value: Vec3<i64>) -> Self {
        Vec3 {
            x: value.x as f64,
            y: value.y as f64,
            z: value.z as f64,
        }
    }
}

impl From<Hailstone<i64>> for Hailstone<f64> {
    fn from(value: Hailstone<i64>) -> Self {
        Hailstone {
            pos: value.pos.into(),
            vel: value.vel.into(),
        }
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
struct Hailstone<T> {
    pos: Vec3<T>,
    vel: Vec3<T>,
}
fn parse_hailstone(s: &str) -> IResult<&str, Hailstone<i64>> {
    let (s, (pos, vel)) = separated_pair(parse_vec3, tag(" @ "), parse_vec3)(s)?;
    Ok((s, Hailstone { pos, vel }))
}

fn parse_hailstones(s: &str) -> Result<Vec<Hailstone<i64>>> {
    Ok(separated_list1(char('\n'), parse_hailstone)(s)
        .map_err(|err| anyhow!("{err}"))?
        .1)
}

fn intersection(left: Hailstone<f64>, right: Hailstone<f64>) -> Option<(f64, f64)> {
    // to y = ax + b
    let left_slope = left.vel.y / left.vel.x;
    let left_b = left.pos.y - left_slope * left.pos.x;
    let right_slope = right.vel.y / right.vel.x;
    let right_b = right.pos.y - right_slope * right.pos.x;
    if is_close!(left_slope, right_slope) {
        if is_close!(left_b, right_b) {
            panic!("Identical hailstone?")
        } else {
            None
        }
    } else {
        let x = (right_b - left_b) / (left_slope - right_slope);
        let y = x * left_slope + left_b;
        if ((x > left.pos.x) == (left.vel.x > 0f64)) && ((x > right.pos.x) == (right.vel.x > 0f64))
        {
            Some((x, y))
        } else {
            None // In the past
        }
    }
}

fn count_intersections_within_area(hailstones: &[Hailstone<i64>], amin: f64, amax: f64) -> usize {
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in 0..i {
            if let Some((x, y)) = intersection(hailstones[i].into(), hailstones[j].into()) {
                if x >= amin && x <= amax && y >= amin && y <= amax {
                    count += 1;
                }
            }
        }
    }
    count
}
pub fn part_1(s: &str) -> Result<String> {
    let hailstones = parse_hailstones(s)?;
    Ok(
        count_intersections_within_area(&hailstones, 200000000000000f64, 400000000000000f64)
            .to_string(),
    )
}

// Make a number of assumptions that are possibly correct and could constrain the velocities the rock
// must have, namely: 1) all components of p and v are always int 2) t for each collision is int
// 3) the whole part 2 problem has exactly 1 solution, so that only 1 velocity vector is possible,
// which also means that only one velocity is possible in each component 4) the magnitude of the
// velocity vector is small, like the hailstone velocity vectors in the input
fn possible_velocities<F>(
    hailstones: &[Hailstone<i64>],
    dimension: F,
    max_abs_vel: i64,
) -> impl Iterator<Item = i64>
where
    F: Fn(Vec3<i64>) -> i64,
{
    let mut possible = Set::default();
    for i in 0..hailstones.len() {
        let a = hailstones[i];
        let v_a = dimension(a.vel);
        for &b in &hailstones[..i] {
            let v_b = dimension(b.vel);
            if v_a == v_b {
                // Hooray! a and b have a constant distance between them in `dimension`.
                // With our assumption that p, v and t are always int, that means
                // we can require that (v_a - v_rock) divides that distance
                let distance = dimension(a.pos) - dimension(b.pos);
                let new_possible = (-max_abs_vel..=max_abs_vel)
                    .filter(|rock_vel| *rock_vel != v_a && distance % (v_a - *rock_vel) == 0);
                if possible.is_empty() {
                    possible.extend(new_possible);
                } else {
                    possible = possible
                        .intersection(&new_possible.collect())
                        .copied()
                        .collect();
                }
            }
        }
    }
    possible.into_iter()
}

fn find_rock_velocity_vector(hailstones: &[Hailstone<i64>], max_abs_vel: i64) -> Vec3<i64> {
    let x = possible_velocities(hailstones, |v| v.x, max_abs_vel);
    let y = possible_velocities(hailstones, |v| v.y, max_abs_vel);
    let z = possible_velocities(hailstones, |v| v.z, max_abs_vel);
    let poss = x.zip(y).zip(z).collect_vec();
    assert_eq!(poss.len(), 1);
    let ((x, y), z) = poss[0];
    Vec3 { x, y, z }
}

pub fn part_2(s: &str) -> Result<String> {
    let hailstones = parse_hailstones(s)?;
    let rock_vel = find_rock_velocity_vector(&hailstones, 500);
    // The velocity of the rock is known. Choose any 2 hailstones:
    let a = hailstones[0];
    let b = hailstones[1];
    // These two _must_ intersect with the origin of the rock:
    let a_diff = Hailstone {
        pos: a.pos,
        vel: a.vel.sub(rock_vel),
    };
    let b_diff = Hailstone {
        pos: b.pos,
        vel: b.vel.sub(rock_vel),
    };
    // Let's first just intersect them in xy:
    if let Some((x, y)) = intersection(a_diff.into(), b_diff.into()) {
        // This should give us t, and then z by:
        // x = a_diff.pos.x + t * a_diff.vel.x =>
        // t = (x - a_diff.pos.x) / a_diff.vel.x
        let t = ((x as i64) - a_diff.pos.x) / a_diff.vel.x;
        let z = a_diff.pos.z + t * a_diff.vel.z;
        Ok(((x as i64) + (y as i64) + z).to_string())
    } else {
        Err(anyhow!("No intersection found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_count_intersections_in_area() {
        let hailstones = parse_hailstones(EX).unwrap();
        assert_eq!(count_intersections_within_area(&hailstones, 7f64, 27f64), 2);
    }
}
