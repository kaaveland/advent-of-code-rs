use crate::intcode::{ParameterMode, Program};
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::{Finish, IResult};
use rayon::prelude::*;
use std::str::FromStr;

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), map_res(digit1, FromStr::from_str))(input)
}

fn run_intcode_program(prog: &[i32]) -> Option<i64> {
    let mut program = Program::new(prog);
    program.exec().ok()?;
    Some(program.read_addr(0, ParameterMode::Immediate))
}

pub fn part_1(input: &str) -> Result<String> {
    let (_, mut prog) = parse(input)
        .finish()
        .map_err(|err| anyhow!("Unable to parse: {err}"))?;
    prog[1] = 12;
    prog[2] = 2;
    run_intcode_program(&prog)
        .context("Unable to look up addr 0")
        .map(|n| format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let (_, prog) = parse(input)
        .finish()
        .map_err(|err| anyhow!("Unable to parse: {err}"))?;
    let options = (0..100).cartesian_product(0..100).collect_vec();
    let (noun, verb) = options
        .into_par_iter()
        .find_any(|(noun, verb)| {
            let mut modified_prog = prog.clone();
            modified_prog[1] = *noun;
            modified_prog[2] = *verb;
            run_intcode_program(&modified_prog) == Some(19690720)
        })
        .context("Unable to solve")?;
    Ok(format!("{}", 100 * noun + verb))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let prog = vec![1, 0, 0, 0, 99];
        assert_eq!(run_intcode_program(&prog), Some(2));
        let prog = vec![2, 3, 0, 3, 99];
        assert_eq!(run_intcode_program(&prog), Some(2));
        let prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(run_intcode_program(&prog), Some(30));
    }
}
