use anyhow::Result;
use fxhash::FxHashMap as Map;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Tile {
    Empty,
    Round,
    Square,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Tiles {
    tiles: Vec<Tile>,
    height: i32,
    width: i32,
}

impl Tiles {
    fn at(&self, x: i32, y: i32) -> Tile {
        assert!(self.bounds(x, y), "{x}, {y}");
        self.tiles[(x + self.width * y) as usize]
    }
    fn set(&mut self, x: i32, y: i32, tile: Tile) {
        assert!(self.bounds(x, y), "{x}, {y}");
        self.tiles[(x + self.width * y) as usize] = tile;
    }
    fn take_round(&mut self, x: i32, y: i32) {
        self.set(x, y, Tile::Empty);
    }
    fn put_round(&mut self, x: i32, y: i32) {
        self.set(x, y, Tile::Round);
    }
    fn bounds(&self, x: i32, y: i32) -> bool {
        (0..self.height).contains(&y) && (0..self.width).contains(&x)
    }
    fn is_empty(&self, x: i32, y: i32) -> bool {
        self.bounds(x, y) && matches!(self.at(x, y), Tile::Empty)
    }
    fn is_round(&self, x: i32, y: i32) -> bool {
        self.bounds(x, y) && matches!(self.at(x, y), Tile::Round)
    }
    fn parse(input: &str) -> Self {
        let tiles: Vec<_> = input
            .chars()
            .filter(|ch| *ch != '\n')
            .map(|ch| match ch {
                '#' => Tile::Square,
                'O' => Tile::Round,
                '.' => Tile::Empty,
                _ => panic!("Unexpected {ch}"),
            })
            .collect();
        let height = input.lines().count() as i32;
        let width = (tiles.len() as i32) / height;
        assert_eq!(height * width, tiles.len() as i32);
        Tiles {
            tiles,
            height,
            width,
        }
    }
    fn round(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter_map(|(x, y)| match self.at(x, y) {
                Tile::Round => Some((x, y)),
                _ => None,
            })
    }
    fn tilt_board(&mut self, direction: (i32, i32)) {
        let opposite = negate(direction);
        let next = |pos: (i32, i32)| add(pos, direction);
        let prev = |pos: (i32, i32)| add(pos, opposite);
        let is_free = |(x, y), grid: &Tiles| grid.is_empty(x, y);

        let mut work: Vec<_> = self.round().collect();

        while let Some(mut item) = work.pop() {
            if !self.is_round(item.0, item.1) {
                continue;
            }
            self.take_round(item.0, item.1);
            while self.bounds(next(item).0, next(item).1) && is_free(next(item), self) {
                let (x, y) = prev(item);
                if self.is_round(x, y) {
                    work.push((x, y));
                }
                item = next(item);
            }
            self.put_round(item.0, item.1);
        }
    }
}

fn negate(direction: (i32, i32)) -> (i32, i32) {
    mul(direction, (-1, -1))
}

fn add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

fn mul(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 * rhs.0, lhs.1 * rhs.1)
}

fn count_load(grid: &Tiles) -> i32 {
    grid.round().map(|(_, y)| grid.height - y).sum()
}

fn tilt_north(input: &str) -> i32 {
    let mut grid = Tiles::parse(input);
    grid.tilt_board((0, -1));
    count_load(&grid)
}
pub fn part_1(input: &str) -> Result<String> {
    Ok(tilt_north(input).to_string())
}

fn spin_cycle(grid: &mut Tiles) {
    grid.tilt_board((0, -1)); // north
    grid.tilt_board((-1, 0)); // west
    grid.tilt_board((0, 1)); // south
    grid.tilt_board((1, 0)); // east
}

fn footprint(grid: &Tiles) -> Vec<Tile> {
    grid.tiles.clone()
}

fn find_cycle_in_spin_cycle(input: &str) -> i32 {
    let mut grid = Tiles::parse(input);
    let mut seen: Map<_, i32> = Map::default();

    for i in 1..=1_000_000_000 {
        spin_cycle(&mut grid);
        if let Some(previously) = seen.insert(footprint(&grid), i) {
            let cycle_len = i - previously;
            let rem = 1000000000 - i;
            let whole_cycles = rem / cycle_len;
            let do_up_to = whole_cycles * cycle_len;
            let manual_cycles = rem - do_up_to;
            for _ in 0..manual_cycles {
                spin_cycle(&mut grid);
            }
            return count_load(&grid);
        }
    }

    unreachable!()
}

pub fn part_2(input: &str) -> Result<String> {
    Ok(find_cycle_in_spin_cycle(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_part_1() {
        assert_eq!(tilt_north(EX), 136);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(find_cycle_in_spin_cycle(EX), 64);
    }
}
