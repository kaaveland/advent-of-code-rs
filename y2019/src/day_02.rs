use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
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

fn read_addr(addr: i32, prog: &[i32], mem: &HashMap<i32, i32>) -> i32 {
    mem.get(&addr)
        .or_else(|| prog.get(addr as usize))
        .copied()
        .unwrap()
}

fn run_intcode_program(prog: &[i32]) -> Option<i32> {
    let mut mem: HashMap<i32, i32> = HashMap::default();
    let mut instruction_pointer = 0;

    loop {
        let opcode = read_addr(instruction_pointer, prog, &mem);

        match opcode {
            99 => {
                break;
            }
            1 | 2 => {
                let lhs_addr = read_addr(instruction_pointer + 1, prog, &mem);
                let lhs_val = read_addr(lhs_addr, prog, &mem);
                let rhs_addr = read_addr(instruction_pointer + 2, prog, &mem);
                let rhs_val = read_addr(rhs_addr, prog, &mem);
                let mem_dest = read_addr(instruction_pointer + 3, prog, &mem);
                let val = if opcode == 1 {
                    lhs_val + rhs_val
                } else {
                    lhs_val * rhs_val
                };
                mem.insert(mem_dest, val);
            }
            _ => panic!("Unexpected instruction: {opcode}"),
        }
        instruction_pointer += 4;
    }
    mem.get(&0).or_else(|| prog.first()).copied()
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
    use std::{assert_eq, vec};
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
