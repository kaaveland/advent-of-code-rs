use anyhow::{anyhow, Context, Result};
use fxhash::FxHashSet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use Instruction::*;

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

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Command {
    instruction: Instruction,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
}

type Register = [usize; 6];

fn parse_instruction<'a, 'b>(
    ins_tag: &'a str,
    variant: Instruction,
) -> impl FnMut(&'b str) -> IResult<&'b str, Instruction> + use<'b, 'a> {
    map(tag(ins_tag), move |_| variant)
}

fn parse_reg(s: &str) -> IResult<&str, usize> {
    preceded(tag(" "), map_res(digit1, |s: &str| s.parse::<usize>()))(s)
}

fn parse_command(s: &str) -> IResult<&str, Command> {
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

fn parse(s: &str) -> Result<(usize, Vec<Command>)> {
    let (_, out) = separated_pair(
        parse_ip_tag,
        tag("\n"),
        separated_list1(tag("\n"), parse_command),
    )(s)
    .map_err(|err| anyhow!("{err}"))?;
    Ok(out)
}

fn exec(command: &Command, ip: &mut usize, ip_reg: usize, reg: &mut Register) -> Option<()> {
    let ip_reg_val = reg[ip_reg];
    reg[ip_reg] = *ip;
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
    *ip = reg[ip_reg];
    reg[ip_reg] = ip_reg_val;

    Some(())
}

/// Keeping this, because it's how I figured out what the loop does
/// and because we can use it to identify which register that is set up
/// for the loop counter / input
fn trace_until_loop(
    ip_reg: usize,
    program: &[Command],
    initial_reg0: usize,
    loop_count: usize,
) -> Result<Vec<(usize, usize, Command, Register, Register)>> {
    let mut ip = 0;
    let mut reg: Register = [0; 6];
    let mut trace = Vec::new();
    let mut loops = 0;
    reg[0] = initial_reg0;

    while (0..program.len()).contains(&ip) {
        let before = ip;
        let cmd = program[ip];
        let initial = reg;
        exec(&cmd, &mut ip, ip_reg, &mut reg)
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

fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut divisors = vec![];
    while n % 2 == 0 {
        divisors.push(2);
        n /= 2;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            divisors.push(d);
            n /= d;
        } else {
            d += 2;
        }
    }
    divisors.push(n); // prime
    divisors
}

fn sum_product_combinations(divisors: &[usize]) -> usize {
    let mut nats: FxHashSet<usize> = FxHashSet::from_iter([1]);
    for i in 1..(1 << divisors.len()) {
        let mut p = 1;
        for (j, d) in divisors.iter().copied().enumerate() {
            // check if jth bit in i is set:
            if i & (1 << j) != 0 {
                p *= d;
            }
        }
        nats.insert(p);
    }
    nats.into_iter().sum()
}

fn solve(s: &str, reg0: usize) -> Result<usize> {
    let (ip_reg, program) = parse(s)?;
    let prep = trace_until_loop(ip_reg, &program, reg0, 0)?;
    let inp = prep
        .into_iter()
        .filter_map(|(_ip, _new_ip, _cmd, _reg_in, reg_out)| reg_out.into_iter().max())
        .max()
        .context("No input found")?;
    let factors = prime_factors(inp);
    Ok(sum_product_combinations(&factors))
}

pub fn part_1(s: &str) -> Result<String> {
    let reg0 = solve(s, 0)?;
    Ok(reg0.to_string())
}

pub fn part_2(s: &str) -> Result<String> {
    let reg0 = solve(s, 1)?;
    Ok(reg0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ins() {
        let mut parser = parse_instruction("seti", Seti);
        assert!(matches!(parser("seti"), Ok(_)));
    }
    #[test]
    fn test_parse_cmd() {
        assert!(matches!(parse_command("seti 5 0 1\n"), Ok(_)));
    }
    #[test]
    fn test_parse_reg() {
        assert!(matches!(parse_reg(" 5"), Ok(_)));
    }

    #[test]
    fn test_factors() {
        assert_eq!(prime_factors(981), vec![3, 3, 109]);
        assert_eq!(sum_product_combinations(&[3, 3, 109]), 1430);
    }
}
