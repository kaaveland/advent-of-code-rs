use anyhow::Result;
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use std::ops::Range;

fn to_grid(input: &str) -> Vec<&[u8]> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect()
}

trait Grid2 {
    fn value(&self, x: i32, y: i32) -> Option<u8>;
    fn rows(&self) -> Range<i32>;
    fn cols(&self) -> Range<i32>;

    fn val_match<P>(&self, x: i32, y: i32, p: P) -> bool
    where
        P: Fn(&u8) -> bool,
    {
        self.value(x, y).map(|ch| p(&ch)).unwrap_or(false)
    }

    fn is_num(&self, x: i32, y: i32) -> bool {
        self.val_match(x, y, u8::is_ascii_digit)
    }
    fn is_space(&self, x: i32, y: i32) -> bool {
        self.val_match(x, y, |&ch| ch == b'.')
    }
    fn is_sym(&self, x: i32, y: i32) -> bool {
        self.val_match(x, y, |&ch| !(ch.is_ascii_digit() || ch == b'.'))
    }
}

impl Grid2 for Vec<&[u8]> {
    fn value(&self, x: i32, y: i32) -> Option<u8> {
        if (0..self.len()).contains(&(y as usize)) && (0..self[0].len()).contains(&(x as usize)) {
            Some(self[y as usize][x as usize])
        } else {
            None
        }
    }
    fn rows(&self) -> Range<i32> {
        0..(self.len() as i32)
    }

    fn cols(&self) -> Range<i32> {
        0..(self[0].len() as i32)
    }
}

fn number_locations(grid: &Vec<&[u8]>) -> Vec<(i32, HashSet<(i32, i32)>)> {
    let mut numbers = vec![];
    for y in grid.rows() {
        let mut x = 0;
        while grid.cols().contains(&x) {
            while grid.is_space(x, y) || grid.is_sym(x, y) {
                x += 1;
            }
            if !grid.cols().contains(&x) {
                break;
            } else {
                let start = x;
                let mut n: i32 = 0;
                while grid.is_num(x, y) {
                    n = n * 10 + grid.value(x, y).map(|ch| ch - b'0').unwrap_or(0) as i32;
                    x += 1;
                }
                numbers.push((
                    n,
                    ((start - 1)..=x)
                        .flat_map(|x| (-1..=1).map(move |dy| (x, y + dy)))
                        .collect(),
                ));
            }
        }
    }
    numbers
}

pub fn part_1(input: &str) -> Result<String> {
    let grid = to_grid(input);
    let number_locs = number_locations(&grid);
    let part_number_sum = number_locs
        .iter()
        .filter(|(_, pos)| pos.iter().any(|(x, y)| grid.is_sym(*x, *y)))
        .map(|(n, _)| *n)
        .sum::<i32>();
    Ok(part_number_sum.to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let grid = to_grid(input);
    let number_locs = number_locations(&grid);
    let mut gear_ratio_sum = 0;

    for (y, x) in grid.rows().cartesian_product(grid.cols()) {
        if grid.value(x, y) != Some(b'*') {
            continue;
        }
        let gear_numbers = number_locs
            .iter()
            .filter(|(_, pos)| pos.contains(&(x, y)))
            .map(|(n, _)| *n)
            .collect_vec();
        if gear_numbers.len() == 2 {
            gear_ratio_sum += gear_numbers[0] * gear_numbers[1];
        }
    }

    Ok(gear_ratio_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    #[test]
    fn test_p1() {
        assert_eq!(part_1(EX).unwrap(), "4361".to_string());
    }
    #[test]
    fn test_p2() {
        assert_eq!(part_2(EX).unwrap(), "467835");
    }
}
