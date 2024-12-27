use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;
use Instruction::*;

pub type Registers<const N: usize> = [usize; N];

pub(crate) fn exec<const N: usize>(
    mut reg: Registers<N>,
    command: &Command,
) -> Option<Registers<N>> {
    let val_a = command.reg_a;
    let reg_a = reg.get(val_a).copied();
    let val_b = command.reg_b;
    let reg_b = reg.get(val_b).copied();

    let val_c = match command.instruction {
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
    reg[command.reg_c] = val_c;
    Some(reg)
}

pub(crate) fn exec_with_ipreg<const N: usize>(
    command: &Command,
    ip: &mut usize,
    ip_reg: usize,
    mut reg: Registers<N>,
) -> Option<Registers<N>> {
    let ip_reg_val = reg[ip_reg];
    reg[ip_reg] = *ip;
    reg = exec(reg, command)?;
    *ip = reg[ip_reg];
    reg[ip_reg] = ip_reg_val;
    Some(reg)
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Instruction {
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

impl TryFrom<usize> for Instruction {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if let Some(ins) = ALL.get(value) {
            Ok(*ins)
        } else {
            Err(format!("No such instruction: {value}"))
        }
    }
}

pub const ALL: [Instruction; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Command {
    pub(crate) instruction: Instruction,
    pub(crate) reg_a: usize,
    pub(crate) reg_b: usize,
    pub(crate) reg_c: usize,
}

pub fn parse_instruction<'a, 'b>(
    ins_tag: &'a str,
    variant: Instruction,
) -> impl FnMut(&'b str) -> IResult<&'b str, Instruction> + use<'b, 'a> {
    map(tag(ins_tag), move |_| variant)
}

pub fn parse_reg(s: &str) -> IResult<&str, usize> {
    preceded(tag(" "), map_res(digit1, |s: &str| s.parse::<usize>()))(s)
}

pub fn parse_command(s: &str) -> IResult<&str, Command> {
    let (s, instruction) = alt((
        parse_instruction("addr", Addr),
        parse_instruction("addi", Addi),
        parse_instruction("mulr", Mulr),
        parse_instruction("muli", Muli),
        parse_instruction("banr", Banr),
        parse_instruction("bani", Bani),
        parse_instruction("borr", Borr),
        parse_instruction("bori", Bori),
        parse_instruction("setr", Setr),
        parse_instruction("seti", Seti),
        parse_instruction("gtir", Gtir),
        parse_instruction("gtri", Gtri),
        parse_instruction("gtrr", Gtrr),
        parse_instruction("eqir", Eqir),
        parse_instruction("eqri", Eqri),
        parse_instruction("eqrr", Eqrr),
    ))(s)?;
    let (s, (reg_a, reg_b, reg_c)) = tuple((parse_reg, parse_reg, parse_reg))(s)?;
    Ok((
        s,
        Command {
            instruction,
            reg_a,
            reg_b,
            reg_c,
        },
    ))
}

fn parse_ip_tag(s: &str) -> IResult<&str, usize> {
    preceded(tag("#ip "), map_res(digit1, |n: &str| n.parse::<usize>()))(s)
}

pub fn parse_elflang_asm(s: &str) -> anyhow::Result<(usize, Vec<Command>)> {
    let (_, out) = separated_pair(
        parse_ip_tag,
        tag("\n"),
        separated_list1(tag("\n"), parse_command),
    )(s)
    .map_err(|err| anyhow!("{err}"))?;
    Ok(out)
}

pub fn posint(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse())(s)
}

pub fn sep(by: &str) -> impl FnMut(&str) -> IResult<&str, usize> + '_ {
    move |s: &str| terminated(posint, tag(by))(s)
}

pub fn parse_elflang_bin_cmd(s: &str) -> IResult<&str, Command> {
    let (s, (a, b, c, d)) = tuple((
        map_res(sep(" "), |n: usize| {
            Ok::<Instruction, String>(n.try_into()?)
        }),
        sep(" "),
        sep(" "),
        posint,
    ))(s)?;
    Ok((
        s,
        Command {
            instruction: a,
            reg_a: b,
            reg_b: c,
            reg_c: d,
        },
    ))
}

pub fn parse_elflang_bin(s: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(char('\n'), parse_elflang_bin_cmd)(s)
}

type ExecTrace<const N: usize> = (usize, usize, Command, Registers<N>, Registers<N>);

/// Keeping this, because it's how I figured out what the loop does
/// and because we can use it to identify which register that is set up
/// for the loop counter / input
pub fn trace_until_loop<const N: usize>(
    ip_reg: usize,
    program: &[Command],
    initial_reg0: usize,
    loop_count: usize,
) -> anyhow::Result<Vec<ExecTrace<N>>> {
    let mut ip = 0;
    let mut reg: Registers<N> = [0; N];
    let mut trace = Vec::new();
    let mut loops = 0;
    reg[0] = initial_reg0;

    while (0..program.len()).contains(&ip) {
        let before = ip;
        let cmd = program[ip];
        let initial = reg;
        reg = exec_with_ipreg(&cmd, &mut ip, ip_reg, reg)
            .with_context(|| format!("{cmd:?} {reg:?} {ip:?} {ip_reg:?}"))?;
        ip += 1;
        trace.push((before, ip, cmd, initial, reg));
        if ip < before && loops >= loop_count {
            break;
        } else if ip < before {
            loops += 1;
        }
    }

    Ok(trace)
}
