use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, one_of};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::{pair, preceded};
use nom::IResult;

enum Lexeme<'a> {
    Escaped(&'a str),
    Hex(&'a str),
    Lit(&'a str),
}

fn parse_escaped(s: &str) -> IResult<&str, Lexeme<'_>> {
    let (s, ch) = preceded(char('\\'), recognize(one_of("\\\"")))(s)?;
    Ok((s, Lexeme::Escaped(ch)))
}

fn parse_hex(s: &str) -> IResult<&str, Lexeme<'_>> {
    let hexdigs = "0123456789abcdefABCDEF";
    let (s, hex) = preceded(
        tag("\\x"),
        recognize(pair(one_of(hexdigs), one_of(hexdigs))),
    )(s)?;
    Ok((s, Lexeme::Hex(hex)))
}

fn parse(s: &str) -> anyhow::Result<Vec<Lexeme<'_>>> {
    many0(alt((
        parse_hex,
        parse_escaped,
        map(recognize(anychar), Lexeme::Lit),
    )))(s)
    .map_err(|err| anyhow!("{err}"))
    .map(|ok| ok.1)
}

impl<'a> Lexeme<'a> {
    fn escaped_len(&self) -> usize {
        use Lexeme::*;
        match self {
            Escaped(c) => c.len() + 1, // \"
            Hex(h) => h.len() + 2,     // \xab
            Lit(c) => c.len(),
        }
    }
    fn len(&self) -> usize {
        use Lexeme::*;
        match self {
            Escaped(s) | Lit(s) => s.len(),
            Hex(_) => 1,
        }
    }
    fn escape_it_len(&self) -> usize {
        use Lexeme::*;
        match self {
            Lit(c) => c.len(),
            Hex(h) => h.len() + 3,     // \xab -> \\xab
            Escaped(c) => c.len() + 3, // \" -> \\\"
        }
    }
}

fn lengthdiff<'a>(lexemes: impl Iterator<Item = Lexeme<'a>>) -> usize {
    lexemes.map(|l| l.escaped_len() - l.len()).sum::<usize>() + 2 // open and close "
}

fn p2_lengthdiff<'a>(lexemes: impl Iterator<Item = Lexeme<'a>>) -> usize {
    4 + // "" -> "\"\""
    lexemes.map(|l| l.escape_it_len() - l.escaped_len()).sum::<usize>()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let r: anyhow::Result<Vec<_>> = s
        .lines()
        .map(|s| {
            let lex = parse(s)?;
            Ok(lengthdiff(lex.into_iter()))
        })
        .collect();
    let r: usize = r?.iter().sum();
    Ok(r.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let r: anyhow::Result<Vec<_>> = s
        .lines()
        .map(|s| {
            let lex = parse(s)?;
            Ok(p2_lengthdiff(lex.into_iter()))
        })
        .collect();
    let r: usize = r?.iter().sum();
    Ok(r.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let s = parse("").unwrap();
        assert_eq!(lengthdiff(s.into_iter()), 2);
        let s = parse("abc").unwrap();
        assert_eq!(lengthdiff(s.into_iter()), 2);
        let s = parse("aaa\\\"aaa").unwrap();
        assert_eq!(lengthdiff(s.into_iter()), 3);
        let s = parse("\\x27").unwrap();
        assert_eq!(lengthdiff(s.into_iter()), 5);
    }

    #[test]
    fn test_unparse() {
        let s = parse("").unwrap();
        assert_eq!(p2_lengthdiff(s.into_iter()), 4);
        let s = parse("abc").unwrap();
        assert_eq!(p2_lengthdiff(s.into_iter()), 4);
        let s = parse("aaa\\\"aaa").unwrap();
        assert_eq!(p2_lengthdiff(s.into_iter()), 6);
        let s = parse("\\x27").unwrap();
        assert_eq!(p2_lengthdiff(s.into_iter()), 5);
    }
}
