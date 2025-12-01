use anyhow::{anyhow, Result};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
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
    wire: &'a str,
    reg: char,
    index: usize,
    value: bool,
}

fn parse_provided_value(s: &str) -> IResult<&str, ProvidedValue<'_>> {
    let (s, wire) = recognize(many1(none_of(":\n")))(s)?;
    let reg = wire.chars().next().unwrap();
    let index = if let Ok(dig) = wire[1..].parse::<usize>() {
        dig
    } else {
        panic!("{wire}")
    };
    let (s, _skip) = tag(": ")(s)?;
    let (s, value) = alt((map(char('1'), |_| true), map(char('0'), |_| false)))(s)?;
    Ok((
        s,
        ProvidedValue {
            wire,
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
    wire: &'a str,
    z_index: Option<usize>,
    op: BinaryOp,
    lhs: &'a str,
    rhs: &'a str,
}

fn parse_calculated_value(s: &str) -> IResult<&str, CalculatedValue<'_>> {
    let (s, lhs) = terminated(recognize(many1(none_of(" "))), tag(" "))(s)?;
    let (s, op) = terminated(parse_binop, tag(" "))(s)?;
    let (s, rhs) = terminated(recognize(many1(none_of(" "))), tag(" -> "))(s)?;
    let (s, wire) = recognize(many1(none_of("\n")))(s)?;
    let z_index = wire[1..].parse::<usize>().ok();
    Ok((
        s,
        CalculatedValue {
            wire,
            z_index,
            op,
            lhs,
            rhs,
        },
    ))
}

fn parse(s: &str) -> Result<(Vec<ProvidedValue<'_>>, Vec<CalculatedValue<'_>>)> {
    let (_, r) = separated_pair(
        separated_list1(tag("\n"), parse_provided_value),
        tag("\n\n"),
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
        deps.entry(c.lhs).or_default().insert(c.wire);
        deps.entry(c.rhs).or_default().insert(c.wire);
    }
    deps
}

fn calculate<'a>(
    provided_value: &'a [ProvidedValue],
    calculated_values: &'a [CalculatedValue],
) -> FxHashMap<usize, bool> {
    let mut deps = dependencies(calculated_values);
    let by_name: FxHashMap<_, _> = calculated_values.iter().map(|c| (c.wire, c)).collect();
    let mut known = FxHashMap::default();
    let mut work: VecDeque<_> = provided_value.iter().map(|p| (p.wire, p.value)).collect();
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
                    work.push_back((calc.wire, result));
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

pub fn part_2(s: &str) -> Result<String> {
    let (_, calculated) = parse(s)?;
    // Half adder on LSB
    // takes x and y and outputs sum and carry
    // made with sum = x ^ y and carry x & y
    // Full adder:
    // x, y and carry-in (chained, initially from half adder)
    // sum = (x ^ y) ^ (carry-in) -> this sets bit in z, so z should only come from XOR (other than MSB)
    // carry-out: = ((x ^ y) & carry-in) | (x & y)
    // Other than half adder, | is the only consumer of & outputs
    // Swapping carry-in for sum would look like ^ going to |
    let mut sus = vec![];

    let in_out = |s: &str| s.starts_with("x") || s.starts_with("y") || s.starts_with("z");

    for c in &calculated {
        // z should be all XOR except the highest one
        if c.z_index.is_some() && c.z_index != Some(45) && c.op != Xor {
            sus.push(c.wire);
        }
        // XOR should be writing to z or reading from x, y (probably swapped with above)
        if c.op == Xor && !in_out(c.lhs) && !in_out(c.rhs) && !in_out(c.wire) {
            sus.push(c.wire);
        }
        // AND, other than the very first carry, should go to OR
        if c.op == And
            && c.lhs != "x00"
            && c.rhs != "y00"
            && calculated
                .iter()
                .any(|child| child.op != Or && (child.lhs == c.wire || child.rhs == c.wire))
        {
            sus.push(c.wire);
        }
        // XOR should never go directly to OR (probably swapped with above)
        if c.op == Xor
            && calculated
                .iter()
                .any(|child| child.op == Or && (child.lhs == c.wire || child.rhs == c.wire))
        {
            sus.push(c.wire);
        }
    }

    sus.sort();
    Ok(sus.into_iter().unique().join(","))
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
                wire: "x09",
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
                wire: "z41",
                z_index: Some(41),
                op: Xor,
                lhs: "twb",
                rhs: "jgm",
            }
        );

        assert_eq!(
            parse_calculated_value("twb OR jgm -> zjx").unwrap().1,
            CalculatedValue {
                wire: "zjx",
                z_index: None,
                op: Or,
                lhs: "twb",
                rhs: "jgm",
            }
        );
    }
}
