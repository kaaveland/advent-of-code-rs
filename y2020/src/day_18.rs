use anyhow::anyhow;
use nom::branch::alt;
use nom::character::complete::{char as parse_char, digit1, one_of, space0};
use nom::combinator::{map, map_res};
use nom::multi::fold_many0;
use nom::sequence::{pair, preceded, terminated};
use nom::{Finish, IResult};

// int = -digit | digi
// digit = [0-9] | digit[0-9]
fn parse_minus(i: &str) -> IResult<&str, char> {
    parse_char('-')(i)
}

fn parse_pos_int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, |n: &str| n.parse::<i64>())(i)
}

fn parse_neg_int(i: &str) -> IResult<&str, i64> {
    map(preceded(parse_minus, parse_pos_int), |n| -n)(i)
}

fn parse_int(i: &str) -> IResult<&str, i64> {
    preceded(space0, alt((parse_neg_int, parse_pos_int)))(i)
}

fn calc_1(line: &str) -> Result<i64, anyhow::Error> {
    fn parse_operator(i: &str) -> IResult<&str, char> {
        terminated(preceded(space0, one_of("+-*/")), space0)(i)
    }

    fn operand(i: &str) -> IResult<&str, i64> {
        alt((parse_int, parens))(i)
    }

    fn parens(i: &str) -> IResult<&str, i64> {
        let (i, _begin) = preceded(space0, parse_char('('))(i)?;
        let (i, val) = expr(i)?;
        let (i, _end) = preceded(space0, parse_char(')'))(i)?;
        Ok((i, val))
    }

    fn expr(i: &str) -> IResult<&str, i64> {
        // Possibly parenthesised expr, 1 + 2 * 3 ..
        let (i, lhs) = operand(i)?;
        fold_many0(
            pair(parse_operator, operand),
            move || lhs,
            |acc, (op, rhs)| match op {
                '+' => acc + rhs,
                '-' => acc - rhs,
                '/' => acc / rhs,
                '*' => acc * rhs,
                _ => unreachable!(),
            },
        )(i)
    }

    expr(line)
        .finish()
        .map_err(|e| anyhow!("Unable to parse: {line} due to {e:?}"))
        .map(|(rem, r)| {
            assert_eq!(rem.len(), 0);
            r
        })
}

pub fn part_1(input: &str) -> Result<String, anyhow::Error> {
    let mut sum = 0;
    for line in input.lines() {
        let add = calc_1(line)?;
        sum += add;
    }
    Ok(format!("{sum}"))
}

fn calc_2(line: &str) -> Result<i64, anyhow::Error> {
    fn addsub(i: &str) -> IResult<&str, char> {
        // " + "
        preceded(space0, terminated(one_of("+-"), space0))(i)
    }
    fn muldiv(i: &str) -> IResult<&str, char> {
        // " * "
        preceded(space0, terminated(one_of("*/"), space0))(i)
    }
    fn operand(i: &str) -> IResult<&str, i64> {
        // " 91 "
        preceded(space0, terminated(alt((parse_int, parens)), space0))(i)
    }
    fn parens(i: &str) -> IResult<&str, i64> {
        // "(expr)"
        let (i, _begin) = preceded(space0, parse_char('('))(i)?;
        let (i, val) = expr(i)?;
        let (i, _end) = preceded(space0, parse_char(')'))(i)?;
        Ok((i, val))
    }
    fn factor(i: &str) -> IResult<&str, i64> {
        let (i, lhs) = operand(i)?;

        fold_many0(
            pair(addsub, operand),
            move || lhs,
            |acc, (op, rhs)| match op {
                '+' => acc + rhs,
                '-' => acc - rhs,
                _ => unreachable!(),
            },
        )(i)
    }
    fn expr(i: &str) -> IResult<&str, i64> {
        // " 91 " || " 91 + ... " || "(expr) + ...
        let (i, lhs) = factor(i)?;

        fold_many0(
            pair(muldiv, factor),
            move || lhs,
            |acc, (op, rhs)| match op {
                '*' => acc * rhs,
                '/' => acc / rhs,
                _ => unreachable!(),
            },
        )(i)
    }
    expr(line)
        .map_err(|e| anyhow!("Unable to parse: '{line}' due to {e:?}"))
        .map(|(_, res)| res)
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;
    for line in input.lines() {
        let add = calc_2(line)?;
        sum += add;
    }
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ints() {
        let n = parse_int("-14").unwrap().1;
        assert_eq!(n, -14);
        let n = parse_int("17").unwrap().1;
        assert_eq!(n, 17);
        let n = parse_int(" 17").unwrap().1;
        assert_eq!(n, 17);
        let n = parse_int("lol -17");
        assert!(n.is_err());
        let n = parse_int("\n17");
        assert!(n.is_err());
    }

    #[test]
    fn calculates_p1_expr() {
        let n = calc_1("4 * 5").unwrap();
        assert_eq!(n, 20);
        let n = calc_1(" 4* 5").unwrap();
        assert_eq!(n, 20);
        let n = calc_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
        assert_eq!(n, 13632);
    }

    #[test]
    fn calculates_p2_expr() {
        let n = calc_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
        assert_eq!(n, 23340);
        let n = calc_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap();
        assert_eq!(n, 669060);
    }
}
