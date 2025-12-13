use crate::day_12::assembunny::{Op, Registers};
use anyhow::anyhow;

mod assembunny {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1};
    use nom::combinator::{map, map_res};
    use nom::multi::separated_list1;
    use nom::sequence::preceded;
    use nom::IResult;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    pub struct Registers {
        pub a: i32,
        pub b: i32,
        pub c: i32,
        pub d: i32,
        pub ip: i32,
    }

    impl Registers {
        fn val_of(&self, reg: Register) -> i32 {
            match reg {
                Register::A => self.a,
                Register::B => self.b,
                Register::C => self.c,
                Register::D => self.d,
            }
        }
        fn atom_val(&self, atom: &Atom) -> i32 {
            match atom {
                Atom::Lit(x) => *x,
                Atom::Reg(src) => self.val_of(*src),
            }
        }
        fn reg_set(&mut self, reg: Register, to: i32) {
            match reg {
                Register::A => {
                    self.a = to;
                }
                Register::B => {
                    self.b = to;
                }
                Register::C => {
                    self.c = to;
                }
                Register::D => {
                    self.d = to;
                }
            }
        }
        pub fn next(&self, op: Op) -> Self {
            let mut reg = *self;
            let mut jmp = false;
            match op {
                Op::Cpy(Atom::Lit(x), dst) => {
                    reg.reg_set(dst, x);
                }
                Op::Cpy(Atom::Reg(src), dst) => {
                    reg.reg_set(dst, reg.val_of(src));
                }
                Op::Inc(dst) => {
                    reg.reg_set(dst, reg.val_of(dst) + 1);
                }
                Op::Dec(dst) => {
                    reg.reg_set(dst, reg.val_of(dst) - 1);
                }
                Op::Jnz(lhs, rhs) => {
                    let lhs = self.atom_val(&lhs);
                    let rhs = self.atom_val(&rhs);
                    if lhs != 0 {
                        reg.ip += rhs;
                        jmp = true;
                    }
                }
            }
            if !jmp {
                reg.ip += 1;
            }
            reg
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Register {
        A,
        B,
        C,
        D,
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Atom {
        Lit(i32),
        Reg(Register),
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Op {
        Cpy(Atom, Register),
        Inc(Register),
        Dec(Register),
        Jnz(Atom, Atom),
    }
    fn parse_register(s: &str) -> IResult<&str, Register> {
        alt((
            map(char('a'), |_| Register::A),
            map(char('b'), |_| Register::B),
            map(char('c'), |_| Register::C),
            map(char('d'), |_| Register::D),
        ))(s)
    }
    fn posint(s: &str) -> IResult<&str, i32> {
        map_res(digit1, |n: &str| n.parse::<i32>())(s)
    }
    fn parse_int(s: &str) -> IResult<&str, Atom> {
        let p = alt((preceded(char('-'), map(posint, |n: i32| -n)), posint));
        map(p, Atom::Lit)(s)
    }
    fn parse_atom(s: &str) -> IResult<&str, Atom> {
        alt((map(parse_register, Atom::Reg), parse_int))(s)
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
    pub fn parse(s: &str) -> IResult<&str, Vec<Op>> {
        separated_list1(
            char('\n'),
            alt((parse_inc, parse_dec, parse_cpy, parse_jnz)),
        )(s)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_parse_reg() {
            assert_eq!(Register::A, parse_register("a").unwrap().1);
            assert!(parse_register("9").is_err());
        }
        #[test]
        fn test_parse_int() {
            assert_eq!(Atom::Lit(11), parse_int("11").unwrap().1);
            assert_eq!(Atom::Lit(-11), parse_int("-11").unwrap().1);
        }
        #[test]
        fn test_parse_atom() {
            assert_eq!(Atom::Lit(11), parse_atom("11").unwrap().1);
            assert_eq!(Atom::Reg(Register::D), parse_atom("d").unwrap().1);
        }
        #[test]
        fn test_parse_inc() {
            assert_eq!(Op::Inc(Register::D), parse_inc("inc d").unwrap().1);
        }
        #[test]
        fn test_parse_dec() {
            assert_eq!(Op::Dec(Register::C), parse_dec("dec c").unwrap().1);
        }
        #[test]
        fn test_parses_cpy() {
            assert_eq!(
                Op::Cpy(Atom::Lit(13), Register::C),
                parse_cpy("cpy 13 c").unwrap().1
            );
        }
    }
}

fn exec(prog: &[Op], set_c: i32) -> Registers {
    let mut reg = Registers {
        c: set_c,
        ..Registers::default()
    };
    while reg.ip < prog.len() as i32 {
        let op = prog[reg.ip as usize];
        reg = reg.next(op);
    }
    reg
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let (_, prog) = assembunny::parse(s).map_err(|err| anyhow!("{err}"))?;
    let reg = exec(&prog, 0);
    Ok(format!("{}", reg.a))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let (_, prog) = assembunny::parse(s).map_err(|err| anyhow!("{err}"))?;
    let reg = exec(&prog, 1);
    Ok(format!("{}", reg.a))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn test_parse() {
        let (_, prog) = assembunny::parse(EX).unwrap();
        assert_eq!(prog.len(), 6);
    }

    #[test]
    fn test_prog() {
        let (_, prog) = assembunny::parse(EX).unwrap();
        let reg = exec(&prog, 0);
        assert_eq!(reg.a, 42);
    }
}
