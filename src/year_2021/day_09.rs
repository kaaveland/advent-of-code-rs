use anyhow::{Context, Result};
use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug)]
struct Map {
    row_width: usize,
    map: Vec<u8>,
}

impl Map {
    fn new(row_width: usize) -> Map {
        Map {
            row_width,
            map: Vec::with_capacity(row_width),
        }
    }
    fn lookup(&self, x: isize, y: isize) -> Option<u8> {
        if x >= 0 && (x as usize) < self.row_width {
            if y >= 0 && (y as usize) < self.map.len() / self.row_width {
                Some(self.map[(y as usize) * self.row_width + (x as usize)])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn low_points(&self) -> Vec<(isize, isize, u8)> {
        let xmax = self.row_width;
        let ymax = self.map.len() / xmax;

        (0..ymax)
            .flat_map(|y| {
                (0..xmax).filter_map(move |x| {
                    self.val_if_low(x as isize, y as isize)
                        .map(|val| (x as isize, y as isize, val))
                })
            })
            .collect()
    }

    fn val_if_low(&self, x: isize, y: isize) -> Option<u8> {
        self.lookup(x, y).filter(|&val| {
            neighbours(x, y)
                .iter()
                .all(|(xn, yn)| self.lookup(*xn, *yn).map(|nval| nval > val).unwrap_or(true))
        })
    }
}

fn neighbours(x: isize, y: isize) -> Vec<(isize, isize)> {
    vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .collect()
}

fn parse(input: &str) -> Result<Map> {
    let mut lines = input.lines();
    let first = lines.next().context("Empty input")?;
    let mut map = Map::new(first.len());
    map.map.extend(
        first
            .chars()
            .filter_map(|ch| ch.to_digit(10).map(|dig| dig as u8)),
    );
    for next in lines {
        map.map.extend(
            next.chars()
                .filter_map(|ch| ch.to_digit(10).map(|dig| dig as u8)),
        );
    }
    Ok(map)
}

fn basin_area(map: &Map, x: isize, y: isize, val: u8) -> usize {
    let mut basin_members: FxHashSet<_> = FxHashSet::default();
    let mut work = vec![(x, y, val)];

    while let Some((x, y, n)) = work.pop() {
        if !basin_members.insert((x, y)) {
            continue;
        }
        let neigh = neighbours(x, y);
        let next = neigh
            .iter()
            .filter(|(xn, yn)| !basin_members.contains(&(*xn, *yn)))
            .filter_map(|(xn, yn)| {
                map.lookup(*xn, *yn)
                    .filter(|nval| *nval < 9 && *nval > n)
                    .map(|nval| (*xn, *yn, nval))
            });
        work.extend(next);
    }

    basin_members.len()
}

fn solve_1(input: &str) -> Result<u32> {
    let map = parse(input)?;

    let s = map
        .low_points()
        .iter()
        .map(|(_, _, val)| (*val + 1) as u32)
        .sum();

    Ok(s)
}

fn solve_2(input: &str) -> Result<usize> {
    let map = parse(input)?;

    let s = map
        .low_points()
        .iter()
        .map(|(x, y, val)| basin_area(&map, *x, *y, *val))
        .sorted()
        .rev()
        .take(3)
        .product();

    Ok(s)
}

pub fn part_1(input: &str) -> Result<String> {
    let s = solve_1(input)?;
    Ok(format!("{s}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let s = solve_2(input)?;
    Ok(format!("{s}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(15, solve_1(EXAMPLE).unwrap());
    }

    #[test]
    fn test_examplle_p2() {
        assert_eq!(1134, solve_2(EXAMPLE).unwrap());
    }

    #[test]
    fn test_parse() {
        let map = parse(EXAMPLE).unwrap();
        assert_eq!(map.row_width, 10);
        assert_eq!(map.map.len(), 10 * 5);
        assert_eq!(map.lookup(-1, 0), None);
        assert_eq!(map.lookup(0, 0), Some(2u8));
        assert_eq!(map.lookup(0, 4), Some(9u8));
        assert_eq!(map.lookup(9, 4), Some(8u8));
        assert_eq!(map.lookup(10, 4), None);
    }
    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";
}
