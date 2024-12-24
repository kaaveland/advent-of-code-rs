use anyhow::{anyhow, Result};
use fxhash::{FxHashMap, FxHashSet};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, none_of};
use nom::combinator::{map, recognize};
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::collections::VecDeque;
use BinaryOp::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
struct ProvidedValue<'a> {
    reg_name: &'a str,
    reg: char,
    index: usize,
    value: bool,
}

fn parse_provided_value(s: &str) -> IResult<&str, ProvidedValue> {
    let (s, reg_name) = recognize(many1(none_of(":\n")))(s)?;
    let reg = reg_name.chars().next().unwrap();
    let index = if let Ok(dig) = reg_name[1..].parse::<usize>() {
        dig
    } else {
        panic!("{reg_name}")
    };
    let (s, _skip) = tag(": ")(s)?;
    let (s, value) = alt((map(char('1'), |_| true), map(char('0'), |_| false)))(s)?;
    Ok((
        s,
        ProvidedValue {
            reg_name,
            reg,
            index,
            value,
        },
    ))
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
enum BinaryOp {
    Xor,
    And,
    Or,
}

fn parse_binop(s: &str) -> IResult<&str, BinaryOp> {
    alt((
        map(tag("AND"), |_| And),
        map(tag("XOR"), |_| Xor),
        map(tag("OR"), |_| Or),
    ))(s)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
struct CalculatedValue<'a> {
    dest_name: &'a str,
    z_index: Option<usize>,
    op: BinaryOp,
    lhs: &'a str,
    rhs: &'a str,
}

fn parse_calculated_value(s: &str) -> IResult<&str, CalculatedValue> {
    let (s, lhs) = terminated(recognize(many1(none_of(" "))), tag(" "))(s)?;
    let (s, op) = terminated(parse_binop, tag(" "))(s)?;
    let (s, rhs) = terminated(recognize(many1(none_of(" "))), tag(" -> "))(s)?;
    let (s, dest_name) = recognize(many1(none_of("\n")))(s)?;
    let z_index = if let Ok(ix) = dest_name[1..].parse::<usize>() {
        Some(ix)
    } else {
        None
    };
    Ok((
        s,
        CalculatedValue {
            dest_name,
            z_index,
            op,
            lhs,
            rhs,
        },
    ))
}

fn parse(s: &str) -> Result<(Vec<ProvidedValue>, Vec<CalculatedValue>)> {
    let (_, r) = separated_pair(
        separated_list1(tag("\n"), parse_provided_value),
        tag("\n"),
        separated_list1(tag("\n"), parse_calculated_value),
    )(s)
    .map_err(|err| anyhow!("{err}"))?;
    Ok(r)
}

fn dependencies<'a>(
    calculated: &'a [CalculatedValue<'a>],
) -> FxHashMap<&'a str, FxHashSet<&'a str>> {
    let mut deps: FxHashMap<&str, FxHashSet<_>> = FxHashMap::default();
    for c in calculated {
        deps.entry(c.lhs).or_default().insert(c.dest_name);
        deps.entry(c.rhs).or_default().insert(c.dest_name);
    }
    deps
}

fn calculate<'a>(
    provided_value: &'a [ProvidedValue],
    calculated_values: &'a [CalculatedValue],
) -> FxHashMap<usize, bool> {
    let mut deps = dependencies(calculated_values);
    let by_name: FxHashMap<_, _> = calculated_values.iter().map(|c| (c.dest_name, c)).collect();
    let mut known = FxHashMap::default();
    let mut work: VecDeque<_> = provided_value
        .iter()
        .map(|p| (p.reg_name, p.value))
        .collect();
    let mut z = FxHashMap::default();

    while let Some((reg, value)) = work.pop_front() {
        known.insert(reg, value);
        if let Some((_, depends)) = deps.remove_entry(reg) {
            for calculation in depends {
                let calc = *by_name.get(calculation).unwrap();
                if let Some((lhs, rhs)) = known
                    .get(calc.lhs)
                    .and_then(|lhs| known.get(calc.rhs).map(|rhs| (*lhs, *rhs)))
                {
                    let result = match calc.op {
                        Or => lhs || rhs,
                        Xor => lhs ^ rhs,
                        And => lhs && rhs,
                    };
                    if let Some(ix) = calc.z_index {
                        z.insert(ix, result);
                    }
                    work.push_back((calc.dest_name, result));
                }
            }
        }
    }
    z
}

pub fn part_1(s: &str) -> Result<String> {
    let (provided, calculated) = parse(s)?;
    check_assumptions(&provided);
    let z = calculate(&provided, &calculated);
    let z: usize = z
        .into_iter()
        .map(|(ix, v)| if v { 1 << ix } else { 0 })
        .sum();

    Ok(z.to_string())
}

fn check_assumptions(provided: &[ProvidedValue]) {
    assert!(provided.iter().all(|p| p.reg == 'y' || p.reg == 'x'));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn test_calc() -> Result<()> {
        let (prov, calc) = parse(EX)?;
        let z = calculate(&prov, &calc);
        let z: usize = z
            .into_iter()
            .map(|(ix, v)| if v { 1 << ix } else { 0 })
            .sum();
        assert_eq!(z, 2024);
        Ok(())
    }

    #[test]
    fn binop() {
        assert_eq!(parse_binop("AND").unwrap().1, And);
    }

    #[test]
    fn provided_value() {
        assert_eq!(
            parse_provided_value("x09: 1").unwrap().1,
            ProvidedValue {
                reg_name: "x09",
                reg: 'x',
                index: 9,
                value: true
            }
        );
    }

    #[test]
    fn calculated_value() {
        assert_eq!(
            parse_calculated_value("twb XOR jgm -> z41").unwrap().1,
            CalculatedValue {
                dest_name: "z41",
                z_index: Some(41),
                op: Xor,
                lhs: "twb",
                rhs: "jgm",
            }
        );

        assert_eq!(
            parse_calculated_value("twb OR jgm -> zjx").unwrap().1,
            CalculatedValue {
                dest_name: "zjx",
                z_index: None,
                op: Or,
                lhs: "twb",
                rhs: "jgm",
            }
        );
    }
}
