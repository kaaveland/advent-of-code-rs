use crate::intcode::Program;
use anyhow::{Context, Result};

pub fn part_1(input: &str) -> Result<String> {
    let mut prog = Program::parse(input.lines().next().context("Missing line in input")?)?;
    prog.input(1);
    prog.exec()?;
    let n = prog.output().last().copied().context("No output")?;
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let mut prog = Program::parse(input.lines().next().context("Missing line in input")?)?;
    prog.input(5);
    prog.exec()?;
    let n = prog.output().last().copied().context("No output")?;
    Ok(format!("{n}"))
}
