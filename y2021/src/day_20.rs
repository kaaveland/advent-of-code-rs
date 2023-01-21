use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::{max, min};

type Pixel = bool;
const SIZE: usize = 512;

type Algorithm = [Pixel; SIZE];
type Kernel = [Pixel; 9]; // 3 x 3 kernel
type Coord = i32;
type Coordinate = (Coord, Coord);
type Image = HashMap<Coordinate, Pixel>;

fn parse(input: &str) -> Option<(Algorithm, Image)> {
    let (algorithm, image) = input.split_once("\n\n")?;

    assert_eq!(algorithm.len(), SIZE);
    let mut alg: Algorithm = [false; SIZE];

    for (i, ch) in algorithm.chars().enumerate() {
        alg[i] = ch == '#';
    }

    let image = image
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as Coord, y as Coord), ch == '#'))
        })
        .collect();

    Some((alg, image))
}

const NEIGHBOURS: [Coordinate; 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn convolution_kernel(image: &Image, position: Coordinate, background: Pixel) -> Kernel {
    let (x, y) = position;
    let mut kernel = [background; 9];
    for (i, (dx, dy)) in NEIGHBOURS.iter().enumerate() {
        kernel[i] = *image.get(&(x + *dx, y + *dy)).unwrap_or(&background);
    }
    kernel
}

fn lookup(kernel: &Kernel, algorithm: &Algorithm) -> Option<Pixel> {
    let index = kernel
        .iter()
        .fold(0, |idx, bit| idx * 2 + usize::from(*bit));
    algorithm.get(index).copied()
}

fn convolve(image: &Image, algorithm: &Algorithm, background: Pixel) -> Image {
    let (xmin, xmax) = image
        .keys()
        .fold((Coord::MAX, Coord::MIN), |(xmin, xmax), (x, _)| {
            (min(xmin, *x), max(xmax, *x))
        });
    let (xmin, xmax) = (xmin - 1, xmax + 1);
    let (ymin, ymax) = image
        .keys()
        .fold((Coord::MAX, Coord::MIN), |(ymin, ymax), (_, y)| {
            (min(ymin, *y), max(ymax, *y))
        });
    let (ymin, ymax) = (ymin - 1, ymax + 1);
    let coords = (xmin..=xmax).cartesian_product(ymin..=ymax).collect_vec();
    coords
        .into_par_iter()
        .filter_map(|(x, y)| {
            let kernel = convolution_kernel(image, (x, y), background);
            lookup(&kernel, algorithm).map(|pixel| ((x, y), pixel))
        })
        .collect()
}

pub fn part_1(input: &str) -> Result<String> {
    let (alg, image) = parse(input).with_context(|| anyhow!("Bad input"))?;
    let image = convolve(&image, &alg, !alg[0]);
    let image = convolve(&image, &alg, alg[0]);
    let lit = image.values().filter(|pixel| **pixel).count();
    Ok(format!("{lit}"))
}
pub fn part_2(input: &str) -> Result<String> {
    let (alg, mut image) = parse(input).with_context(|| anyhow!("Bad input"))?;
    for round in 0..50 {
        image = convolve(&image, &alg, alg[0] && (round & 1 == 1));
    }
    let lit = image.values().filter(|pixel| **pixel).count();
    Ok(format!("{lit}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let problem = parse(EXAMPLE);
        assert!(problem.is_some());
    }

    #[test]
    fn test_example() {
        let (alg, image) = parse(EXAMPLE).unwrap();
        let image = convolve(&image, &alg, false);
        let image = convolve(&image, &alg, false);
        let lit = image.values().filter(|pixel| **pixel).count();
        assert_eq!(lit, 35);
    }
    const EXAMPLE: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
}
