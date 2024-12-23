use anyhow::{anyhow, Context};
use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use Choice::*;
use Instruction::*;
type Register = [usize; 4];
type Program = [usize; 4];

#[repr(usize)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    Addr = 0,
    Addi = 1,
    Mulr = 2,
    Muli = 3,
    Banr = 4,
    Bani = 5,
    Borr = 6,
    Bori = 7,
    Setr = 8,
    Seti = 9,
    Gtir = 10,
    Gtri = 11,
    Gtrr = 12,
    Eqir = 13,
    Eqri = 14,
    Eqrr = 15,
}

const ALL: [Instruction; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

fn exec(mut register: Register, program: Program, ins: Instruction) -> Option<Register> {
    let [_, a, b, c] = program;
    let val_b = b;
    let val_a = a;
    let reg_b = register.get(b).copied();
    let reg_a = register.get(a).copied();
    let reg_c = c;

    let val_c = match ins {
        Addr => reg_a? + reg_b?,
        Addi => reg_a? + val_b,
        Mulr => reg_a? * reg_b?,
        Muli => reg_a? * val_b,
        Banr => reg_a? & reg_b?,
        Bani => reg_a? & val_b,
        Borr => reg_a? | reg_b?,
        Bori => reg_a? | val_b,
        Setr => reg_a?,
        Seti => val_a,
        Gtir => usize::from(val_a > reg_b?),
        Gtri => usize::from(reg_a? > val_b),
        Gtrr => usize::from(reg_a? > reg_b?),
        Eqir => usize::from(val_a == reg_b?),
        Eqri => usize::from(reg_a? == val_b),
        Eqrr => usize::from(reg_a? == reg_b?),
    };
    register[reg_c] = val_c;

    Some(register)
}

fn compatible_instructions(before: Register, program: Program, after: Register) -> [bool; 16] {
    let mut out = [false; 16];
    for i in 0..16 {
        out[i] = exec(before, program, ALL[i]) == Some(after);
    }
    out
}

fn posint(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse())(s)
}

fn sep(by: &str) -> impl FnMut(&str) -> IResult<&str, usize> + '_ {
    move |s: &str| terminated(posint, tag(by))(s)
}

fn parse_register(s: &str) -> IResult<&str, Register> {
    let (s, (a, b, c, d)) = delimited(
        char('['),
        tuple((sep(", "), sep(", "), sep(", "), posint)),
        char(']'),
    )(s)?;
    Ok((s, [a, b, c, d]))
}

fn parse_program(s: &str) -> IResult<&str, Program> {
    let (s, (a, b, c, d)) = tuple((sep(" "), sep(" "), sep(" "), posint))(s)?;
    Ok((s, [a, b, c, d]))
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Example {
    before: Register,
    program: Program,
    after: Register,
}

fn parse_examples(s: &str) -> IResult<&str, Vec<Example>> {
    separated_list1(tag("\n\n"), parse_example)(s)
}

fn parse_programs(s: &str) -> IResult<&str, Vec<Program>> {
    separated_list1(char('\n'), parse_program)(s)
}

fn parse(s: &str) -> anyhow::Result<(Vec<Example>, Vec<Program>)> {
    fn p(s: &str) -> IResult<&str, (Vec<Example>, Vec<Program>)> {
        separated_pair(parse_examples, many1(char('\n')), parse_programs)(s)
    }
    p(s).map_err(|e| anyhow!("{e}")).map(|(_, r)| r)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let (examples, _) = parse(s)?;
    let n = examples
        .into_iter()
        .filter(|e| {
            compatible_instructions(e.before, e.program, e.after)
                .into_iter()
                .filter(|i| *i)
                .count()
                >= 3
        })
        .count();
    Ok(format!("{n}"))
}

fn parse_example(s: &str) -> IResult<&str, Example> {
    let (s, before) = preceded(tag("Before: "), parse_register)(s)?;
    let (s, program) = preceded(char('\n'), parse_program)(s)?;
    let (s, after) = preceded(tag("\nAfter:  "), parse_register)(s)?;
    Ok((
        s,
        Example {
            before,
            program,
            after,
        },
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Choice {
    Decided(Instruction),
    Options(FxHashSet<Instruction>),
}

fn initial() -> [Choice; 16] {
    let mut all = vec![];
    for _ in 0..16 {
        all.push(Options(FxHashSet::from_iter(ALL.into_iter())));
    }

    all.try_into().unwrap()
}

fn choose(choices: &mut [Choice; 16], place: usize, instruction: Instruction) {
    if let Some(choice) = choices.get_mut(place) {
        if let Options(set) = choice {
            assert!(set.contains(&instruction));
            *choice = Decided(instruction);
        } else if let Decided(old) = choice {
            assert_eq!(*old, instruction);
        }
    } else {
        panic!("Out of bounds: {place}")
    }
    for other_place in 0..16 {
        if other_place != place {
            eliminate(choices, other_place, instruction);
        }
    }
}

fn eliminate(choices: &mut [Choice; 16], place: usize, instruction: Instruction) {
    if let Some(choice) = choices.get_mut(place) {
        if let Options(set) = choice {
            set.remove(&instruction);
            if set.len() == 1 {
                let chosen = set.iter().copied().next().unwrap();
                choose(choices, place, chosen);
            }
        } else if let Decided(old) = choice {
            assert_ne!(*old, instruction);
        }
    }
}

fn done(choices: &[Choice; 16]) -> bool {
    choices.iter().all(|choice| matches!(choice, Decided(_)))
}

fn identify_opcodes(examples: &[Example]) -> [Instruction; 16] {
    let mut out = [Addr; 16];
    let mut choices = initial();
    for ex in examples.into_iter() {
        for (place, compat) in compatible_instructions(ex.before, ex.program, ex.after)
            .iter()
            .enumerate()
        {
            if !compat {
                eliminate(&mut choices, ex.program[0], ALL[place]);
            }
        }
        if done(&choices) {
            break;
        }
    }
    for (o, choice) in out.iter_mut().zip(choices) {
        match choice {
            Decided(i) => {
                *o = i;
            }
            _ => panic!("Failed to solve"),
        }
    }
    out
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let (examples, programs) = parse(s)?;
    let mapping = identify_opcodes(&examples);
    let mut registers = [0; 4];
    for prog in programs {
        registers = exec(registers, prog, mapping[prog[0]]).context("Unable to execute program")?;
    }
    Ok(format!("{}", registers[0]))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_opcode() {
        let mut possible = [false; 16];
        possible[Mulr as usize] = true;
        possible[Addi as usize] = true;
        possible[Seti as usize] = true;
        assert_eq!(
            compatible_instructions([3, 2, 1, 1], [9, 2, 1, 2], [3, 2, 2, 1]),
            possible
        );
    }
}
