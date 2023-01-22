use anyhow::Result;
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;

type Cube = [i64; 3];
type HyperCube = [i64; 4];

#[derive(Eq, PartialEq, Debug)]
struct ActiveCubes {
    minbounds: Cube,
    maxbounds: Cube,
    cubes: HashSet<Cube>,
}

#[derive(Eq, PartialEq, Debug)]
struct ActiveHyperCubes {
    minbounds: HyperCube,
    maxbounds: HyperCube,
    hyper_cubes: HashSet<HyperCube>,
}

fn parse(input: &str) -> ActiveCubes {
    let cubes: HashSet<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some([x as i64, y as i64, 0])
                } else {
                    None
                }
            })
        })
        .collect();
    let mut minbounds = cubes.iter().fold([0, 0, 0], |mins, cand| {
        [mins[0].min(cand[0]), mins[1].min(cand[1]), -1]
    });
    let mut maxbounds = cubes.iter().fold([0, 0, 0], |max, cand| {
        [max[0].max(cand[0]), max[1].max(cand[1]), 1]
    });
    minbounds[0] -= 1;
    minbounds[1] -= 1;
    maxbounds[0] += 1;
    maxbounds[1] += 1;

    ActiveCubes {
        minbounds,
        maxbounds,
        cubes,
    }
}

fn add(lhs: &Cube, rhs: &Cube) -> Cube {
    let mut out = [0; 3];
    out.iter_mut()
        .zip(lhs.iter())
        .zip(rhs.iter())
        .for_each(|((dst, lhs), rhs)| *dst = lhs + rhs);
    out
}

fn addh(lhs: &HyperCube, rhs: &HyperCube) -> HyperCube {
    let mut out = [0; 4];
    out.iter_mut()
        .zip(lhs.iter())
        .zip(rhs.iter())
        .for_each(|((dst, lhs), rhs)| *dst = lhs + rhs);
    out
}

fn step(cubes: &HashSet<Cube>, minbounds: &Cube, maxbounds: &Cube) -> HashSet<Cube> {
    let dxdydz = (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|((dx, dy), dz)| [dx, dy, dz])
        .filter(|dxdydz| dxdydz != &[0, 0, 0])
        .collect_vec();

    let consider = (minbounds[0]..=maxbounds[0])
        .cartesian_product(minbounds[1]..=maxbounds[1])
        .cartesian_product(minbounds[2]..=maxbounds[2])
        .map(|((dx, dy), dz)| [dx, dy, dz]);

    consider
        .filter(|cube| {
            let n = dxdydz
                .iter()
                .map(|delta| add(cube, delta))
                .filter(|neigh| cubes.contains(neigh))
                .count();
            (n == 2 && cubes.contains(cube)) || n == 3
        })
        .collect()
}

fn steph(
    cubes: &HashSet<HyperCube>,
    minbounds: &HyperCube,
    maxbounds: &HyperCube,
) -> HashSet<HyperCube> {
    let dxdydz = (-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .cartesian_product(-1..=1)
        .map(|(((dx, dy), dz), dw)| [dx, dy, dz, dw])
        .filter(|dxdydz| dxdydz != &[0, 0, 0, 0])
        .collect_vec();

    let consider = (minbounds[0]..=maxbounds[0])
        .cartesian_product(minbounds[1]..=maxbounds[1])
        .cartesian_product(minbounds[2]..=maxbounds[2])
        .cartesian_product(minbounds[3]..=maxbounds[3])
        .map(|(((dx, dy), dz), dw)| [dx, dy, dz, dw]);

    consider
        .filter(|cube| {
            let n = dxdydz
                .iter()
                .map(|delta| addh(cube, delta))
                .filter(|neigh| cubes.contains(neigh))
                .count();
            (n == 2 && cubes.contains(cube)) || n == 3
        })
        .collect()
}

fn steps(cubes: &ActiveCubes, steps: usize) -> usize {
    let mut active = cubes.cubes.clone();
    let mut minbounds = cubes.minbounds;
    let mut maxbounds = cubes.maxbounds;

    for _ in 0..steps {
        active = step(&active, &minbounds, &maxbounds);
        minbounds.iter_mut().for_each(|lim| *lim -= 1);
        maxbounds.iter_mut().for_each(|lim| *lim += 1);
    }

    active.len()
}

fn stepsh(cubes: &ActiveHyperCubes, steps: usize) -> usize {
    let mut active = cubes.hyper_cubes.clone();
    let mut minbounds = cubes.minbounds;
    let mut maxbounds = cubes.maxbounds;

    for _ in 0..steps {
        active = steph(&active, &minbounds, &maxbounds);
        minbounds.iter_mut().for_each(|lim| *lim -= 1);
        maxbounds.iter_mut().for_each(|lim| *lim += 1);
    }

    active.len()
}

pub fn part_1(input: &str) -> Result<String> {
    let initial = parse(input);
    let result = steps(&initial, 6);
    Ok(format!("{result}"))
}

fn add_extra_dim(cubes: &ActiveCubes) -> ActiveHyperCubes {
    let hyper_cubes = cubes
        .cubes
        .iter()
        .map(|[x, y, z]| [*x, *y, *z, 0])
        .collect();
    let minbounds = [
        cubes.minbounds[0],
        cubes.minbounds[1],
        cubes.minbounds[2],
        -1,
    ];
    let maxbounds = [
        cubes.maxbounds[0],
        cubes.maxbounds[1],
        cubes.maxbounds[2],
        1,
    ];
    ActiveHyperCubes {
        minbounds,
        maxbounds,
        hyper_cubes,
    }
}

pub fn part_2(input: &str) -> Result<String> {
    let initial = parse(input);
    let initial = add_extra_dim(&initial);
    let result = stepsh(&initial, 6);
    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let initial = parse(
            ".#.
..#
###",
        );
        let living = steps(&initial, 6);
        assert_eq!(living, 112);
    }

    #[test]
    fn test_p2() {
        let initial = parse(
            ".#.
..#
###",
        );
        let initial = add_extra_dim(&initial);
        let living = stepsh(&initial, 6);
        assert_eq!(living, 848);
    }
}
