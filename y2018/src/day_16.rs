use crate::elflang;
use crate::elflang::Instruction::*;
use crate::elflang::{
    parse_elflang_bin, parse_elflang_bin_cmd, posint, sep, Command, Instruction, Registers, ALL,
};
use anyhow::{anyhow, Context};
use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;
use Choice::*;

fn compatible_instructions(
    before: Registers<4>,
    mut program: Command,
    after: Registers<4>,
) -> [bool; 16] {
    let mut out = [false; 16];
    for i in 0..16 {
        program.instruction = ALL[i];
        out[i] = elflang::exec(before, &program) == Some(after);
    }
    out
}

fn parse_register(s: &str) -> IResult<&str, Registers<4>> {
    let (s, (a, b, c, d)) = delimited(
        char('['),
        tuple((sep(", "), sep(", "), sep(", "), posint)),
        char(']'),
    )(s)?;
    Ok((s, [a, b, c, d]))
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Example {
    before: Registers<4>,
    program: Command,
    after: Registers<4>,
}

fn parse_examples(s: &str) -> IResult<&str, Vec<Example>> {
    separated_list1(tag("\n\n"), parse_example)(s)
}

fn parse(s: &str) -> anyhow::Result<(Vec<Example>, Vec<Command>)> {
    fn p(s: &str) -> IResult<&str, (Vec<Example>, Vec<Command>)> {
        separated_pair(parse_examples, many1(char('\n')), parse_elflang_bin)(s)
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
    let (s, program) = preceded(char('\n'), parse_elflang_bin_cmd)(s)?;
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

fn initial<const N: usize>() -> [Choice; N] {
    let mut all = vec![];
    for _ in 0..N {
        all.push(Options(FxHashSet::from_iter(ALL.into_iter())));
    }

    all.try_into().unwrap()
}

fn choose<const N: usize>(choices: &mut [Choice; N], place: usize, instruction: Instruction) {
    if let Some(choice) = choices.get_mut(place) {
        if let Options(_) = choice {
            *choice = Decided(instruction);
            for other_place in 0..N {
                if other_place != place {
                    eliminate(choices, other_place, instruction);
                }
            }
        }
    }
}

fn eliminate<const N: usize>(choices: &mut [Choice; N], place: usize, instruction: Instruction) {
    if let Some(Options(set)) = choices.get_mut(place) {
        set.remove(&instruction);
        if set.len() == 1 {
            let chosen = set.iter().copied().next().unwrap();
            choose(choices, place, chosen);
        }
    }
}

fn done<const N: usize>(choices: &[Choice; N]) -> bool {
    choices.iter().all(|choice| matches!(choice, Decided(_)))
}

fn identify_opcodes<const N: usize>(examples: &[Example]) -> [Instruction; N] {
    let mut out = [Addr; N];
    let mut choices = initial::<N>();
    for ex in examples.iter() {
        for (place, compat) in compatible_instructions(ex.before, ex.program, ex.after)
            .iter()
            .enumerate()
        {
            if !compat {
                eliminate(&mut choices, ex.program.instruction as usize, ALL[place]);
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
    let mapping = identify_opcodes::<16>(&examples);
    let mut registers = [0; 4];
    for mut prog in programs {
        prog.instruction = mapping[prog.instruction as usize];
        registers = elflang::exec(registers, &prog).context("Unable to execute program")?;
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
            compatible_instructions(
                [3, 2, 1, 1],
                Command {
                    instruction: Addi,
                    reg_a: 2,
                    reg_b: 1,
                    reg_c: 2
                },
                [3, 2, 2, 1]
            ),
            possible
        );
    }
}
