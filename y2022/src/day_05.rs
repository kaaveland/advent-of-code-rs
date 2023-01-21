use anyhow::{anyhow, Context, Result};
use std::iter::Iterator;

pub struct Instruction {
    source: usize,
    dest: usize,
    count: usize,
}

pub struct Problem {
    state: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse_instruction(instr: &str) -> Result<Instruction> {
    let split: Vec<&str> = instr.split(' ').collect();
    if let [_move, count, _from, source, _to, dest] = split[..] {
        let src: usize = source.parse()?;
        let dst: usize = dest.parse()?;
        let cnt: usize = count.parse()?;
        Ok(Instruction {
            source: src,
            dest: dst,
            count: cnt,
        })
    } else {
        panic!("Invalid instruction")
    }
}

fn parse_problem(description: &str) -> Result<Problem> {
    let split: Vec<&str> = description.splitn(2, "\n\n").collect();
    let initial_state = *split
        .first()
        .with_context(|| anyhow!("Wrong formatting: No empty line separator"))?;
    let instructions = *split
        .get(1)
        .with_context(|| anyhow!("Wrong formatting: No empty line separator"))?;

    Ok(Problem {
        state: parse_stacks(initial_state)?,
        instructions: instructions
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(parse_instruction)
            .collect::<Result<Vec<_>>>()?,
    })
}

fn parse_stacks(initial_state: &str) -> Result<Vec<Vec<char>>> {
    let lines: Vec<&str> = initial_state.split('\n').collect();
    let stack_count = lines
        .last()
        .with_context(|| anyhow!("Wrong formatting: Empty initial_state"))?
        .split(' ')
        .filter(|s| !s.is_empty())
        .count();
    let mut out: Vec<Vec<char>> = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        out.push(Vec::new());
    }
    for line in &lines[..lines.len() - 1] {
        let line = *line;
        for (i, ch) in line.chars().enumerate() {
            if ch.is_ascii_uppercase() {
                let stack = out
                    .get_mut(i / 4)
                    .with_context(|| anyhow!("Wrong formatting: Not that many stacks"))?;
                stack.push(ch.to_owned());
            }
        }
    }
    for i in 0..stack_count {
        let v = out.get(i).unwrap();
        out[i] = v.iter().cloned().rev().collect();
    }
    Ok(out)
}

fn execute_instruction(problem: &mut Problem, instruction: usize) {
    let instr = problem
        .instructions
        .get(instruction)
        .expect("Wrong instructions index!");
    for _i in 0..instr.count {
        let source = problem
            .state
            .get_mut(instr.source - 1)
            .expect("Wrong source");
        let ch = source.pop().expect("Stack empty!");
        let dest = problem.state.get_mut(instr.dest - 1).expect("Wrong dest");
        dest.push(ch)
    }
}

fn solve_part1(problem: &mut Problem) -> String {
    for i in 0..problem.instructions.len() {
        execute_instruction(problem, i)
    }
    problem
        .state
        .iter()
        .map(|s| s.last().unwrap())
        .cloned()
        .collect()
}

fn execute_instruction_part2(problem: &mut Problem, instruction: usize) {
    let instr = problem
        .instructions
        .get(instruction)
        .expect("Wrong instructions index!");
    let source = problem
        .state
        .get_mut(instr.source - 1)
        .expect("Wrong source");
    let mut boxes: Vec<char> = Vec::with_capacity(instr.count);
    for _ in 0..instr.count {
        boxes.push(source.pop().expect("Empty stack"))
    }
    let dest = problem.state.get_mut(instr.dest - 1).expect("Wrong dest");
    for _ in 0..instr.count {
        dest.push(boxes.pop().expect("Empty move stack"));
    }
}

fn solve_part2(problem: &mut Problem) -> String {
    for i in 0..problem.instructions.len() {
        execute_instruction_part2(problem, i)
    }
    problem
        .state
        .iter()
        .map(|s| s.last().unwrap())
        .cloned()
        .collect()
}

pub fn part_1(input: &str) -> Result<String> {
    let mut problem = parse_problem(input)?;
    Ok(solve_part1(&mut problem))
}

pub fn part_2(input: &str) -> Result<String> {
    let mut problem = parse_problem(input)?;
    Ok(solve_part2(&mut problem))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let instr = parse_instruction("move 3 from 2 to 1").unwrap();
        assert_eq!(instr.count, 3);
        assert_eq!(instr.source, 2);
        assert_eq!(instr.dest, 1);
    }
}
