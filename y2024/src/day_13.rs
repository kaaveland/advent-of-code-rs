use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Equation {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl Equation {
    fn as_part_2(&self) -> Self {
        Self {
            px: self.px + 10000000000000,
            py: self.py + 10000000000000,
            ..*self
        }
    }
    fn solve(&self) -> Option<(i64, i64)> {
        let det = (self.ax * self.by) - (self.bx * self.ay);
        if det == 0 {
            None
        } else {
            let a = (self.px * self.by - self.bx * self.py) / det;
            let b = (self.ax * self.py - self.px * self.ay) / det;
            if a * self.ax + b * self.bx == self.px && a * self.ay + b * self.by == self.py {
                Some((a, b))
            } else {
                None
            }
        }
    }
}

fn posint(input: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, (ax, ay)) = tuple((
        preceded(tag("Button A: X+"), posint),
        preceded(tag(", Y+"), posint),
    ))(input)?;
    let (input, (bx, by)) = tuple((
        preceded(tag("\nButton B: X+"), posint),
        preceded(tag(", Y+"), posint),
    ))(input)?;
    let (input, (px, py)) = tuple((
        preceded(tag("\nPrize: X="), posint),
        preceded(tag(", Y="), posint),
    ))(input)?;

    Ok((
        input,
        Equation {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        },
    ))
}

fn solve(input: &str, p2: bool) -> anyhow::Result<i64> {
    let (_, equations) = separated_list1(tag("\n\n"), parse_equation)(input)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(equations
        .into_iter()
        .map(|eq| if p2 { eq.as_part_2() } else { eq })
        .map(|eq| {
            if let Some((a, b)) = eq.solve() {
                a * 3 + b
            } else {
                0
            }
        })
        .sum())
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    solve(input, false).map(|n| format!("{n}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    solve(input, true).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_parse() {
        let ex = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";
        let (_, eq) = parse_equation(ex).unwrap();
        assert_eq!(
            eq,
            Equation {
                ax: 94,
                ay: 34,
                bx: 22,
                by: 67,
                px: 8400,
                py: 5400,
            }
        );
    }

    #[test]
    fn test_p1() {
        assert_eq!(solve(EXAMPLE, false).unwrap(), 480);
    }
}
