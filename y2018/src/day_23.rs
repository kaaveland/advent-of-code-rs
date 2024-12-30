use anyhow::{anyhow, Context};
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Bot {
    coords: [i32; 3],
    radius: i32,
}

fn manhattan(lhs: &[i32; 3], rhs: &[i32; 3]) -> i32 {
    lhs.iter()
        .zip(rhs.iter())
        .map(|(lhs, rhs)| (*lhs - *rhs).abs())
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Cube {
    min_coords: [i32; 3],
    max_coords: [i32; 3],
}

impl Cube {
    #[inline]
    fn contains(&self, point: [i32; 3]) -> bool {
        let [x, y, z] = point;
        let xrange = self.min_coords[0]..=self.max_coords[0];
        let yrange = self.min_coords[1]..=self.max_coords[1];
        let zrange = self.min_coords[2]..=self.max_coords[2];
        xrange.contains(&x) && yrange.contains(&y) && zrange.contains(&z)
    }

    #[inline]
    fn size(&self) -> i32 {
        self.max_coords[0] - self.min_coords[0] + 1
    }

    #[inline]
    fn expand(&self) -> Self {
        let sz = self.size() * 2 - 1;
        let [xmin, ymin, zmin] = self.min_coords;
        Self {
            min_coords: self.min_coords,
            max_coords: [xmin + sz, ymin + sz, zmin + sz],
        }
    }

    #[inline]
    fn closest_to_origin(&self) -> i32 {
        self.min_coords
            .iter()
            .zip(self.max_coords.iter())
            .map(|(c1, c2)| c1.abs().min(c2.abs()))
            .sum()
    }

    fn new(min_coords: [i32; 3], max_coords: [i32; 3]) -> Self {
        let [xsize, ysize, zsize] = [
            max_coords[0] - min_coords[0] + 1,
            max_coords[1] - min_coords[1] + 1,
            max_coords[2] - min_coords[2] + 1,
        ];
        assert_eq!(xsize, ysize);
        assert_eq!(ysize, zsize);
        assert_eq!(
            xsize & (xsize - 1),
            0,
            "xsize must be a power of 2: {xsize}"
        );

        Self {
            min_coords,
            max_coords,
        }
    }

    fn split(&self) -> [Self; 8] {
        let [xmin, ymin, zmin] = self.min_coords;
        let [xmax, ymax, zmax] = self.max_coords;
        let xlen = xmax - xmin;
        let ylen = ymax - ymin;
        let zlen = zmax - zmin;
        let mid_x = xmin + xlen / 2;
        let mid_y = ymin + ylen / 2;
        let mid_z = zmin + zlen / 2;
        [
            Self::new([xmin, ymin, zmin], [mid_x, mid_y, mid_z]),
            Self::new([mid_x + 1, ymin, zmin], [xmax, mid_y, mid_z]),
            Self::new([xmin, mid_y + 1, zmin], [mid_x, ymax, mid_z]),
            Self::new([mid_x + 1, mid_y + 1, zmin], [xmax, ymax, mid_z]),
            Self::new([xmin, ymin, mid_z + 1], [mid_x, mid_y, zmax]),
            Self::new([mid_x + 1, ymin, mid_z + 1], [xmax, mid_y, zmax]),
            Self::new([xmin, mid_y + 1, mid_z + 1], [mid_x, ymax, zmax]),
            Self::new([mid_x + 1, mid_y + 1, mid_z + 1], [xmax, ymax, zmax]),
        ]
    }
}

fn intersection_count(cube: &Cube, bots: &[Bot]) -> usize {
    let [x1, y1, z1] = cube.min_coords;
    let [x2, y2, z2] = cube.max_coords;

    bots.iter()
        .filter(|bot| {
            let [bx, by, bz] = bot.coords;
            // Find the closest distance between the cube and the bot in all 3 dimensions
            let x = (x1 - bx).max(0) + (bx - x2).max(0);
            let y = (y1 - by).max(0) + (by - y2).max(0);
            let z = (z1 - bz).max(0) + (bz - z2).max(0);
            // Their sum is the manhattan distance between the closest point in the cube
            // and the bot location
            x + y + z <= bot.radius
        })
        .count()
}

fn parse_bots(s: &str) -> anyhow::Result<Vec<Bot>> {
    let r = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)")?;
    let mut bots = vec![];
    for line in s.lines() {
        let m = r
            .captures(line)
            .with_context(|| anyhow!("Bad line: {line}"))?;
        let coords = [
            m.get(1).unwrap().as_str().parse()?,
            m.get(2).unwrap().as_str().parse()?,
            m.get(3).unwrap().as_str().parse()?,
        ];
        let radius = m.get(4).unwrap().as_str().parse()?;
        bots.push(Bot { coords, radius });
    }
    bots.sort_by_key(|bot| bot.radius);
    Ok(bots)
}

