use crate::intcode::Program;
use anyhow::{anyhow, Result};

fn line_to_inputs(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.trim_end()
        .chars()
        .map(|c| c as i64)
        .chain(std::iter::once(10))
}
fn input_script(script: &str, prog: &mut Program) {
    script.lines().for_each(|line| {
        line_to_inputs(line).for_each(|i| prog.input(i));
    });
}

// read-only registers for detecting ground
// 1 tile away: A
// 2 tiles away: B
// 3 tiles away: C
// 4 tiles away: D
// true if ground, false otherwise
// Two write-registers: T temporary value, J jump register
// J is true if we should jump, false otherwise
// 3 instructions: AND X Y => Y = X & Y, OR X Y => Y = X | Y, NOT X Y => Y = !X

pub fn part_1(input: &str) -> Result<String> {
    let mut prog = Program::parse(input.trim_end())?;
    // Only jump if D is ground and there's a hole in A, B or C
    input_script(
        "OR A J
        AND B J
        AND C J
        NOT J J
        AND D J
        WALK",
        &mut prog,
    );
    prog.exec()?;
    prog.output()
        .last()
        .map(|i| i.to_string())
        .ok_or_else(|| anyhow!("No output"))
}

// Five new registers for up to 9 tiles away: E through I

pub fn part_2(input: &str) -> Result<String> {
    let mut prog = Program::parse(input.trim_end())?;
    // Only jump if D is ground and there's a hole in A, B or C
    // In addition, don't jump if E or H are holes
    input_script(
        "OR A J
        AND B J
        AND C J
        NOT J J
        AND D J
        OR E T
        OR H T
        AND T J
        RUN",
        &mut prog,
    );
    prog.exec()?;
    prog.output()
        .last()
        .map(|i| i.to_string())
        .ok_or_else(|| anyhow!("No output"))
}
