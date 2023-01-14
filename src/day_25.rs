use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::fmt::{Debug, Formatter, Write};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Tile {
    Empty,
    East,
    South,
}
type Board = Vec<Tile>;
#[derive(Eq, PartialEq, Clone)]
struct Seafloor {
    board: Board,
    width: usize,
    height: usize,
}

impl Debug for Seafloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Tile::*;
        for i in 0..self.height {
            for tile in self.board[i * self.width..].iter().take(self.width) {
                f.write_char(match tile {
                    Empty => '.',
                    East => '>',
                    South => 'v',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Result<Seafloor> {
    use Tile::*;
    let height = input.lines().filter(|line| !line.is_empty()).count();

    let board: Result<Vec<_>> = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                '.' => Ok(Empty),
                '>' => Ok(East),
                'v' => Ok(South),
                _ => Err(anyhow!("Illegal tile: {ch}")),
            })
        })
        .collect();
    let board = board?;
    let width = board.len() / height;
    Ok(Seafloor {
        board,
        height,
        width,
    })
}

#[inline]
fn next_east(src: usize, width: usize) -> usize {
    let row = src / width;
    let col = src % width;
    row * width + (col + 1).rem_euclid(width)
}

#[inline]
fn next_south(src: usize, width: usize, height: usize) -> usize {
    (src + width).rem_euclid(height * width)
}

fn step(board: &mut [Tile], width: usize, height: usize) -> bool {
    use Tile::*;
    let mut moved = false;

    let east_swaps = board
        .iter()
        .enumerate()
        .filter(|(_, t)| **t == East)
        .map(|(now, _)| (now, next_east(now, width)))
        .filter(|(_, go)| board[*go] == Empty)
        .collect_vec();

    for (src, dst) in east_swaps.into_iter() {
        moved = true;
        board[dst] = East;
        board[src] = Empty;
    }

    let south_swaps = board
        .iter()
        .enumerate()
        .filter(|(_, t)| **t == South)
        .map(|(now, _)| (now, next_south(now, width, height)))
        .filter(|(_, go)| board[*go] == Empty)
        .collect_vec();

    for (src, dst) in south_swaps.into_iter() {
        moved = true;
        board[dst] = South;
        board[src] = Empty;
    }

    moved
}

fn solve(mut seafloor: Seafloor) -> usize {
    let mut rounds = 1;
    while step(&mut seafloor.board, seafloor.width, seafloor.height) {
        rounds += 1;
    }
    rounds
}

pub fn part_1(_input: &str) -> Result<String> {
    let seafloor = parse(_input)?;
    let solution = solve(seafloor);
    Ok(format!("{solution}"))
}

pub fn part_2(_input: &str) -> Result<String> {
    Ok("Submit the answers and click the button".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east() {
        // 3 x 3:
        // 012
        // 345
        // 678
        assert_eq!(next_east(0, 3), 1);
        assert_eq!(next_east(2, 3), 0);
        assert_eq!(next_east(4, 3), 5);
        assert_eq!(next_east(5, 3), 3);
        assert_eq!(next_east(6, 3), 7);
        assert_eq!(next_east(8, 3), 6);
    }

    #[test]
    fn test_south() {
        // 3 x 3:
        // 012
        // 345
        // 678
        assert_eq!(next_south(0, 3, 3), 3);
        assert_eq!(next_south(2, 3, 3), 5);
        assert_eq!(next_south(5, 3, 3), 8);
        assert_eq!(next_south(7, 3, 3), 1);
        assert_eq!(next_south(8, 3, 3), 2);
    }

    #[test]
    fn test_horizontal() {
        let mut initial = parse("...>>>>>...").unwrap();
        step(&mut initial.board, initial.width, initial.height);
        let expect = parse("...>>>>.>..").unwrap();
        assert_eq!(expect, initial);
        step(&mut initial.board, initial.width, initial.height);
        let expect = parse("...>>>.>.>.").unwrap();
        assert_eq!(expect, initial);
    }

    #[test]
    fn test_small_example() {
        let mut initial = parse(
            "..........
.>v....v..
.......>..
..........",
        )
        .unwrap();
        step(&mut initial.board, initial.width, initial.height);
        let expect = parse(
            "..........
.>........
..v....v>.
..........
",
        )
        .unwrap();
        assert_eq!(expect, initial);
    }

    #[test]
    fn test_example() {
        let mut seafloor = parse(EXAMPLE).unwrap();
        step(&mut seafloor.board, seafloor.width, seafloor.height);
        let stepped_once = parse(STEPPED_ONCE).unwrap();
        assert_eq!(stepped_once, seafloor);
        let seafloor = parse(EXAMPLE).unwrap();
        let sol = solve(seafloor);
        assert_eq!(sol, 58);
    }

    const EXAMPLE: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
    const STEPPED_ONCE: &str = "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v";
}
