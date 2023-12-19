use anyhow::{anyhow, Result};
use fxhash::FxHashMap as Map;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, one_of};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn parse_var<'a>(v: char) -> impl FnMut(&'a str) -> IResult<&'a str, i32> {
    preceded(
        preceded(char(v), char('=')),
        map_res(digit1, FromStr::from_str),
    )
}
fn parse_part(s: &str) -> IResult<&str, Part> {
    let inner = tuple((
        parse_var('x'),
        preceded(char(','), parse_var('m')),
        preceded(char(','), parse_var('a')),
        preceded(char(','), parse_var('s')),
    ));
    let (s, (x, m, a, s_)) = delimited(char('{'), inner, char('}'))(s)?;

    Ok((s, Part { x, m, a, s: s_ }))
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Attribute {
    X,
    M,
    A,
    S,
}
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum WorkflowRule {
    Always,
    AttrLess(Attribute, i32),
    AttrMore(Attribute, i32),
}
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Destination<'a> {
    Accept,
    Reject,
    Rule(&'a str),
}

fn parse_attribute(s: &str) -> IResult<&str, Attribute> {
    let (s, ch) = one_of("xmas")(s)?;
    let a = match ch {
        'x' => Attribute::X,
        'm' => Attribute::M,
        'a' => Attribute::A,
        's' => Attribute::S,
        _ => unreachable!(),
    };
    Ok((s, a))
}
fn parse_destination(s: &str) -> IResult<&str, Destination> {
    alt((
        map(char('A'), |_| Destination::Accept),
        map(char('R'), |_| Destination::Reject),
        map(alpha1, Destination::Rule),
    ))(s)
}
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Rule<'a> {
    rule: WorkflowRule,
    dest: Destination<'a>,
}
fn parse_workflow_rule(s: &str) -> IResult<&str, Rule> {
    let condition_rule = map(
        tuple((
            parse_attribute,
            one_of("<>"),
            map_res(digit1, FromStr::from_str),
        )),
        |(att, ch, d)| match ch {
            '<' => WorkflowRule::AttrLess(att, d),
            '>' => WorkflowRule::AttrMore(att, d),
            _ => unreachable!(),
        },
    );
    let cond = map(
        separated_pair(condition_rule, char(':'), parse_destination),
        |(rule, dest)| Rule { rule, dest },
    );
    alt((
        cond,
        map(parse_destination, |dest| Rule {
            rule: WorkflowRule::Always,
            dest,
        }),
    ))(s)
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

fn parse_workflow(s: &str) -> IResult<&str, Workflow> {
    let (s, name) = alpha1(s)?;
    let (s, rules) = delimited(
        char('{'),
        separated_list1(char(','), parse_workflow_rule),
        char('}'),
    )(s)?;
    Ok((s, Workflow { name, rules }))
}

fn parse(s: &str) -> Result<(Map<&str, Workflow>, Vec<Part>)> {
    Ok(separated_pair(
        map(separated_list1(char('\n'), parse_workflow), by_name),
        tag("\n\n"),
        separated_list1(char('\n'), parse_part),
    )(s)
    .map_err(|err| anyhow!("{err}"))?
    .1)
}

fn by_name(workflows: Vec<Workflow>) -> Map<&str, Workflow> {
    workflows.into_iter().map(|wf| (wf.name, wf)).collect()
}

impl Attribute {
    fn get(&self, part: &Part) -> i32 {
        match self {
            Attribute::X => part.x,
            Attribute::M => part.m,
            Attribute::A => part.a,
            Attribute::S => part.s,
        }
    }

    fn get_range(&self, part: &PartRange) -> (i32, i32) {
        match self {
            Attribute::X => part.x,
            Attribute::M => part.m,
            Attribute::A => part.a,
            Attribute::S => part.s,
        }
    }

    fn set_range(&self, part: &PartRange, range: (i32, i32)) -> PartRange {
        let mut p = *part;
        match self {
            Attribute::X => {
                p.x = range;
            }
            Attribute::M => {
                p.m = range;
            }
            Attribute::A => {
                p.a = range;
            }
            Attribute::S => {
                p.s = range;
            }
        }
        p
    }
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct PartRange {
    x: (i32, i32),
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32),
}
impl WorkflowRule {
    fn applies_to(&self, part: &Part) -> bool {
        match self {
            WorkflowRule::Always => true,
            WorkflowRule::AttrLess(att, than) => att.get(part) < *than,
            WorkflowRule::AttrMore(att, than) => att.get(part) > *than,
        }
    }
    fn split_range(&self, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self {
            WorkflowRule::Always => (Some(*part), None),
            WorkflowRule::AttrLess(att, than) => {
                let (start, end) = att.get_range(part);
                if start < *than {
                    let left = (start, *than - 1);
                    let right = (*than, end);
                    (
                        Some(att.set_range(part, left)).filter(|_| start < *than),
                        Some(att.set_range(part, right)).filter(|_| end >= *than),
                    )
                } else {
                    (None, Some(*part))
                }
            }
            WorkflowRule::AttrMore(att, than) => {
                let (start, end) = att.get_range(part);
                if end > *than {
                    let left = (start, *than);
                    let right = (*than + 1, end);
                    (
                        Some(att.set_range(part, right)).filter(|_| end > *than),
                        Some(att.set_range(part, left)).filter(|_| start <= *than),
                    )
                } else {
                    (None, Some(*part))
                }
            }
        }
    }
}

impl<'a> Workflow<'a> {
    fn apply_to(&self, part: &Part) -> Option<Destination> {
        for workflow_rule in self.rules.iter() {
            if workflow_rule.rule.applies_to(part) {
                return Some(workflow_rule.dest);
            }
        }
        None
    }
    fn split_range(&self, part: &PartRange) -> Vec<(Destination, PartRange)> {
        let mut o = vec![];
        let mut r = *part;
        for workflow_rule in self.rules.iter() {
            let (stopped, not) = workflow_rule.rule.split_range(&r);
            if let Some(stopped) = stopped {
                o.push((workflow_rule.dest, stopped));
            }
            if let Some(not) = not {
                r = not;
            } else {
                assert_eq!(r, stopped.unwrap());
                break;
            }
        }
        o
    }
}

impl PartRange {
    fn score(&self) -> u64 {
        fn score(r: (i32, i32)) -> u64 {
            let (start, end) = r;
            (end - start + 1) as u64
        }
        score(self.x) * score(self.m) * score(self.a) * score(self.s)
    }
}
fn range_splitting(workflows: &Map<&str, Workflow>) -> u64 {
    let mut combinations: u64 = 0;
    let mut work = vec![(
        "in",
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    )];
    while let Some((stage, range)) = work.pop() {
        if let Some(wf) = workflows.get(stage) {
            let next = wf.split_range(&range);
            for (d, n) in next {
                match d {
                    Destination::Accept => {
                        combinations += n.score();
                    }
                    Destination::Reject => {}
                    Destination::Rule(r) => {
                        work.push((r, n));
                    }
                }
            }
        }
    }
    combinations
}

fn is_part_accepted(workflows: &Map<&str, Workflow>, part: &Part) -> bool {
    let mut name = "in";
    while let Some(wf) = workflows.get(name) {
        if let Some(next) = wf.apply_to(part) {
            match next {
                Destination::Accept => {
                    return true;
                }
                Destination::Reject => {
                    return false;
                }
                Destination::Rule(n) => {
                    name = n;
                }
            }
        }
    }
    unreachable!()
}

fn add_accepted_parts(input: &str) -> Result<i32> {
    let (wf, parts) = parse(input)?;
    Ok(parts
        .into_iter()
        .filter(|part| is_part_accepted(&wf, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum())
}

fn add_range_combinations(input: &str) -> Result<u64> {
    let (wf, _) = parse(input)?;
    Ok(range_splitting(&wf))
}

pub fn part_1(s: &str) -> Result<String> {
    add_accepted_parts(s).map(|n| n.to_string())
}

pub fn part_2(s: &str) -> Result<String> {
    add_range_combinations(s).map(|n| n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_p1() {
        assert_eq!(add_accepted_parts(EX).unwrap(), 19114);
    }

    #[test]
    fn test_p2() {
        assert_eq!(add_range_combinations(EX).unwrap(), 167409079868000);
    }

    #[test]
    fn parses_ex() {
        assert!(parse(EX).is_ok());
        let (wf, parts) = parse(EX).unwrap();
        assert_eq!(wf.len(), 11);
        assert_eq!(parts.len(), 5);
        assert_eq!(
            parts[4],
            Part {
                x: 2127,
                m: 1623,
                a: 2188,
                s: 1013
            }
        );
        assert_eq!(
            *wf.get("hdj").unwrap(),
            Workflow {
                name: "hdj",
                rules: vec![
                    Rule {
                        rule: WorkflowRule::AttrMore(Attribute::M, 838),
                        dest: Destination::Accept
                    },
                    Rule {
                        rule: WorkflowRule::Always,
                        dest: Destination::Rule("pv")
                    }
                ]
            }
        );
    }

    #[test]
    fn test_workflow_parser() {
        let wf = "px{a<2006:qkq,m>2090:A,rfg}";
        assert_eq!(
            parse_workflow(wf).unwrap().1,
            Workflow {
                name: "px",
                rules: vec![
                    Rule {
                        rule: WorkflowRule::AttrLess(Attribute::A, 2006),
                        dest: Destination::Rule("qkq")
                    },
                    Rule {
                        rule: WorkflowRule::AttrMore(Attribute::M, 2090),
                        dest: Destination::Accept
                    },
                    Rule {
                        rule: WorkflowRule::Always,
                        dest: Destination::Rule("rfg")
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part_parser() {
        assert_eq!(
            parse_part("{x=787,m=2655,a=1222,s=2876}").unwrap().1,
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }
        );
    }
}
