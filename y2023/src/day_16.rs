use crate::day_16::Dir::{East, North, South, West};
use anyhow::Result;
use fxhash::FxHashSet as Set;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

fn add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}
impl Dir {
    fn to_vec2(self) -> (i32, i32) {
        use Dir::*;
        match self {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0),
        }
    }
}
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Beam {
    dir: Dir,
    pos: (i32, i32),
}

trait Grid {
    fn at(&self, x: i32, y: i32) -> Option<Tile>;
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct VecGrid {
    tiles: Vec<Tile>,
    height: i32,
    width: i32,
}

impl VecGrid {
    fn new(input: &str) -> Self {
        use Tile::*;
        let tiles: Vec<_> = input
            .chars()
            .filter_map(|ch| match ch {
                '.' => Some(Empty),
                '/' => Some(ForwardMirror),
                '\\' => Some(BackwardMirror),
                '|' => Some(VerticalSplitter),
                '-' => Some(HorizontalSplitter),
                _ => None,
            })
            .collect();
        let height = input.lines().count() as i32;
        let width = (tiles.len() as i32) / height;
        assert_eq!(height * width, tiles.len() as i32);
        VecGrid {
            tiles,
            height,
            width,
        }
    }
}

impl Grid for VecGrid {
    fn at(&self, x: i32, y: i32) -> Option<Tile> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(self.tiles[(x + self.width * y) as usize])
        } else {
            None
        }
    }
}

type BeamPlaces = Set<Beam>;

fn single_beam(tile: Tile, dir: Dir) -> Option<Dir> {
    use Dir::*;
    use Tile::*;
    match (tile, dir) {
        (ForwardMirror, East) => Some(North),
        (ForwardMirror, West) => Some(South),
        (ForwardMirror, North) => Some(East),
        (ForwardMirror, South) => Some(West),
        (BackwardMirror, East) => Some(South),
        (BackwardMirror, West) => Some(North),
        (BackwardMirror, North) => Some(West),
        (BackwardMirror, South) => Some(East),
        (Empty, d) => Some(d),
        (VerticalSplitter, North) => Some(North),
        (VerticalSplitter, South) => Some(South),
        (HorizontalSplitter, East) => Some(East),
        (HorizontalSplitter, West) => Some(West),
        _ => None,
    }
}

fn dual_beam(tile: Tile, dir: Dir) -> Option<[Dir; 2]> {
    use Dir::*;
    use Tile::*;
    match (tile, dir) {
        (VerticalSplitter, West) => Some([North, South]),
        (VerticalSplitter, East) => Some([North, South]),
        (HorizontalSplitter, North) => Some([West, East]),
        (HorizontalSplitter, South) => Some([West, East]),
        _ => None,
    }
}
fn energize_grid(grid: &dyn Grid, inital_beam: Beam) -> usize {
    let mut work = Vec::new();
    let mut cache = BeamPlaces::default();
    let mut places = Set::default();
    work.push(inital_beam);
    while let Some(Beam { pos, dir }) = work.pop() {
        if let Some(tile) = grid.at(pos.0, pos.1) {
            cache.insert(Beam { pos, dir });
            places.insert(pos);

            if let Some(next_dir) = single_beam(tile, dir) {
                let next = Beam {
                    pos: add(pos, next_dir.to_vec2()),
                    dir: next_dir,
                };
                if grid.at(next.pos.0, next.pos.1).is_some() && cache.insert(next) {
                    work.push(next);
                }
            } else if let Some(split) = dual_beam(tile, dir) {
                for next_dir in split {
                    let next = Beam {
                        pos: add(pos, next_dir.to_vec2()),
                        dir: next_dir,
                    };
                    if grid.at(next.pos.0, next.pos.1).is_some() && cache.insert(next) {
                        work.push(next);
                    }
                }
            }
        }
    }
    places.len()
}

fn find_maximal_grid_energy(grid: &VecGrid) -> usize {
    let candidates = (0..grid.height)
        .map(|y| Beam {
            pos: (0, y),
            dir: East,
        })
        .chain((0..grid.height).map(|y| Beam {
            pos: (grid.width - 1, y),
            dir: West,
        }))
        .chain((0..grid.width).map(|x| Beam {
            pos: (x, 0),
            dir: South,
        }))
        .chain((0..grid.width).map(|x| Beam {
            pos: (x, grid.height - 1),
            dir: North,
        }))
        .collect_vec();
    candidates
        .into_par_iter()
        .map(|beam| energize_grid(grid, beam))
        .max()
        .unwrap_or(0)
}

pub fn part_1(input: &str) -> Result<String> {
    let grid = VecGrid::new(input);
    Ok(energize_grid(
        &grid,
        Beam {
            pos: (0, 0),
            dir: East,
        },
    )
    .to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let grid = VecGrid::new(input);
    Ok(find_maximal_grid_energy(&grid).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap(), "46".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EX).unwrap(), "51".to_string());
    }
}
