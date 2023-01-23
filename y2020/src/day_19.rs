use anyhow::anyhow;
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{alpha1, char as match_char, digit1, one_of, space0, space1};
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Rule {
    Lit(char),
    Seq(Vec<usize>),
    Choice(Box<Rule>, Box<Rule>),
}

fn parse_id(i: &str) -> IResult<&str, usize> {
    map_res(terminated(digit1, match_char(':')), FromStr::from_str)(i)
}

fn parse_lit(i: &str) -> IResult<&str, Rule> {
    map(
        delimited(match_char('"'), one_of("ab"), match_char('"')),
        Rule::Lit,
    )(i)
}

fn parse_seq(i: &str) -> IResult<&str, Rule> {
    let single = map_res(terminated(digit1, space0), FromStr::from_str);
    map(many1(single), Rule::Seq)(i)
}

fn parse_choice(i: &str) -> IResult<&str, Rule> {
    let (i, left) = parse_seq(i)?;
    let (i, _) = terminated(match_char('|'), space1)(i)?;
    let (i, right) = parse_seq(i)?;
    Ok((i, Rule::Choice(Box::new(left), Box::new(right))))
}

fn parse_rule(i: &str) -> IResult<&str, (usize, Rule)> {
    let (i, id) = parse_id(i)?;
    let (i, rule) = preceded(space0, alt((parse_choice, parse_seq, parse_lit)))(i)?;
    Ok((i, (id, rule)))
}

fn parse_rules(i: &str) -> IResult<&str, Vec<(usize, Rule)>> {
    terminated(
        many1(terminated(parse_rule, match_char('\n'))),
        match_char('\n'),
    )(i)
}

pub fn part_1(input: &str) -> Result<String, anyhow::Error> {
    Ok("Not implemented yet".into())
}

#[cfg(test)]
mod tests {
    use crate::day_19::{parse_id, parse_seq, Rule};

    #[test]
    fn test_parse_rules() {
        assert_eq!(parse_id("1: 2 3 | 3 2\n"), Ok((" 2 3 | 3 2\n", 1)));
        assert_eq!(parse_seq("1 2 3 |"), Ok(("|", Rule::Seq(vec![1, 2, 3]))));
        assert_eq!(
            parse_choice("1 | 2"),
            Ok((
                "",
                Rule::Choice(Box::new(Rule::Seq(vec![1])), Box::new(Rule::Seq(vec![2])))
            ))
        );
        let (_, rules) = parse_rules(EXAMPLE).unwrap();
        assert_eq!(rules.len(), 6);
        assert_eq!(rules[5].1, Rule::Lit('b'));
    }
    const EXAMPLE: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";
    use super::*;
}
