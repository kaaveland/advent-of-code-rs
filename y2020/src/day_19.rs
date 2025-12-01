use anyhow::anyhow;
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char as match_char, digit1, one_of, space0, space1};
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::{Finish, IResult};
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

#[derive(Eq, PartialEq, Clone, Debug)]
enum Validation<'a> {
    Continue(&'a [char]),
    Branch(Vec<Validation<'a>>),
    Invalid,
}

fn validate_seq<'a>(
    line: &'a [char],
    seq: &Vec<usize>,
    rules: &HashMap<usize, Rule>,
) -> Validation<'a> {
    let mut possible = vec![line];
    for rule_no in seq {
        let mut next = vec![];
        while let Some(continuation) = possible.pop() {
            let result = validate_rule_no(continuation, *rule_no, rules);
            match result {
                Validation::Branch(mut options) => {
                    while let Some(cont) = options.pop() {
                        match cont {
                            Validation::Continue(s) => next.push(s),
                            Validation::Branch(choices) => options.extend(choices.into_iter()),
                            _ => {}
                        }
                    }
                }
                Validation::Continue(rest) => next.push(rest),
                _ => {}
            }
        }
        possible = next;
    }
    Validation::Branch(possible.into_iter().map(Validation::Continue).collect())
}

fn validate_lit(line: &[char], ch: char) -> Validation<'_> {
    match line.first() {
        None => Validation::Invalid,
        Some(lch) if ch != *lch => Validation::Invalid,
        _ => Validation::Continue(&line[1..]),
    }
}

fn validate_rule<'a>(
    line: &'a [char],
    rule: &Rule,
    rules: &HashMap<usize, Rule>,
) -> Validation<'a> {
    match rule {
        Rule::Lit(ch) => validate_lit(line, *ch),
        Rule::Seq(opts) => validate_seq(line, opts, rules),
        Rule::Choice(left, right) => {
            let left_path = validate_rule(line, left, rules);
            let right_path = validate_rule(line, right, rules);
            Validation::Branch(vec![left_path, right_path])
        }
    }
}

fn is_valid(line: &str, rules: &HashMap<usize, Rule>) -> bool {
    let line = line.chars().collect_vec();
    let validation = validate_rule_no(&line, 0, rules);
    fn inner(validation: &Validation) -> bool {
        match validation {
            Validation::Invalid => false,
            Validation::Continue(seq) => seq.is_empty(),
            Validation::Branch(options) => options.iter().any(inner),
        }
    }
    inner(&validation)
}

fn validate_rule_no<'a>(
    line: &'a [char],
    rule_no: usize,
    rules: &HashMap<usize, Rule>,
) -> Validation<'a> {
    let rule = rules.get(&rule_no);
    rule.map(|rule| validate_rule(line, rule, rules))
        .unwrap_or(Validation::Invalid)
}

pub fn part_1(input: &str) -> Result<String, anyhow::Error> {
    let (rest, rules) = parse_rules(input)
        .finish()
        .map_err(|e| anyhow!("Unable to parse due to {e:?}"))?;
    let rules: HashMap<usize, Rule> = rules.into_iter().collect();
    let n = rest
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_valid(line, &rules))
        .count();
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String, anyhow::Error> {
    let (rest, rules) = parse_rules(input)
        .finish()
        .map_err(|e| anyhow!("Unable to parse due to {e:?}"))?;
    let mut rules: HashMap<usize, Rule> = rules.into_iter().collect();
    let (_, new_rules) = parse_rules(
        "8: 42 | 42 8
11: 42 31 | 42 11 31

",
    )
    .finish()
    .map_err(|e| anyhow!("Unable to parse due to {e:?}"))?;
    for (id, rule) in new_rules.into_iter() {
        rules.insert(id, rule);
    }
    let n = rest
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_valid(line, &rules))
        .count();

    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_validate_rules() {
        let (_, rules) = parse_rules(EXAMPLE).unwrap();
        let rules: HashMap<usize, Rule> = rules.into_iter().collect();
        assert!(is_valid("aaaabb", &rules));
    }

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
