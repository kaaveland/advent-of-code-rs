use anyhow::{anyhow, Context, Result};
use fxhash::FxHashSet;
use itertools::Itertools;

fn row_of(bp: &str) -> i32 {
    bp[..7].chars().fold(0, |s, n| s * 2 + i32::from(n == 'B'))
}

fn col_of(bp: &str) -> i32 {
    bp[7..].chars().fold(0, |s, n| s * 2 + i32::from(n == 'R'))
}

fn seat_id(bp: &str) -> i32 {
    row_of(bp) * 8 + col_of(bp)
}

pub fn part_1(input: &str) -> Result<String> {
    let m = input
        .lines()
        .map(seat_id)
        .max()
        .with_context(|| anyhow!("No boarding passes"));
    m.map(|n| format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let seats: FxHashSet<_> = input.lines().map(seat_id).sorted().collect();
    let low = *seats
        .iter()
        .find(|&&n| seats.contains(&(n + 2)) && !seats.contains(&(n + 1)))
        .with_context(|| anyhow!("Missing seat not found"))?;
    Ok(format!("{}", low + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_ex() {
        let bp = "FBFBBFFRLR";
        assert_eq!(row_of(bp), 44);
    }

    #[test]
    fn test_seat_ex() {
        let bp = "FBFBBFFRLR";
        assert_eq!(col_of(bp), 5);
    }
}
