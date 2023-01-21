use anyhow::{anyhow, Result};
use std::fmt::{Debug, Formatter, Write};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Occupied,
    Empty,
    Floor,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Tile::*;
        let ch = match self {
            Floor => '.',
            Occupied => '#',
            Empty => 'L',
        };
        f.write_char(ch)
    }
}

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

type WaitingArea = (usize, usize, Vec<Tile>);

fn parse(input: &str) -> Result<WaitingArea> {
    use Tile::*;
    let out: Result<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                'L' => Ok(Empty),
                '.' => Ok(Floor),
                _ => Err(anyhow!("Illegal char: {ch}")),
            })
        })
        .collect();
    out.map(|v: Vec<Tile>| {
        let height = input.lines().filter(|line| !line.is_empty()).count();
        let width = v.len() / height;
        (width, height, v)
    })
}

#[inline]
fn to_xy(n: usize, width: usize) -> (i32, i32) {
    (n.rem_euclid(width) as i32, (n / width) as i32)
}

fn from_xy(x: i32, y: i32, width: usize, height: usize) -> Option<usize> {
    let i = x + y * (width as i32);
    if x >= 0 && x < (width as i32) && i >= 0 && i < (width * height) as i32 {
        Some(i as usize)
    } else {
        None
    }
}

fn count_occupied_neighbours(area: &[Tile], n: usize, width: usize, height: usize) -> usize {
    use Tile::*;
    let (x, y) = to_xy(n, width);
    let neighbours = NEIGHBOURS
        .iter()
        .cloned()
        .filter_map(|(dx, dy)| from_xy(x + dx, y + dy, width, height))
        .filter_map(|n| area.get(n));
    neighbours.filter(|&&tile| tile == Occupied).count()
}

fn next_waiting_area(current: &WaitingArea) -> WaitingArea {
    use Tile::*;
    let (width, height, area) = current;
    let next = (0..area.len())
        .map(|i| {
            let here = area[i];
            let n = count_occupied_neighbours(area, i, *width, *height);
            match here {
                Floor => Floor,
                Empty if n == 0 => Occupied,
                Occupied if n >= 4 => Empty,
                other => other,
            }
        })
        .collect();
    (*width, *height, next)
}

fn solve(input: &str, make_next: fn(&WaitingArea) -> WaitingArea) -> Result<usize> {
    let mut waiting_area = parse(input)?;
    loop {
        let next = make_next(&waiting_area);
        if waiting_area == next {
            return Ok(next
                .2
                .iter()
                .filter(|&&tile| tile == Tile::Occupied)
                .count());
        } else {
            waiting_area = next;
        }
    }
}

pub fn part_1(input: &str) -> Result<String> {
    solve(input, next_waiting_area).map(|n| format!("{n}"))
}

fn scan(
    area: &[Tile],
    width: usize,
    height: usize,
    mut x: i32,
    mut y: i32,
    dx: i32,
    dy: i32,
) -> Option<Tile> {
    let width = width as i32;
    let height = height as i32;

    while (0..width).contains(&(x + dx)) && (0..height).contains(&(y + dy)) {
        x += dx;
        y += dy;
        let n = from_xy(x, y, width as usize, height as usize)?;
        if area[n] != Tile::Floor {
            return Some(area[n]);
        }
    }
    None
}

fn count_occupied_neighbours_2(area: &[Tile], n: usize, width: usize, height: usize) -> usize {
    use Tile::*;
    let (x, y) = to_xy(n, width);
    let neighbours = NEIGHBOURS
        .iter()
        .cloned()
        .filter_map(|(dx, dy)| scan(area, width, height, x, y, dx, dy));
    neighbours.filter(|&tile| tile == Occupied).count()
}

fn next_waiting_area_2(current: &WaitingArea) -> WaitingArea {
    use Tile::*;
    let (width, height, area) = current;
    let next: Vec<_> = (0..area.len())
        .map(|i| {
            let here = area[i];
            let n = count_occupied_neighbours_2(area, i, *width, *height);
            match here {
                Floor => Floor,
                Empty if n == 0 => Occupied,
                Occupied if n >= 5 => Empty,
                other => other,
            }
        })
        .collect();
    (*width, *height, next)
}

pub fn part_2(input: &str) -> Result<String> {
    solve(input, next_waiting_area_2).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let n = solve(EXAMPLE, next_waiting_area_2).unwrap();
        assert_eq!(n, 26);
    }

    #[test]
    fn test_1() {
        let n = solve(EXAMPLE, next_waiting_area).unwrap();
        assert_eq!(n, 37);
    }

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
}
