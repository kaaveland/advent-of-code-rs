use crate::day_17::OperandKind::{Combo, Discard, Lit};
use anyhow::{anyhow, Context};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res, success};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum OperandKind {
    Lit,
    Combo,
    Discard,
}

impl Instruction {
    fn operand_kind(&self) -> OperandKind {
        use Instruction::*;
        match self {
            Adv | Bdv | Cdv | Bst | Out => Combo,
            Bxl | Jnz => Lit,
            Bxc => Discard,
        }
    }
}

impl TryFrom<i64> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: i64) -> Result<Self, Self::Error> {
        match opcode {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => Err(anyhow!("Unknown opcode: {opcode}")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: i64,
    program: Vec<i64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Register {
    A(i64),
    B(i64),
    C(i64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Program {
    instructions: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Parse {
    Register(Register),
    Program(Program),
    Skip,
}

fn posint(s: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(s)
}

fn int(s: &str) -> IResult<&str, i64> {
    alt((map(preceded(char('-'), posint), |x| -x), posint))(s)
}

fn parse_register(s: &str) -> IResult<&str, Register> {
    preceded(
        tag("Register "),
        alt((
            map(preceded(char('A'), preceded(tag(": "), int)), |x| {
                Register::A(x)
            }),
            map(preceded(char('B'), preceded(tag(": "), int)), |x| {
                Register::B(x)
            }),
            map(preceded(char('C'), preceded(tag(": "), int)), |x| {
                Register::C(x)
            }),
        )),
    )(s)
}

fn parse_program(s: &str) -> IResult<&str, Program> {
    map(
        preceded(tag("Program: "), separated_list1(char(','), int)),
        |instructions| Program { instructions },
    )(s)
}

fn line(s: &str) -> IResult<&str, Parse> {
    alt((
        map(parse_register, Parse::Register),
        map(parse_program, Parse::Program),
        success(Parse::Skip),
    ))(s)
}

impl TryFrom<&str> for Computer {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![],
        };
        let (_, parse) =
            separated_list1(char('\n'), line)(value).map_err(|err| anyhow!("{err}"))?;
        for instr in parse {
            match instr {
                Parse::Register(Register::A(v)) => {
                    computer.register_a = v;
                }
                Parse::Register(Register::B(v)) => {
                    computer.register_b = v;
                }
                Parse::Register(Register::C(v)) => {
                    computer.register_c = v;
                }
                Parse::Program(program) => {
                    computer.program = program.instructions;
                }
                Parse::Skip => {}
            }
        }
        Ok(computer)
    }
}

impl Computer {
    fn cycle(&mut self, output: &mut Vec<i64>) -> anyhow::Result<bool> {
        if self.instruction_pointer >= self.program.len() as i64 {
            Ok(true)
        } else {
            let mut jump = false;
            let instruction: Instruction =
                self.program[self.instruction_pointer as usize].try_into()?;
            let operand = self.program[self.instruction_pointer as usize + 1];
            let operand = match instruction.operand_kind() {
                Combo if (0..=3).contains(&operand) => Ok(operand),
                Combo if operand == 4 => Ok(self.register_a),
                Combo if operand == 5 => Ok(self.register_b),
                Combo if operand == 6 => Ok(self.register_c),
                Combo if operand == 7 => Err(anyhow!(
                    "Combo operand {operand} not implemented but is reserved"
                )),
                Discard => Ok(0),
                Lit => Ok(operand),
                _ => Err(anyhow!("Combo operand {operand} is unspecified")),
            }?;
            match instruction {
                Instruction::Adv | Instruction::Bdv | Instruction::Cdv => {
                    let numerator = self.register_a;
                    assert!(operand >= 0);
                    let result = numerator / 2i64.pow(operand as u32);
                    match instruction {
                        Instruction::Adv => {
                            self.register_a = result;
                        }
                        Instruction::Bdv => {
                            self.register_b = result;
                        }
                        Instruction::Cdv => {
                            self.register_c = result;
                        }
                        _ => unreachable!(),
                    }
                }
                Instruction::Bxl => {
                    self.register_b ^= operand;
                }
                Instruction::Bst => {
                    self.register_b = operand.rem_euclid(8);
                }
                Instruction::Jnz => {
                    if self.register_a != 0 {
                        self.instruction_pointer = operand;
                        jump = true;
                    }
                }
                Instruction::Bxc => {
                    self.register_b ^= self.register_c;
                }
                Instruction::Out => {
                    output.push(operand.rem_euclid(8));
                }
            }

            if !jump {
                self.instruction_pointer += 2;
            }

            Ok(false)
        }
    }
}

fn run_program(computer: &mut Computer) -> anyhow::Result<Vec<i64>> {
    let mut output = vec![];
    while !computer.cycle(&mut output)? {}
    Ok(output)
}

fn run_program_with_a_reg(computer: &mut Computer, a_reg: i64) -> anyhow::Result<Vec<i64>> {
    computer.register_a = a_reg;
    computer.instruction_pointer = 0;
    computer.register_b = 0;
    computer.register_c = 0;
    let mut output = vec![];
    while !computer.cycle(&mut output)? {}
    Ok(output)
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let mut computer = Computer::try_from(input)?;
    let output = run_program(&mut computer)?;
    Ok(output
        .into_iter()
        .map(|n| format!("{n}"))
        .join(",")
        .to_string())
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let mut computer = Computer::try_from(input)?;
    let mut work = VecDeque::from([0]);
    let prog = computer.program.clone().into_iter().rev().collect_vec();

    for require in prog {
        for _ in 0..work.len() {
            // safe: we're in a loop of work.len() and don't pop anywhere else
            let val = work.pop_front().unwrap();

            for oct_digit in 0..8 {
                let reg_candidate = (val << 3) | oct_digit;
                let output = run_program_with_a_reg(&mut computer, reg_candidate)?;
                if output.first() == Some(&require) {
                    work.push_back(reg_candidate);
                }
            }
        }
    }
    let sol = work.pop_front().context("Unable to solve")?;
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test_parse() {
        let parsed = Computer::try_from(EXAMPLE).unwrap();
        assert_eq!(parsed.register_a, 729);
        assert_eq!(parsed.register_b, 0);
        assert_eq!(parsed.register_c, 0);
        assert_eq!(parsed.program, vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(parsed.instruction_pointer, 0);
    }

    #[test]
    fn test_example_1() {
        let mut parsed = Computer::try_from(EXAMPLE).unwrap();
        let output = run_program(&mut parsed).unwrap();
        assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_example_2() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            instruction_pointer: 0,
            program: vec![2, 6],
        };
        let _output = run_program(&mut computer).unwrap();
        assert_eq!(computer.register_b, 1);
    }

    #[test]
    fn test_example_3() {
        let mut computer = Computer {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let output = run_program(&mut computer).unwrap();
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn test_example_4() {
        let mut computer = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let output = run_program(&mut computer).unwrap();
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.register_a, 0);
    }

    #[test]
    fn test_example_5() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![1, 7],
        };
        let _output = run_program(&mut computer).unwrap();
        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn test_example_6() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            instruction_pointer: 0,
            program: vec![4, 0],
        };
        let _output = run_program(&mut computer).unwrap();
        assert_eq!(computer.register_b, 44354);
    }
}
