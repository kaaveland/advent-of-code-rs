use anyhow::{anyhow, Context};
use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, digit1, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

#[derive(Copy, Clone, Debug)]
enum Atom<'a> {
    Lit(u32),
    Ref(&'a str),
}

#[derive(Copy, Clone, Debug)]
enum Expr<'a> {
    Atom(Atom<'a>),
    Not(Atom<'a>),
    And(Atom<'a>, Atom<'a>),
    Or(Atom<'a>, Atom<'a>),
    Rshift(Atom<'a>, Atom<'a>),
    Lshift(Atom<'a>, Atom<'a>),
}

struct Wire<'a> {
    wire: &'a str,
    expr: Expr<'a>,
}

fn posint(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(s)
}

fn parse_atom(s: &str) -> IResult<&str, Atom> {
    alt((map(posint, Atom::Lit), map(alphanumeric1, Atom::Ref)))(s)
}

fn parse_not(s: &str) -> IResult<&str, Expr> {
    preceded(tag("NOT "), map(parse_atom, Expr::Not))(s)
}

fn parse_ref(s: &str) -> IResult<&str, Expr> {
    map(parse_atom, Expr::Atom)(s)
}

fn parse_binop<'a, F>(
    syntax: &'static str,
    cons: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Expr<'a>>
where
    F: Fn(Atom<'a>, Atom<'a>) -> Expr<'a> + 'a,
{
    map(
        tuple((
            terminated(parse_atom, space1),
            terminated(tag(syntax), space1),
            parse_atom,
        )),
        move |(lhs, _, rhs)| cons(lhs, rhs),
    )
}

fn parse_expr(s: &str) -> IResult<&str, Expr> {
    alt((
        parse_not,
        parse_binop("AND", Expr::And),
        parse_binop("OR", Expr::Or),
        parse_binop("LSHIFT", Expr::Lshift),
        parse_binop("RSHIFT", Expr::Rshift),
        parse_ref,
    ))(s)
}

fn parse_wire(s: &str) -> IResult<&str, Wire> {
    let (s, (expr, wire)) = separated_pair(parse_expr, tag(" -> "), alphanumeric1)(s)?;
    Ok((s, Wire { wire, expr }))
}

fn parse(s: &str) -> anyhow::Result<Vec<Wire>> {
    Ok(separated_list1(char('\n'), parse_wire)(s)
        .map_err(|err| anyhow!("{err}"))?
        .1)
}

fn evaluate_atom<'a>(
    atom: &'a Atom,
    wires_by_name: &'a FxHashMap<&'a str, Expr>,
    bindings: &mut FxHashMap<&'a str, u32>,
) -> anyhow::Result<u32> {
    match atom {
        Atom::Lit(v) => Ok(*v),
        Atom::Ref(wire) => {
            if let Some(v) = bindings.get(wire) {
                Ok(*v)
            } else {
                let v = evaluate(wire, wires_by_name, bindings)?;
                bindings.insert(*wire, v);
                Ok(v)
            }
        }
    }
}

fn evaluate<'a>(
    wire: &'a str,
    wires_by_name: &'a FxHashMap<&str, Expr>,
    bindings: &mut FxHashMap<&'a str, u32>,
) -> anyhow::Result<u32> {
    if let Some(calculated) = bindings.get(wire) {
        Ok(*calculated)
    } else {
        let expr = wires_by_name
            .get(wire)
            .with_context(|| format!("Unknown wire: {wire}"))?;
        let answer = match expr {
            Expr::Atom(Atom::Lit(v)) => *v,
            Expr::Atom(Atom::Ref(v)) => evaluate(v, wires_by_name, bindings)?,
            Expr::Not(a) => !evaluate_atom(a, wires_by_name, bindings)?,
            Expr::And(lhs, rhs) => {
                evaluate_atom(lhs, wires_by_name, bindings)?
                    & evaluate_atom(rhs, wires_by_name, bindings)?
            }
            Expr::Or(lhs, rhs) => {
                evaluate_atom(lhs, wires_by_name, bindings)?
                    | evaluate_atom(rhs, wires_by_name, bindings)?
            }
            Expr::Lshift(lhs, rhs) => {
                evaluate_atom(lhs, wires_by_name, bindings)?
                    << evaluate_atom(rhs, wires_by_name, bindings)?
            }
            Expr::Rshift(lhs, rhs) => {
                evaluate_atom(lhs, wires_by_name, bindings)?
                    >> evaluate_atom(rhs, wires_by_name, bindings)?
            }
        };
        bindings.insert(wire, answer);
        Ok(answer)
    }
}

fn read(s: &str) -> anyhow::Result<FxHashMap<&str, Expr>> {
    let wires = parse(s)?;
    let mut m = FxHashMap::default();
    for w in wires {
        m.insert(w.wire, w.expr);
    }
    Ok(m)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let by_wire = read(s)?;
    let ans = evaluate("a", &by_wire, &mut FxHashMap::default())?;
    Ok(ans.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let by_wire = read(s)?;
    let ans = evaluate("a", &by_wire, &mut FxHashMap::default())?;
    let ans = evaluate("a", &by_wire, &mut FxHashMap::from_iter([("b", ans)]))?;
    Ok(ans.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(matches!(
            parse_wire("123 -> x"),
            Ok((
                _,
                Wire {
                    expr: Expr::Atom(Atom::Lit(123)),
                    wire: "x"
                }
            ))
        ));
        assert!(matches!(
            parse_wire("x AND y -> z"),
            Ok((
                _,
                Wire {
                    expr: Expr::And(Atom::Ref("x"), Atom::Ref("y")),
                    wire: "z"
                }
            ))
        ));
        assert!(matches!(
            parse_wire("p LSHIFT 2 -> q"),
            Ok((
                _,
                Wire {
                    expr: Expr::Lshift(Atom::Ref("p"), Atom::Lit(2)),
                    wire: "q"
                }
            ))
        ));
        assert!(matches!(
            parse_wire("NOT e -> f"),
            Ok((
                _,
                Wire {
                    expr: Expr::Not(Atom::Ref("e")),
                    wire: "f"
                }
            ))
        ));
    }
}
