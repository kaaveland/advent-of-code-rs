use crate::day_13::Reflection::{Horizontal, Vertical};
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use shared::grid_parser;

type Map = Vec<Vec<bool>>;
fn parse(input: &str) -> Map {
    let height = input.lines().filter(|line| !line.is_empty()).count();
    let width = input.lines().next().map(|line| line.len()).unwrap_or(0);
    let mut map = vec![vec![false; width]; height];
    grid_parser(input, &|ch| if ch == '#' { Some(ch) } else { None })
        .for_each(|((x, y), _): ((i32, i32), char)| map[y as usize][x as usize] = true);
    map
}

fn is_mirrored_horizontally(map: &Map, row: usize) -> bool {
    let before = (0..).take_while(|i| *i <= row).map(|i| row - i);
    let after = (1..).take_while(|i| *i + row < map.len()).map(|i| row + i);
    let (mirrored, size) = before
        .zip(after)
        .fold((true, 0), |(mirrors, count), (before, after)| {
            (mirrors && map[before] == map[after], count + 1)
        });
    mirrored && size > 0
}

fn is_mirrored_vertically(map: &Map, col: usize) -> bool {
    let before = (0..).take_while(|i| *i <= col).map(|i| col - i);
    let after = (1..)
        .take_while(|i| *i + col < map[0].len())
        .map(|i| col + i);
    let (mirrored, size) =
        before
            .zip(after)
            .fold((true, 0), |(mirrors, count), (before, after)| {
                (
                    mirrors && map.iter().all(|row| row[before] == row[after]),
                    count + 1,
                )
            });
    mirrored && size > 0
}

fn find_horizontal_mirror(map: &Map) -> Option<usize> {
    (0..map.len()).find(|i| is_mirrored_horizontally(map, *i))
}

fn find_vertical_mirror(map: &Map) -> Option<usize> {
    (0..map[0].len()).find(|i| is_mirrored_vertically(map, *i))
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Reflection {
    fn value(&self) -> usize {
        match self {
            Horizontal(i) => (*i + 1) * 100,
            Vertical(i) => *i + 1,
        }
    }

    fn from_one_of(horz: &Option<usize>, vert: &Option<usize>) -> Self {
        match (horz, vert) {
            (Some(h), _) => Horizontal(*h),
            (_, Some(v)) => Vertical(*v),
            _ => panic!("Both horizontal and vertical"),
        }
    }
}

fn find_smudge(map: &mut Map) -> Option<usize> {
    let before = Reflection::from_one_of(&find_horizontal_mirror(map), &find_vertical_mirror(map));
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter_map(|(i, j)| {
            map[i][j] = !map[i][j];
            let possibly_new_reflection = (0..map.len())
                .filter(|row| is_mirrored_horizontally(map, *row))
                .map(Horizontal)
                .find(|after| *after != before)
                .or((0..map[0].len())
                    .filter(|col| is_mirrored_vertically(map, *col))
                    .map(Vertical)
                    .find(|after| *after != before));
            map[i][j] = !map[i][j];
            possibly_new_reflection
        })
        .next()
        .map(|reflection| reflection.value())
}

pub fn part_1(input: &str) -> Result<String> {
    let mut s = 0;
    for block in input.split("\n\n") {
        let map = parse(block);
        s += find_vertical_mirror(&map)
            .map(|i| (i + 1) as i32)
            .unwrap_or(0);
        s += find_horizontal_mirror(&map)
            .map(|i| (i + 1) as i32)
            .unwrap_or(0)
            * 100;
    }
    Ok(s.to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let mut s = 0;
    for block in input.split("\n\n") {
        let mut map = parse(block);
        s += find_smudge(&mut map).with_context(|| anyhow!("Unable to find smudge:\n{block}"))?;
    }
    Ok(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const HORZ_EX: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    const VERT_EX: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    #[test]
    fn test_horz() {
        let map = parse(HORZ_EX);
        assert!(is_mirrored_horizontally(&map, 3));
        assert!(!is_mirrored_horizontally(&map, 1));
        assert!(!is_mirrored_horizontally(&map, 0));
        assert!(!is_mirrored_horizontally(&map, 6));
        assert_eq!(find_horizontal_mirror(&map), Some(3));
    }
    #[test]
    fn test_vert() {
        let map = parse(VERT_EX);
        assert!(is_mirrored_vertically(&map, 4));
        assert_eq!(find_vertical_mirror(&map), Some(4));
        assert!(!is_mirrored_vertically(&map, 8));
        assert!(!is_mirrored_vertically(&map, 7));
        assert!(!is_mirrored_vertically(&map, 3));
    }
    #[test]
    fn test_part_one() {
        let mut ex = String::new();
        ex.push_str(VERT_EX);
        ex.push_str("\n\n");
        ex.push_str(HORZ_EX);
        assert_eq!(part_1(ex.as_str()).unwrap(), "405".to_string());
    }
    #[test]
    fn test_part_two() {
        let mut ex = String::new();
        ex.push_str(VERT_EX);
        ex.push_str("\n\n");
        ex.push_str(HORZ_EX);
        assert_eq!(part_2(ex.as_str()).unwrap(), "400".to_string());
    }
}
