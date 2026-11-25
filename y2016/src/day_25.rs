use crate::day_12::assembunny::Atom;
use crate::day_12::assembunny::Register::*;
use crate::day_23::parse::Op;
use anyhow::{anyhow, bail};

fn clock_offset(program: &[Op]) -> anyhow::Result<i32> {
    match program {
        [Op::Cpy(Atom::Reg(A), D), Op::Cpy(Atom::Lit(c), C), Op::Cpy(Atom::Lit(b), B), Op::Inc(D), Op::Dec(B), Op::Jnz(Atom::Reg(B), Atom::Lit(-2)), Op::Dec(C), Op::Jnz(Atom::Reg(C), Atom::Lit(-5)), ..] => {
            Ok(b * c)
        }
        _ => bail!(
            "unexpected program head: {:?}",
            &program[..program.len().min(8)]
        ),
    }
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let (_, program) =
        crate::day_23::parse::parse(input.trim_end()).map_err(|e| anyhow!("parse error: {e}"))?;
    let offset = clock_offset(&program)?;
    // Smallest n > offset whose binary form is 1010…10: 2, 10, 42, 170, …
    let mut n = 2;
    while n <= offset {
        n = 4 * n + 2;
    }
    Ok(format!("{}", n - offset))
}

pub fn part_2(_: &str) -> anyhow::Result<String> {
    Ok("Collect the stars and click the button (-:".to_string())
}
