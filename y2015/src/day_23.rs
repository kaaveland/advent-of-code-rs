use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Copy, Clone, Debug)]
enum Register {
    A,
    B,
}

#[derive(Copy, Clone, Debug)]
enum JumpArg {
    Forward(usize),
    Backward(usize),
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jump(JumpArg),
    Jie(Register, JumpArg),
    Jio(Register, JumpArg),
}

fn parse_reg(s: &str) -> IResult<&str, Register> {
    alt((
        map(tag("a"), |_| Register::A),
        map(tag("b"), |_| Register::B),
    ))(s)
}

fn parse_jumparg(s: &str) -> IResult<&str, JumpArg> {
    alt((
        map(
            map_res(preceded(tag("+"), digit1), |s: &str| s.parse()),
            JumpArg::Forward,
        ),
        map(
            map_res(preceded(tag("-"), digit1), |s: &str| s.parse()),
            JumpArg::Backward,
        ),
    ))(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("hlf "), parse_reg), Instruction::Hlf),
        map(preceded(tag("tpl "), parse_reg), Instruction::Tpl),
        map(preceded(tag("inc "), parse_reg), Instruction::Inc),
        map(preceded(tag("jmp "), parse_jumparg), Instruction::Jump),
        map(
            preceded(
                tag("jie "),
                separated_pair(parse_reg, tag(", "), parse_jumparg),
            ),
            |(reg, jmp)| Instruction::Jie(reg, jmp),
        ),
        map(
            preceded(
                tag("jio "),
                separated_pair(parse_reg, tag(", "), parse_jumparg),
            ),
            |(reg, jmp)| Instruction::Jio(reg, jmp),
        ),
    ))(s)
}

fn parse(s: &str) -> anyhow::Result<Vec<Instruction>> {
    separated_list1(tag("\n"), parse_instruction)(s)
        .map_err(|err| anyhow!("{err}"))
        .map(|(_, v)| v)
}

fn run(program: &[Instruction], initial_a: usize) -> (usize, usize) {
    use Instruction::*;
    use JumpArg::*;
    use Register::*;

    let mut ix = 0;
    let mut a = initial_a;
    let mut b = 0;

    loop {
        if ix >= program.len() {
            return (a, b);
        }
        let instruction = program[ix];
        //println!("{ix}/{} {a} {b}: {:?}", program.len(), &program[ix]);
        ix = match instruction {
            Jump(Forward(dx)) => ix + dx,
            Jump(Backward(dx)) => ix - dx,
            Jie(A, Forward(dx)) if a % 2 == 0 => ix + dx,
            Jie(A, Backward(dx)) if a % 2 == 0 => ix - dx,
            Jie(B, Forward(dx)) if b % 2 == 0 => ix + dx,
            Jie(B, Backward(dx)) if b % 2 == 0 => ix - dx,
            Jio(A, Forward(dx)) if a == 1 => ix + dx,
            Jio(A, Backward(dx)) if a == 1 => ix - dx,
            Jio(B, Forward(dx)) if b == 1 => ix + dx,
            Jio(B, Backward(dx)) if b == 1 => ix - dx,
            _ => ix + 1,
        };
        match instruction {
            Hlf(A) => a /= 2,
            Hlf(B) => b /= 2,
            Tpl(A) => a *= 3,
            Tpl(B) => b *= 3,
            Inc(A) => a += 1,
            Inc(B) => b += 1,
            _ => (),
        }
    }
}

fn eval(s: &str, initial_a: usize) -> anyhow::Result<(usize, usize)> {
    let prog = parse(s)?;
    Ok(run(&prog, initial_a))
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let (_, b) = eval(s, 0)?;
    Ok(b.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let (_, b) = eval(s, 1)?;
    Ok(b.to_string())
}
