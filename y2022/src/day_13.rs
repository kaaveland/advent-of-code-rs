use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

use anyhow::{Context, Error, Result};

#[derive(PartialEq, Eq, Debug)]
enum Packets {
    Integer(i32),
    List(Vec<Packets>),
}

impl PartialOrd for Packets {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packets {
    fn cmp(&self, other: &Self) -> Ordering {
        use std::cmp::Ordering::{Equal, Greater, Less};
        use Packets::{Integer, List};
        match self {
            Integer(left) => match other {
                Integer(right) => left.cmp(right),
                List(_) => List(vec![Integer(*left)]).cmp(other),
            },
            List(left) => match other {
                Integer(right) => self.cmp(&List(vec![Integer(*right)])),
                List(right) => left
                    .iter()
                    .zip(right)
                    .find_map(|(left, right)| match left.cmp(right) {
                        Less => Some(Less),
                        Equal => None,
                        Greater => Some(Greater),
                    })
                    .unwrap_or_else(|| left.len().cmp(&right.len())),
            },
        }
    }
}

fn list_from_line(line: &str) -> Result<Packets> {
    parse_item(&mut line.chars().peekable())
}

fn parse_item(chars: &mut Peekable<Chars>) -> Result<Packets> {
    match chars.next() {
        Some('[') => parse_list(chars),
        Some(c) => {
            let mut digits: String = c.into();
            while let Some(digit) = chars.next_if(|&ch| ch.is_ascii_digit()) {
                digits.push(digit);
            }
            let int = digits.parse()?;
            Ok(Packets::Integer(int))
        }
        None => Err(Error::msg("Parse error")),
    }
}

fn parse_list(chars: &mut Peekable<Chars>) -> Result<Packets> {
    let mut many = Vec::new();
    loop {
        if let Some(ch) = chars.next_if(|&ch| ch == ',' || ch == ']') {
            match ch {
                ',' => {}
                ']' => break,
                _ => {
                    return Err(Error::msg("Syntax error"));
                }
            }
        } else {
            let item = parse_item(chars)?;
            many.push(item);
        }
    }
    Ok(Packets::List(many))
}

fn line_pairs(lines: &str) -> Result<Vec<(Packets, Packets)>> {
    let mut result = Vec::new();
    let mut it = lines.lines();
    while let Some(left) = it.next() {
        let right = it.next().context("Need pair of lines")?;
        let _blank = it.next();
        let left_p = list_from_line(left)?;
        let right_p = list_from_line(right)?;
        result.push((left_p, right_p))
    }
    Ok(result)
}

pub fn part_1(input: &str) -> Result<String> {
    let pairs = line_pairs(input)?;
    let mut sum = 0;
    for (index, (left, right)) in pairs.iter().enumerate() {
        if left < right {
            sum += index + 1;
        }
    }
    Ok(format!("{sum}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let pairs = line_pairs(input)?;
    let firsts = pairs.iter().map(|(left, _)| left);
    let snds = pairs.iter().map(|(_, right)| right);
    let mut all: Vec<_> = firsts.chain(snds).collect();
    let d1 = list_from_line("[[2]]")?;
    let d2 = list_from_line("[[6]]")?;
    all.push(&d1);
    all.push(&d2);
    all.sort();
    let mut d1_i = 0;
    let mut d2_i = 0;
    for (i, p) in all.iter().enumerate() {
        if *p == &d1 {
            d1_i = i + 1;
        } else if *p == &d2 {
            d2_i = i + 1;
        }
    }
    let sol = d1_i * d2_i;
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let left = list_from_line("[1,1,3,1,1]").unwrap();
        let right = list_from_line("[1,1,5,1,1]").unwrap();
        assert!(left < right);
    }

    #[test]
    fn test_example_2() {
        let left = list_from_line("[[1],[2,3,4]]").unwrap();
        let right = list_from_line("[[1],4]").unwrap();
        assert!(left < right);
    }

    #[test]
    fn test_example_3() {
        let left = list_from_line("[9]").unwrap();
        let right = list_from_line("[[8,7,6]]").unwrap();
        assert!(left > right);
    }

    #[test]
    fn test_example_4() {
        let left = list_from_line("[[4,4],4,4]").unwrap();
        let right = list_from_line("[[4,4],4,4,4]").unwrap();
        assert!(left < right);
    }

    #[test]
    fn test_example_5() {
        let left = list_from_line("[7,7,7,7]").unwrap();
        let right = list_from_line("[7,7,7]").unwrap();
        assert!(left > right);
    }

    #[test]
    fn test_example_6() {
        let left = list_from_line("[]").unwrap();
        let right = list_from_line("[3]").unwrap();
        assert!(left < right);
    }

    #[test]
    fn test_example_7() {
        let left = list_from_line("[[[]]]").unwrap();
        let right = list_from_line("[[]]").unwrap();
        assert!(left > right);
    }

    #[test]
    fn test_example_8() {
        let left = list_from_line("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let right = list_from_line("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert!(left > right);
    }
}
