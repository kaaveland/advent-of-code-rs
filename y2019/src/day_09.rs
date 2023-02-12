use crate::intcode::Program;
use anyhow::{Context, Result};

fn run_prog_with(prog: &str, input: i64) -> Result<String> {
    let mut prog = Program::parse(prog.lines().next().context("Empty input")?)?;
    prog.input(input);
    prog.exec()?;
    Ok(format!("{:?}", prog.output()[0]))
}

pub fn part_1(input: &str) -> Result<String> {
    run_prog_with(input, 1)
}

pub fn part_2(input: &str) -> Result<String> {
    run_prog_with(input, 2)
}