/// Find the lowest coordinate and make a 1x1x1 cube around it. Then double the side length
/// of the cube until it contains all the bot positions
fn make_initial_cube_around(bots: &[Bot]) -> Option<Cube> {
    let xmin = bots.iter().map(|bot| bot.coords[0]).min()?;
    let ymin = bots.iter().map(|bot| bot.coords[1]).min()?;
    let zmin = bots.iter().map(|bot| bot.coords[2]).min()?;
    let mut cube = Cube::new([xmin, ymin, zmin], [xmin, ymin, zmin]);
    while bots.iter().any(|bot| !cube.contains(bot.coords)) {
        cube = cube.expand();
    }
    Some(cube)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let bots = parse_bots(s)?;
    let best_bot = *bots.last().unwrap();
    let n = bots
        .into_iter()
        .filter(|b| manhattan(&best_bot.coords, &b.coords) <= best_bot.radius)
        .count();
    Ok(n.to_string())
}

fn best_cubelet(initial: Cube, bots: &[Bot]) -> Cube {
    // No cube-splitting if there's bots we don't intersect yet
    assert_eq!(intersection_count(&initial, bots), bots.len());
    let mut work = BinaryHeap::new();
    work.push(Reverse((
        0,
        initial.closest_to_origin(),
        initial.size(),
        initial,
    )));
    while let Some(Reverse((_outside, _dist, _size, cube))) = work.pop() {
        // This is the cube that minimizes the amount of bots outside it and the distance from origin
        if cube.size() == 1 {
            return cube;
        } else {
            for cubelet in cube.split() {
                let outside = bots.len() - intersection_count(&cubelet, bots);
                work.push(Reverse((
                    outside,
                    cubelet.closest_to_origin(),
                    cubelet.size(),
                    cubelet,
                )));
            }
        }
    }
    unreachable!()
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let bots = parse_bots(s)?;
    let cube = make_initial_cube_around(&bots)
        .with_context(|| anyhow!("Unable to make cube around {bots:?}"))?;
    let found = best_cubelet(cube, &bots);
    let n = found.closest_to_origin();
    Ok(n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fxhash::FxHashSet;
    use itertools::Itertools;

    impl Cube {
        fn points(&self) -> impl Iterator<Item = [i32; 3]> {
            let xrange = self.min_coords[0]..=self.max_coords[0];
            let yrange = self.min_coords[1]..=self.max_coords[1];
            let zrange = self.min_coords[2]..=self.max_coords[2];
            xrange
                .cartesian_product(yrange)
                .cartesian_product(zrange)
                .map(|((x, y), z)| [x, y, z])
        }
    }

    #[test]
    #[should_panic]
    fn cube_with_uneven_sides() {
        Cube::new([0, 0, 0], [3, 4, 3]);
    }

    #[test]
    #[should_panic]
    fn cube_with_size_not_power_of_2() {
        Cube::new([0, 0, 0], [4, 4, 4]);
    }

    #[test]
    fn test_cube_expansion() {
        let initial = Cube::new([0, 0, 0], [0, 0, 0]);
        assert_eq!(initial.size(), 1);
        let next = initial.expand();
        assert_eq!(next.size(), 2);
        let next = next.expand();
        assert_eq!(next.size(), 4);
    }

    #[test]
    fn test_point_detection() {
        let ex = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";
        let bots = parse_bots(ex).unwrap();
        let cube = make_initial_cube_around(&bots).unwrap();
        let best_cube = best_cubelet(cube, &bots);
        assert_eq!(best_cube.min_coords, [12, 12, 12]);
    }

    #[test]
    fn test_bot_collisions() {
        let ex = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";
        let bots = parse_bots(ex).unwrap();
        let cube = Cube::new([12, 12, 12], [12, 12, 12]);
        assert_eq!(intersection_count(&cube, &bots), 5);
        let cube = Cube::new([10, 12, 12], [10, 12, 12]);
        assert!(intersection_count(&cube, &bots) < 5);
    }

    #[test]
    fn cube_splitting() {
        let cube = Cube::new([0, 0, 0], [7, 7, 7]);
        let cube_points: FxHashSet<_> = cube.points().collect();
        let cubelets = cube.split();
        // All the cubelets need to be in the cube
        for cubelet in cubelets.iter() {
            let points: FxHashSet<_> = cubelet.points().collect();
            assert_eq!(cube_points.intersection(&points).count(), points.len());
        }
        for (l, r) in cubelets
            .iter()
            .cartesian_product(cubelets.iter())
            .filter(|(left, right)| left != right)
        {
            assert_eq!(l.size(), r.size());
            // check that no cube intersects another
            let points_in_l: FxHashSet<_> = l.points().collect();
            let points_in_r: FxHashSet<_> = r.points().collect();
            assert_eq!(points_in_l.intersection(&points_in_r).count(), 0);
        }
    }

    #[test]
    fn collision_detection() {
        let cube = Cube::new([0, 0, 0], [7, 7, 7]);
        let bot = Bot {
            coords: [1, 1, 1],
            radius: 1,
        };
        assert_eq!(intersection_count(&cube, &[bot]), 1);
        let bot = Bot {
            coords: [9, 9, 9],
            radius: 1,
        };
        assert_eq!(intersection_count(&cube, &[bot]), 0);
        let bot = Bot {
            coords: [9, 9, 9],
            // 7, 7, 7 is manhattan distance 6 away
            radius: 6,
        };
        assert_eq!(intersection_count(&cube, &[bot]), 1);
        let bot = Bot {
            coords: [9, 9, 9],
            // 7, 7, 7 is manhattan distance 6 away
            radius: 5,
        };
        assert_eq!(intersection_count(&cube, &[bot]), 0);
    }
}
