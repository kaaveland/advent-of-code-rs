use anyhow::Context;

#[derive(Debug, Eq, PartialEq)]
struct Equation {
    equals: u64,
    operands: Vec<u64>,
}

fn parse_equations(input: &str) -> anyhow::Result<Vec<Equation>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, rest) = line.split_once(':').context("Missing :")?;
            let equals = first.parse()?;
            let operands: anyhow::Result<Vec<_>> = rest
                .split_whitespace()
                .filter(|n| !n.is_empty())
                .map(|n| Ok(n.parse()?))
                .collect();
            Ok(Equation {
                equals,
                operands: operands?,
            })
        })
        .collect()
}

fn reduces<F>(left: u64, target: u64, rest: &[u64], ops: &[F]) -> bool
where
    F: Fn(u64, u64) -> u64,
{
    if left > target {
        false
    } else if rest.is_empty() {
        left == target
    } else {
        ops.iter()
            .any(|op| reduces(op(left, rest[0]), target, &rest[1..], ops))
    }
}

fn sum_reducible<F>(equations: &[Equation], ops: &[F]) -> u64
where
    F: Fn(u64, u64) -> u64,
{
    equations
        .iter()
        .filter_map(|eq| {
            if reduces(eq.operands[0], eq.equals, &eq.operands[1..], ops) {
                Some(eq.equals)
            } else {
                None
            }
        })
        .sum()
}

fn concat(left: u64, right: u64) -> u64 {
    let exp = u64::ilog10(right) + 1;
    10u64.pow(exp) * left + right
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let equations = parse_equations(input)?;
    let n = sum_reducible(&equations, &vec![|x, y| x * y, (|x, y| x + y)]);
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let equations = parse_equations(input)?;
    let n = sum_reducible(
        &equations,
        &vec![|x, y| x * y, |x, y| x + y, |x, y| concat(x, y)],
    );
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_p1() {
        assert_eq!(part_1(EXAMPLE).unwrap().as_str(), "3749");
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(EXAMPLE).unwrap().as_str(), "11387");
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 345), 12345);
    }
}
