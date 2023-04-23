use crate::intcode::Program;
use anyhow::{Context, Result};

fn affected_by_beam(x: i64, y: i64, prog: &Program) -> Result<i64> {
    let mut prog = prog.clone();
    prog.input(x);
    prog.input(y);
    prog.exec()?;
    Ok(prog.output().last().copied().unwrap_or(0))
}

pub fn part_1(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Empty input")?)?;
    let s: i64 = (0..50)
        .filter_map(|y| min_max_x_at(y, &prog).map(|(left, right)| right - left + 1))
        .sum();
    Ok(format!("{s}"))
}

fn min_max_x_at(y: i64, prog: &Program) -> Option<(i64, i64)> {
    let in_tractor = |x: i64| affected_by_beam(x, y, prog).map(|p| p == 1).ok();
    // Do exhaustive search for small y
    if y < 20 {
        let in_beam = (0..=y).filter(|x| affected_by_beam(*x, y, prog).ok() == Some(1));
        in_beam
            .clone()
            .min()
            .and_then(|left| in_beam.max().map(|right| (left, right)))
    } else {
        let mut right = 9 * y / 10;
        let mut left = 7 * y / 10;
        while !in_tractor(left)? {
            left += 1;
        }
        while !in_tractor(right)? {
            right -= 1;
        }
        Some((left, right))
    }
}

pub fn part_2(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Empty input")?)?;
    let mut y = 512;
    let in_beam = |x: i64, y: i64| affected_by_beam(x, y, &prog).map(|p| p == 1);
    let mut x = 0;

    loop {
        while !in_beam(x, y)? {
            x += 1;
        }
        if in_beam(x + 99, y - 99)? {
            return Ok(format!("{}", 10_000 * x + (y - 99)));
        }
        y += 1;
    }
}
