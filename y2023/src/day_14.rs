use anyhow::Result;
use fxhash::FxHashMap as Map;
use fxhash::FxHashSet as Set;

use shared::grid_parser;
use std::collections::VecDeque;

type Grid = (Set<(i32, i32)>, Set<(i32, i32)>, i32);

fn parse(input: &str) -> Grid {
    let immobile = grid_parser(input, &|ch| if ch == '#' { Some(ch) } else { None })
        .map(|(c, _)| c)
        .collect();
    let mobile = grid_parser(input, &|ch| if ch == 'O' { Some(ch) } else { None })
        .map(|(c, _)| c)
        .collect();
    (
        immobile,
        mobile,
        input.lines().filter(|line| !line.is_empty()).count() as i32,
    )
}
fn negate(direction: (i32, i32)) -> (i32, i32) {
    (-direction.0, -direction.1)
}

fn add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
fn tilt_board(grid: &mut Grid, direction: (i32, i32)) {
    fn is_free(pos: (i32, i32), grid: &Grid) -> bool {
        !(grid.1.contains(&pos) || grid.0.contains(&pos))
    }

    let opposite = negate(direction);

    let next = |pos: (i32, i32)| add(pos, direction);
    let prev = |pos: (i32, i32)| add(pos, opposite);

    let bounds = 0..grid.2;

    let mut work: VecDeque<_> = grid
        .1
        .iter()
        .filter(|&pos| is_free(next(*pos), grid))
        .copied()
        .collect();

    while let Some(mut item) = work.pop_front() {
        grid.1.remove(&item);
        while bounds.contains(&next(item).0)
            && bounds.contains(&next(item).1)
            && is_free(next(item), grid)
        {
            if grid.1.contains(&prev(item)) {
                work.push_back(prev(item));
            }
            item = next(item);
        }
        grid.1.insert(item);
    }
}

fn count_load(grid: &Grid) -> i32 {
    grid.1.iter().map(|rock| grid.2 - rock.1).sum()
}

fn tilt_north(input: &str) -> i32 {
    let mut grid = parse(input);
    tilt_board(&mut grid, (0, -1));
    count_load(&grid)
}
pub fn part_1(input: &str) -> Result<String> {
    Ok(tilt_north(input).to_string())
}

fn spin_cycle(grid: &mut Grid) {
    tilt_board(grid, (0, -1)); // north
    tilt_board(grid, (-1, 0)); // west
    tilt_board(grid, (0, 1)); // south
    tilt_board(grid, (1, 0)); // east
}

fn footprint(grid: &Grid) -> Vec<(i32, i32)> {
    let mut xy: Vec<_> = grid.1.iter().copied().collect();
    xy.sort();
    xy
}

fn find_cycle_in_spin_cycle(input: &str) -> i32 {
    let mut grid = parse(input);
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
