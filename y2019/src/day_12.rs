use anyhow::{anyhow, Result};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;
use std::str::FromStr;

type Vec3 = [i32; 3];

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Hash)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(pos: Vec3) -> Self {
        Moon {
            pos,
            ..Moon::default()
        }
    }
    fn energy(self) -> i32 {
        let potential: i32 = self.pos.into_iter().map(|n| n.abs()).sum();
        let kinetic: i32 = self.vel.into_iter().map(|n| n.abs()).sum();
        potential * kinetic
    }
}

fn parse_int(i: &str) -> IResult<&str, i32> {
    fn neg(i: &str) -> IResult<&str, i32> {
        map(
            map_res(preceded(complete::char('-'), digit1), FromStr::from_str),
            |n: i32| -n,
        )(i)
    }
    alt((neg, map_res(digit1, FromStr::from_str)))(i)
}

fn parse_coord(ch: char) -> impl Fn(&str) -> IResult<&str, i32> {
    move |i: &str| preceded(tuple((complete::char(ch), complete::char('='))), parse_int)(i)
}

fn parse_vec3(i: &str) -> IResult<&str, Vec3> {
    fn inner(i: &str) -> IResult<&str, Vec3> {
        let (i, x) = terminated(parse_coord('x'), tag(", "))(i)?;
        let (i, y) = terminated(parse_coord('y'), tag(", "))(i)?;
        let (i, z) = parse_coord('z')(i)?;
        Ok((i, [x, y, z]))
    }
    delimited(complete::char('<'), inner, complete::char('>'))(i)
}

fn parse_moons(i: &str) -> Result<Vec<Moon>> {
    let (_, moons) =
        separated_list1(newline, parse_vec3)(i).map_err(|e| anyhow!("Parser error: {e}"))?;
    Ok(moons.into_iter().map(Moon::new).collect())
}

fn time_step(moons: &[Moon], buf: &mut Vec<Moon>) {
    buf.clear();
    buf.extend(moons.iter().copied());
    for i in 0..moons.len() {
        for j in i..moons.len() {
            let dx = (moons[j].pos[0] - moons[i].pos[0]).signum();
            let dy = (moons[j].pos[1] - moons[i].pos[1]).signum();
            let dz = (moons[j].pos[2] - moons[i].pos[2]).signum();
            buf[i].vel[0] += dx;
            buf[j].vel[0] -= dx;
            buf[i].vel[1] += dy;
            buf[j].vel[1] -= dy;
            buf[i].vel[2] += dz;
            buf[j].vel[2] -= dz;
        }
    }
    buf.iter_mut().for_each(|moon| {
        moon.pos
            .iter_mut()
            .zip(moon.vel.iter().copied())
            .for_each(|(pos, vel)| *pos += vel);
    });
}

fn n_steps(n: usize, moons: &[Moon]) -> Vec<Moon> {
    let mut now = moons.iter().copied().collect_vec();
    let mut scratch = vec![];
    for _ in 0..n {
        time_step(&now, &mut scratch);
        std::mem::swap(&mut now, &mut scratch);
    }
    now
}

fn solve_1(n: usize, input: &str) -> Result<i32> {
    let moons = parse_moons(input)?;
    let last_state = n_steps(n, &moons);
    Ok(last_state.into_iter().map(|moon| moon.energy()).sum())
}

pub fn part_1(input: &str) -> Result<String> {
    solve_1(1000, input).map(|n| format!("{n}"))
}

fn solve_2(input: &str) -> Result<i64> {
    let mut moons = parse_moons(input)?;
    let mut scratch = vec![];
    let mut time = 0;
    let mut x_repeat = None;
    let mut y_repeat = None;
    let mut z_repeat = None;
    let mut x_cache = HashSet::default();
    let mut y_cache = HashSet::default();
    let mut z_cache = HashSet::default();

    let update_cache =
        |repeat: &mut Option<i64>, cache: &mut HashSet<Vec<i32>>, moons: &[Moon], dim, time| {
            if repeat.is_none() {
                let state = moons
                    .iter()
                    .map(|moon| moon.pos[dim])
                    .chain(moons.iter().map(|moon| moon.vel[dim]))
                    .collect_vec();
                if !cache.insert(state) {
                    *repeat = Some(time);
                }
            }
        };

    while x_repeat.is_none() || y_repeat.is_none() || z_repeat.is_none() {
        update_cache(&mut x_repeat, &mut x_cache, &moons, 0, time);
        update_cache(&mut y_repeat, &mut y_cache, &moons, 1, time);
        update_cache(&mut z_repeat, &mut z_cache, &moons, 2, time);

        time_step(&moons, &mut scratch);
        std::mem::swap(&mut moons, &mut scratch);
        time += 1;
    }
    Ok(lcm(
        lcm(x_repeat.unwrap(), y_repeat.unwrap()),
        z_repeat.unwrap(),
    ))
}

fn lcm(lhs: i64, rhs: i64) -> i64 {
    fn gcd(mut lhs: i64, mut rhs: i64) -> i64 {
        while rhs != 0 {
            (lhs, rhs) = (rhs, lhs.rem_euclid(rhs))
        }
        lhs
    }
    let g = gcd(lhs, rhs);
    lhs * rhs / g
}

pub fn part_2(input: &str) -> Result<String> {
    solve_2(input).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsers() {
        assert_eq!(parse_int("-13 "), Ok((" ", -13)));
        assert!(parse_int("- 131").is_err());
        assert_eq!(parse_int("139"), Ok(("", 139)));
        assert_eq!(parse_coord('x')("x=-131, "), Ok((", ", -131)));
        assert_eq!(parse_vec3("<x=1, y=9, z=4>\n<"), Ok(("\n<", [1, 9, 4])));
        let moons = vec![Moon::new([1, 2, 3]), Moon::new([-3, -2, -1])];
        assert_eq!(
            parse_moons("<x=1, y=2, z=3>\n<x=-3, y=-2, z=-1>").unwrap(),
            moons
        );
    }

    #[test]
    fn check_signum_behaviour() {
        assert_eq!(-1, (-10i32).signum());
        assert_eq!(0, (0i32).signum());
        assert_eq!(1, (10i32).signum());
    }

    #[test]
    fn test_timestep() {
        let example = vec![
            Moon::new([-1, 0, 2]),
            Moon::new([2, -10, -7]),
            Moon::new([4, -8, 8]),
            Moon::new([3, 5, -1]),
        ];
        let mut next = vec![];
        time_step(&example, &mut next);
        assert_eq!(
            next,
            vec![
                Moon {
                    pos: [2, -1, 1],
                    vel: [3, -1, -1]
                },
                Moon {
                    pos: [3, -7, -4],
                    vel: [1, 3, 3]
                },
                Moon {
                    pos: [1, -7, 5],
                    vel: [-3, 1, -3]
                },
                Moon {
                    pos: [2, 2, 0],
                    vel: [-1, -3, 1]
                },
            ]
        )
    }

    #[test]
    fn test_example() {
        let ex = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        assert_eq!(solve_1(10, ex).unwrap(), 179);
    }

    #[test]
    fn test_part_2() {
        let ex = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        assert_eq!(solve_2(ex).unwrap(), 4686774924);
    }
}
