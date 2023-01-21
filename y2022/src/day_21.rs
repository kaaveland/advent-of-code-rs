use anyhow::{anyhow, Context, Result};

use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Debug)]
pub enum Atom {
    Binding(String),
    Int(i64),
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Constant(Atom),
    Operator(char, Atom, Atom),
}

#[cfg(test)]
impl Atom {
    fn bind(s: &str) -> Atom {
        Atom::Binding(s.into())
    }
}

fn parse_int(expr: &str) -> Result<Atom> {
    expr.parse().map(Atom::Int).context("Not an int")
}

fn parse_binding(expr: &str) -> Result<Atom> {
    Ok(Atom::Binding(expr.into()))
}

fn parse_operator(expr: &str) -> Result<char> {
    let first = expr.chars().next();
    if expr.len() > 1 {
        Err(anyhow!("Expected single char, got {}", expr))
    } else {
        match first {
            Some('+' | '-' | '/' | '*') => Ok(first.unwrap()),
            _ => Err(anyhow!("Expected operator, got {:?}", first)),
        }
    }
}

fn parse_expr(expr: &str) -> Result<Expression> {
    expr.parse::<i64>()
        .map(Atom::Int)
        .map(Expression::Constant)
        .or_else(|_| {
            let mut parts = expr.split_ascii_whitespace();
            let first = parts.next().context("Expected operand")?;
            let left = parse_int(first).or_else(|_| parse_binding(first))?;
            let operator = parts
                .next()
                .context("Expected =+/-")
                .map(parse_operator)??;
            let third = parts.next().context("Expected operand")?;
            let right = parse_int(third).or_else(|_| parse_binding(third))?;
            Ok(Expression::Operator(operator, left, right))
        })
}

fn parse(input: &str) -> Result<HashMap<String, Expression>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let expr = parts.next().context("No expression").map(parse_expr)??;
            Ok((name.into(), expr))
        })
        .collect()
}

fn resolve(expr: &Expression, bindings: &HashMap<String, i64>) -> Option<i64> {
    match expr {
        Expression::Constant(Atom::Int(n)) => Some(*n),
        Expression::Constant(Atom::Binding(key)) => bindings.get(key).copied(),
        Expression::Operator(op, Atom::Int(left), Atom::Int(right)) => {
            Some(resolve_operator(op, left, right))
        }
        Expression::Operator(op, Atom::Binding(left), Atom::Binding(right))
            if bindings.contains_key(left) && bindings.contains_key(right) =>
        {
            Some(resolve_operator(
                op,
                bindings.get(left).unwrap(),
                bindings.get(right).unwrap(),
            ))
        }
        Expression::Operator(op, Atom::Binding(left), Atom::Int(n))
            if bindings.contains_key(left) =>
        {
            Some(resolve_operator(op, bindings.get(left).unwrap(), n))
        }
        Expression::Operator(op, Atom::Int(n), Atom::Binding(right))
            if bindings.contains_key(right) =>
        {
            Some(resolve_operator(op, n, bindings.get(right).unwrap()))
        }
        _ => None,
    }
}

fn resolve_operator(op: &char, left: &i64, right: &i64) -> i64 {
    match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => panic!("Unknown operator: {op}"),
    }
}

fn calculate(equation: &HashMap<String, Expression>) -> HashMap<String, i64> {
    let mut known_bindings: HashMap<String, i64> = HashMap::new();
    let mut queue: VecDeque<_> = equation.keys().collect();
    let mut last_solve = 0;

    while last_solve < queue.len() {
        if let Some(name) = queue.pop_front() {
            if let Some(new_binding) = equation
                .get(name)
                .and_then(|expr| resolve(expr, &known_bindings))
            {
                known_bindings.insert(name.into(), new_binding);
                last_solve = 0;
            } else {
                last_solve += 1;
                queue.push_back(name);
            }
        }
    }
    known_bindings
}

