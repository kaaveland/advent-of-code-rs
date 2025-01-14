// shape:
//    | 1   2   3   4   5   6
// ---+---+---+---+---+---+---+
//  1 |  1   3   6  10  15  21
//  2 |  2   5   9  14  20
//  3 |  4   8  13  19
//  4 |  7  12  18
//  5 | 11  17
//  6 | 16
//
// The top row are the triangular numbers, the data is 1-indexed
// The task asks for the code at row X, column Y
// We need to translate that into finding out which natural
// number that would've been there, to figure out how many times
// to invoke the code generation. We note that the diagonal up and to the right
// increases by 1 for each 1, 1 move we do.
// Adding column to row places us at a triagonal number in the top row, but it's the
// one that is bigger than the place in the grid, so we need to subtract from that triangle
// number to go back to the place in the grid

use anyhow::Context;
use regex::Regex;

fn calculate_index(row: u64, col: u64) -> u64 {
    let n = col + row - 1;
    (n * (n + 1)) / 2 - row + 1
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"row (\d+), column (\d+)")?;
    let caps = re.captures(s).context("No match")?;
    let row = caps.get(1).unwrap().as_str().parse()?;
    let col = caps.get(2).unwrap().as_str().parse()?;
    let ix = calculate_index(row, col);

    let mut n: u64 = 20151125;
    // The calculation the task gives us is this:
    for _ in 1..ix {
        n = (n * 252533) % 33554393;
    }
    // There _should_ be a faster way to do this using modular exponentiation...
    // but leaving that as TODO
    Ok(n.to_string())
}

pub fn part_2(_: &str) -> anyhow::Result<String> {
    Ok("Collect the stars and click the button (-:".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_calculation() {
        // this is where 5 is in the example grid
        assert_eq!(calculate_index(2, 2), 5);
        assert_eq!(calculate_index(3, 3), 13);
        assert_eq!(calculate_index(2, 5), 20);
        assert_eq!(calculate_index(5, 2), 17);
    }
}
