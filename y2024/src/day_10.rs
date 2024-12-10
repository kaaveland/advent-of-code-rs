use fxhash::FxHashSet;
use itertools::Itertools;
use std::collections::VecDeque;

struct Grid {
    height: i32,
    width: i32,
    grid: Vec<u8>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let lines = input.lines().filter(|l| !l.is_empty());
        let height = lines.clone().count() as i32;
        let width = lines.clone().next().map(|l| l.chars().count()).unwrap_or(0) as i32;
        let grid = input
            .lines()
            .flat_map(|line| line.trim().as_bytes().iter().map(|b| *b - b'0'))
            .collect();
        Grid {
            height,
            width,
            grid,
        }
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }
    fn at(&self, x: i32, y: i32) -> Option<u8> {
        if self.contains(x, y) {
            Some(self.grid[(x + y * self.width) as usize])
        } else {
            None
        }
    }
    fn trailheads(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter(|(x, y)| self.at(*x, *y).unwrap_or(1) == 0)
    }
}

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let map = Grid::parse(input);
    let mut score = 0;
    for (x, y) in map.trailheads() {
        let mut work = VecDeque::new();
        let mut visited = FxHashSet::default();
        work.push_back((x, y));
        while let Some((x, y)) = work.pop_front() {
            if map.contains(x, y) && visited.insert((x, y)) {
                let height = map.at(x, y).unwrap();
                for (dx, dy) in DIRS {
                    if map.at(x + dx, y + dy) == Some(height + 1) {
                        work.push_back((x + dx, y + dy));
                    }
                }
            }
        }
        score += visited
            .into_iter()
            .filter(|(x, y)| map.at(*x, *y) == Some(9))
            .count();
    }
    Ok(format!("{score}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let map = Grid::parse(input);
    let mut score = 0;
    for (x, y) in map.trailheads() {
        let mut work = VecDeque::new();
        work.push_back((x, y));
        while let Some((x, y)) = work.pop_front() {
            if map.at(x, y) == Some(9) {
                score += 1;
            }
            if map.contains(x, y) {
                let height = map.at(x, y).unwrap();
                for (dx, dy) in DIRS {
                    if map.at(x + dx, y + dy) == Some(height + 1) {
                        work.push_back((x + dx, y + dy));
                    }
                }
            }
        }
    }
    Ok(format!("{score}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    #[test]
    fn test_p1() {
        assert_eq!(part_1(EXAMPLE).unwrap(), "36");
    }
    #[test]
    fn test_p2() {
        assert_eq!(part_2(EXAMPLE).unwrap(), "81");
    }
}