fn calculate_part_2(mut equation: HashMap<String, Expression>) -> i64 {
    equation.insert(
        "humn".into(),
        Expression::Constant(Atom::Binding("humn".into())),
    );

    let node = equation.get("root").unwrap();
    let mut known_bindings = calculate(&equation);

    let (left, right) = match node {
        Expression::Operator(_, Atom::Binding(left), Atom::Binding(right)) => (left, right),
        _ => panic!("Unexpected root formula: {node:?}"),
    };
    let (left, right) = if known_bindings.contains_key(left) {
        (left, right)
    } else {
        (right, left)
    };
    let mut left_value = *known_bindings.get(left).unwrap();
    let mut right_expr = equation.get(right).unwrap();

    loop {
        match right_expr {
            Expression::Constant(Atom::Binding(s)) if s == "humn" => {
                return left_value;
            }
            Expression::Operator(op, Atom::Binding(new_left), Atom::Binding(new_right))
                if known_bindings.contains_key(new_left) =>
            {
                let val = *known_bindings.get(new_left).unwrap();
                let next = match op {
                    '+' => left_value - val,
                    '-' => val - left_value,
                    '/' => val / left_value,
                    '*' => left_value / val,
                    _ => panic!("Unknown op: {op}"),
                };
                known_bindings.insert(new_right.into(), next);
                left_value = next;
                right_expr = equation.get(new_right).unwrap();
            }
            Expression::Operator(op, Atom::Binding(new_left), Atom::Binding(new_right))
                if known_bindings.contains_key(new_right) =>
            {
                let val = *known_bindings.get(new_right).unwrap();
                let next = match op {
                    '+' => left_value - val,
                    '-' => left_value + val,
                    '/' => left_value * val,
                    '*' => left_value / val,
                    _ => panic!("Unknown op: {op}"),
                };
                known_bindings.insert(new_left.into(), next);
                left_value = next;
                right_expr = equation.get(new_left).unwrap();
            }
            _ => panic!("Not supposed to happen"),
        }
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let exprs = parse(input)?;
    let original = calculate(&exprs);
    let answer = original
        .get("root")
        .copied()
        .with_context(|| anyhow!("Unable to solve"))?;
    Ok(format!("{answer}"))
}
pub fn part_2(input: &str) -> Result<String> {
    let exprs = parse(input)?;
    let humn = calculate_part_2(exprs);
    Ok(format!("{humn}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    #[test]
    fn test_parse_binding_example() {
        let expr = parse_expr("pppw + sjmn").unwrap();
        assert_eq!(
            expr,
            Expression::Operator(
                '+',
                Atom::Binding("pppw".into()),
                Atom::Binding("sjmn".into())
            )
        );
    }

    #[test]
    fn test_parse_constant_example() {
        let expr = parse_expr("5").unwrap();
        assert_eq!(expr, Expression::Constant(Atom::Int(5)));
    }

    #[test]
    fn test_parse_mixed_example() {
        let expr = parse_expr("5 / drzm").unwrap();
        assert_eq!(
            expr,
            Expression::Operator('/', Atom::Int(5), Atom::Binding("drzm".into()))
        );
    }

    #[test]
    fn test_parse_example() {
        let exprs = parse(EXAMPLE).unwrap();
        assert_eq!(exprs.len(), 15);
        assert_eq!(
            exprs.get("lgvd"),
            Some(&Expression::Operator(
                '*',
                Atom::bind("ljgn"),
                Atom::bind("ptdq")
            ))
        );
    }

    #[test]
    fn test_calculate_example() {
        let exprs = parse(EXAMPLE).unwrap();
        let answer = calculate(&exprs);
        assert_eq!(answer.get("root"), Some(&152));
    }

    #[test]
    fn test_calculate_part_2() {
        let exprs = parse(EXAMPLE).unwrap();
        calculate_part_2(exprs);
    }
}
