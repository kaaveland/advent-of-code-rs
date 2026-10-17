use itertools::Itertools;
use std::mem::swap;

fn next_row(cur: &[bool], next: &mut [bool]) {
    for i in 0..cur.len() {
        let left_is_trap = if i > 0 { cur[i - 1] } else { false };
        let center_is_trap = cur[i];
        let right_is_trap = cur.get(i + 1).copied().unwrap_or(false);
        let conv = [left_is_trap, center_is_trap, right_is_trap];
        let new_trap_shapes = [
            [true, true, false],
            [false, true, true],
            [true, false, false],
            [false, false, true],
        ];
        next[i] = new_trap_shapes.iter().contains(&conv);
    }
}

fn parse_row(cur: &[u8]) -> Vec<bool> {
    cur.trim_ascii().iter().map(|&ch| ch == b'^').collect()
}

fn count_safe_spaces(row: &[u8], it: usize) -> usize {
    let mut row = parse_row(row);
    let mut next = vec![false; row.len()];
    let mut safe = row.iter().filter(|&&is_trap| !is_trap).count();
    for _ in 0..(it - 1) {
        next_row(&row, &mut next);
        swap(&mut row, &mut next);
        safe += row.iter().filter(|&&is_trap| !is_trap).count();
    }
    safe
}

pub fn part_1(row: &str) -> anyhow::Result<String> {
    Ok(format!("{}", count_safe_spaces(row.trim().as_bytes(), 40)))
}

pub fn part_2(row: &str) -> anyhow::Result<String> {
    Ok(format!(
        "{}",
        count_safe_spaces(row.trim().as_bytes(), 400000)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_spaces() {
        let inp = b".^^.^.^^^^";
        let count = count_safe_spaces(inp, 10);
        assert_eq!(count, 38);
    }
}
