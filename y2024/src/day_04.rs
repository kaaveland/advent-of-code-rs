use itertools::Itertools;

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.trim().as_bytes().to_vec())
        .collect()
}

const DIRS: [[i32; 2]; 8] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
    [1, 1],
    [-1, -1],
    [1, -1],
    [-1, 1],
];

fn count_xmas(grid: &[Vec<u8>]) -> usize {
    let mut count = 0;
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    for (x, y) in (0..w).cartesian_product(0..h) {
        for [dx, dy] in DIRS {
            let mut buf = [0; 4];
            for i in 0..4 {
                let (xp, yp) = (x + i * dx, y + i * dy);
                if (0..w).contains(&xp) && (0..h).contains(&yp) {
                    buf[i as usize] = grid[yp as usize][xp as usize];
                }
            }
            if buf == [b'X', b'M', b'A', b'S'] {
                count += 1;
            }
        }
    }
    count
}

fn count_mas(grid: &[Vec<u8>]) -> usize {
    let mut count = 0;
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let acceptable = [[b'M', b'A', b'S'], [b'S', b'A', b'M']];

    for (x, y) in (0..w).cartesian_product(0..h) {
        let mut left = [0; 3];
        let mut right = [0; 3];
        for i in 0..3 {
            let (xp, yp) = (x + i, y + i);
            if (0..w).contains(&xp) && (0..h).contains(&yp) {
                right[i as usize] = grid[yp as usize][xp as usize];
            }
            let (xp, yp) = (x + 2 - i, y + i);
            if (0..w).contains(&xp) && (0..h).contains(&yp) {
                left[i as usize] = grid[yp as usize][xp as usize];
            }
        }
        if acceptable.contains(&left) && acceptable.contains(&right) {
            count += 1;
        }
    }

    count
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let grid = parse_grid(input);
    let n = count_xmas(&grid);
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let grid = parse_grid(input);
    let n = count_mas(&grid);
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn test_p1() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(count_xmas(&grid), 18);
    }
    #[test]
    fn test_p2() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(count_mas(&grid), 9);
    }
}
