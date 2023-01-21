use anyhow::{anyhow, Context, Result};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Tile {
    Tree,
    Empty,
}
#[derive(Eq, PartialEq, Debug, Clone)]
struct Area {
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
}

fn parse(input: &str) -> Result<Area> {
    let width = input
        .lines()
        .next()
        .with_context(|| anyhow!("Empty input"))?
        .len();
    let tiles: Result<Vec<Tile>> = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                '.' => Ok(Tile::Empty),
                '#' => Ok(Tile::Tree),
                _ => Err(anyhow!("Illegal char: {ch}")),
            })
        })
        .collect();
    let tiles = tiles?;
    let height = tiles.len() / width;
    Ok(Area {
        width,
        height,
        tiles,
    })
}

fn traverse(area: &Area, dx: usize, dy: usize) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut trees = 0;
    while y < area.height {
        x = (x + dx).rem_euclid(area.width);
        y += dy;
        if matches!(area.tiles.get(x + y * area.width), Some(Tile::Tree)) {
            trees += 1;
        }
    }
    trees
}

pub fn part_1(input: &str) -> Result<String> {
    let area = parse(input)?;
    let trees_visited = traverse(&area, 3, 1);
    Ok(format!("{trees_visited}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let area = parse(input)?;
    let paths = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let answer: i32 = paths
        .into_iter()
        .map(|(dx, dy)| traverse(&area, dx, dy))
        .product();
    Ok(format!("{answer}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(part_1(EXAMPLE).unwrap(), "7".to_string());
    }
    #[test]
    fn test_p2() {
        assert_eq!(part_2(EXAMPLE).unwrap(), "336".to_string());
    }
    const EXAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}
