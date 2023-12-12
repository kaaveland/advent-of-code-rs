use anyhow::Result;
use fxhash::FxHashSet as Set;
use shared::grid_parser;

type Coord2 = [i64; 2];

fn parse_galaxies(input: &str) -> Vec<Coord2> {
    grid_parser(input, &|ch: char| match ch {
        '#' => Some('#'),
        _ => None,
    })
    .map(|((x, y), _)| [x, y])
    .collect()
}

fn upper_bounds(galaxies: &[Coord2]) -> Coord2 {
    galaxies.iter().fold([0, 0], |[max_x, max_y], &[x, y]| {
        [x.max(max_x), y.max(max_y)]
    })
}

type ExpandingSpace = (Set<i64>, Set<i64>);
fn expanding_space(galaxies: &[Coord2]) -> ExpandingSpace {
    let [max_x, max_y] = upper_bounds(galaxies);
    let mut rows: Set<_> = (0..=max_x).collect();
    let mut cols: Set<_> = (0..=max_y).collect();
    galaxies.iter().map(|[x, _]| x).for_each(|x| {
        rows.remove(x);
    });
    galaxies.iter().map(|[_, y]| y).for_each(|y| {
        cols.remove(y);
    });
    (rows, cols)
}

fn manhattan(
    left: Coord2,
    right: Coord2,
    expanding_space: &ExpandingSpace,
    expanding_space_factor: i64,
) -> i64 {
    let dist = (left[0] - right[0]).abs() + (left[1] - right[1]).abs();
    let xr = left[0].min(right[0])..left[0].max(right[0]);
    let yr = left[1].min(right[1])..left[1].max(right[1]);
    let rows = expanding_space.0.iter().filter(|x| xr.contains(x)).count() as i64;
    let cols = expanding_space.1.iter().filter(|y| yr.contains(y)).count() as i64;
    dist + (rows + cols) * expanding_space_factor
}

fn all_distances(input: &str, expanding_space_factor: i64) -> i64 {
    let galaxies = parse_galaxies(input);
    let expanding_space = expanding_space(&galaxies);
    let mut s = 0;
    for i in 0..galaxies.len() {
        for j in 0..i {
            s += manhattan(
                galaxies[i],
                galaxies[j],
                &expanding_space,
                expanding_space_factor,
            );
        }
    }
    s
}

pub fn part_1(input: &str) -> Result<String> {
    Ok(all_distances(input, 1).to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    Ok(all_distances(input, 1_000_000 - 1).to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    #[test]
    fn test_solve_1() {
        assert_eq!(all_distances(EX, 1), 374);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(all_distances(EX, 9), 1030);
        assert_eq!(all_distances(EX, 99), 8410);
    }
}
