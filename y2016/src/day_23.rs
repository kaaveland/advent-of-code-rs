use crate::day_12::assembunny::{Atom, Register, Registers};
use crate::day_23::parse::{Instruction, Op};

mod parse {
    use crate::day_12::assembunny::{parse_atom, parse_register, Atom, Register};
    use anyhow::anyhow;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::char;
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::preceded;
    use nom::IResult;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Op {
        Cpy(Atom, Register),
        Inc(Register),
        Dec(Register),
        Jnz(Atom, Atom),
        Tgl(Register),
        Mul(Register, Register, Register),
        Noop,
    }
    fn parse_inc(s: &str) -> IResult<&str, Op> {
        map(preceded(tag("inc "), parse_register), Op::Inc)(s)
    }
    fn parse_dec(s: &str) -> IResult<&str, Op> {
        map(preceded(tag("dec "), parse_register), Op::Dec)(s)
    }
    fn parse_cpy(s: &str) -> IResult<&str, Op> {
        let (s, _) = tag("cpy ")(s)?;
        let (s, atom) = parse_atom(s)?;
        let (s, reg) = preceded(char(' '), parse_register)(s)?;
        Ok((s, Op::Cpy(atom, reg)))
    }
    fn parse_jnz(s: &str) -> IResult<&str, Op> {
        let (s, _) = tag("jnz ")(s)?;
        let (s, atom) = parse_atom(s)?;
        let (s, jump_dist) = preceded(char(' '), parse_atom)(s)?;
        Ok((s, Op::Jnz(atom, jump_dist)))
    }
    fn parse_tgl(s: &str) -> IResult<&str, Op> {
        let (s, _) = tag("tgl ")(s)?;
        let (s, i) = parse_register(s)?;
        Ok((s, Op::Tgl(i)))
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Instruction {
        pub op: Op,
        pub toggles: i32,
    }
    pub fn parse(s: &str) -> IResult<&str, Vec<Op>> {
        separated_list1(
            char('\n'),
            alt((parse_inc, parse_dec, parse_cpy, parse_jnz, parse_tgl)),
        )(s)
    }
    pub fn parse_prog(s: &str) -> anyhow::Result<Vec<Instruction>> {
        let (_, ops) = parse(s).map_err(|err| anyhow!("{err}"))?;
        Ok(ops
            .into_iter()
            .map(|op| Instruction { op, toggles: 0 })
            .collect())
    }
}

pub struct AssembunnyMachine {
    registers: Registers,
    program: Vec<Instruction>,
}

impl AssembunnyMachine {
    pub fn next(&mut self) {
        let ip = self.registers.ip;
        assert!(ip >= 0 && (ip as usize) < self.program.len());

        let exec = self.program[ip as usize];
        let toggled = exec.toggles % 2 != 0;
        let op = exec.op;
        let mut jmp = false;

        match (toggled, op) {
            (_, Op::Noop) => {}
            (_, Op::Mul(lhs, rhs, dst)) => {
                self.registers
                    .reg_set(dst, self.registers.val_of(lhs) * self.registers.val_of(rhs));
            }
            // inc becomes dec, and all other one-argument instructions become inc
            (false, Op::Inc(reg)) | (true, Op::Dec(reg)) | (true, Op::Tgl(reg)) => {
                self.registers.reg_set(reg, self.registers.val_of(reg) + 1);
            }
            // inc becomes dec, and all other one-argument instructions become inc
            (false, Op::Dec(reg)) | (true, Op::Inc(reg)) => {
                self.registers.reg_set(reg, self.registers.val_of(reg) - 1);
            }
            (false, Op::Tgl(offset)) => {
                let offset = self.registers.val_of(offset);
                let target = ip + offset;
                if target >= 0 && (target as usize) < self.program.len() {
                    self.program[target as usize].toggles += 1;
                }
            }
            (false, Op::Cpy(x, dst)) => {
                let x = self.registers.atom_val(&x);
                self.registers.reg_set(dst, x);
            }
            // becomes cpy
            (true, Op::Jnz(x, rhs)) => {
                // if rhs is lit, isn't valid
                match rhs {
                    Atom::Lit(_) => {}
                    Atom::Reg(dst) => {
                        let x = self.registers.atom_val(&x);
                        self.registers.reg_set(dst, x);
                    }
                }
            }
            (false, Op::Jnz(lhs_atom, rhs)) => {
                let lhs = self.registers.atom_val(&lhs_atom);
                let rhs = self.registers.atom_val(&rhs);
                if lhs != 0 {
                    self.registers.ip += rhs;
                    jmp = true;
                }
            }
            // becomes jnz
            (true, Op::Cpy(lhs, rhs)) => {
                let lhs = self.registers.atom_val(&lhs);
                let rhs = self.registers.val_of(rhs);
                if lhs != 0 {
                    self.registers.ip += rhs;
                    jmp = true;
                }
            }
        }

        if !jmp {
            self.registers.ip += 1;
        }
    }
}

fn execute(prog: &str, registers: Registers) -> anyhow::Result<Registers> {
    let mut prog = parse::parse_prog(prog.trim())?;
    // hot spot is:
    // ip: 4 cpy b c # move reg b to reg c, 344058144 executions
    // ip: 5 inc a # increment reg a (counter), 823059732 executions
    // ip: 6 dec c # decrement reg c (loop var), 823059732 executions
    // ip: 7 jnz c -2 # jump when c != c to inc a - this loop adds c to a (which is in b), 823059732 executions
    // ip: 8 dec d # decrement reg d (loop var), 344058144 executions
    // ip: 9 jnz d -5 # when d != 0 restart outer loop, 344058144 executions
    // for i in 0..d; for j in 0..b; a++;
    // a += b * d; c = 0; d = 0 # loop vars get cleared by the dec loops
    // replace it with multiply b with d into a, then noops
    prog[4].op = Op::Mul(Register::B, Register::D, Register::A);
    prog[5].op = Op::Cpy(Atom::Lit(0), Register::D);
    prog[6].op = Op::Cpy(Atom::Lit(0), Register::C);
    // Don't adjust program size, could change jump address offsets
    prog[7].op = Op::Noop;
    prog[8].op = Op::Noop;
    prog[9].op = Op::Noop;
    let len = prog.len();
    let mut machine = AssembunnyMachine {
        registers,
        program: prog,
    };
    while (machine.registers.ip as usize) < len {
        machine.next();
    }
    Ok(machine.registers)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let r = execute(
        s,
        Registers {
            a: 7,
            ..Default::default()
        },
    )?;
    Ok(format!("{}", r.a))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let r = execute(
        s,
        Registers {
            a: 12,
            ..Default::default()
        },
    )?;
    Ok(format!("{}", r.a))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a
";
    #[test]
    fn test_ex_1() {
        let r = execute(EX, Default::default()).unwrap();
        assert_eq!(r.a, 3);
    }
}
