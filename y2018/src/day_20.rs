use anyhow::{anyhow, Context};
use fxhash::{FxHashMap, FxHashSet};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair};
use nom::IResult;
use std::collections::VecDeque;
use Ast::*;
use SimpleAst::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Ast {
    Seq(Vec<Ast>),
    Branch(Vec<Ast>),
    Simple(SimpleAst),
}

/// Could've been inlined in Ast, but we need this to be Copy to implement `parser_for` in a simple way
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SimpleAst {
    N,
    W,
    E,
    S,
}

fn parser_for<'a>(tag: char, node: SimpleAst) -> impl FnMut(&'a str) -> IResult<&'a str, Ast> + 'a {
    map(char(tag), move |_| Simple(node))
}

fn parse_direction(s: &str) -> IResult<&str, Ast> {
    alt((
        parser_for('N', N),
        parser_for('W', W),
        parser_for('S', S),
        parser_for('E', E),
    ))(s)
}

fn parse_empty_seq(s: &str) -> IResult<&str, Ast> {
    Ok((s, Seq(Vec::new())))
}

fn parse_expr(s: &str) -> IResult<&str, Ast> {
    let branch = delimited(
        char('('),
        map(
            separated_list1(char('|'), alt((parse_expr, parse_empty_seq))),
            Branch,
        ),
        char(')'),
    );
    let seq = pair(many1(parse_direction), many0(parse_expr));
    alt((
        map(seq, |(l, r)| Seq(l.into_iter().chain(r).collect())),
        branch,
    ))(s)
}

fn parse(s: &str) -> IResult<&str, Ast> {
    delimited(char('^'), parse_expr, char('$'))(s)
}

type Point = (i32, i32);

fn explore(
    positions: &FxHashSet<Point>,
    ast: &Ast,
    map: &mut FxHashMap<Point, FxHashSet<Point>>,
) -> FxHashSet<Point> {
    match ast {
        Seq(seq) => {
            let mut positions = FxHashSet::from_iter(positions.iter().copied());
            for sub in seq {
                positions = explore(&positions, sub, map);
            }
            positions
        }
        Branch(branches) => {
            let mut new_pos = FxHashSet::default();
            for branch in branches {
                new_pos.extend(explore(positions, branch, map));
            }
            new_pos
        }
        Simple(dir) => {
            let (dx, dy) = match dir {
                N => (0, -1),
                W => (-1, 0),
                E => (1, 0),
                S => (0, 1),
            };
            let goto: FxHashSet<_> = positions
                .iter()
                .copied()
                .map(|(x, y)| (x + dx, y + dy))
                .collect();
            for (from, to) in positions.iter().copied().zip(goto.iter().copied()) {
                map.entry(from).or_default().insert(to);
                map.entry(to).or_default().insert(from);
            }
            goto
        }
    }
}

fn shortest_paths(map: &FxHashMap<Point, FxHashSet<Point>>) -> FxHashMap<Point, usize> {
    let mut work = VecDeque::new();
    let empty = FxHashSet::default();
    work.push_back(((0, 0), 0));
    let mut paths = FxHashMap::default();
    while let Some((pos, dist)) = work.pop_front() {
        if let std::collections::hash_map::Entry::Vacant(e) = paths.entry(pos) {
            e.insert(dist);
            for next in map.get(&pos).unwrap_or(&empty).iter().copied() {
                work.push_back((next, dist + 1));
            }
        }
    }
    paths
}

fn find_paths(s: &str) -> anyhow::Result<FxHashMap<Point, usize>> {
    let (_, ast) = parse(s).map_err(|err| anyhow!("{err}"))?;
    let mut map = FxHashMap::default();
    explore(&FxHashSet::from_iter([(0, 0)]), &ast, &mut map);
    Ok(shortest_paths(&map))
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let paths = find_paths(s)?;
    let n = paths.values().copied().max().context("No paths found")?;
    Ok(n.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let paths = find_paths(s)?;
    let n = paths.values().filter(|&l| *l >= 1000).count();
    Ok(n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let ex = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(part_1(ex).unwrap().as_str(), "23");
    }

    #[test]
    fn test_explore() {
        assert_eq!(
            FxHashSet::from_iter([(0, -1)]),
            explore(
                &FxHashSet::from_iter([(0, 0)]),
                &Simple(N),
                &mut FxHashMap::default()
            )
        );
        assert_eq!(
            FxHashSet::from_iter([(1, -1)]),
            explore(
                &FxHashSet::from_iter([(0, 0)]),
                &Seq(vec![Simple(N), Simple(E)]),
                &mut FxHashMap::default()
            )
        );
        assert_eq!(
            FxHashSet::from_iter([(0, -1), (1, 0)]),
            explore(
                &FxHashSet::from_iter([(0, 0)]),
                &Branch(vec![Simple(N), Simple(E)]),
                &mut FxHashMap::default()
            )
        );
    }

    #[test]
    fn parse_test() {
        assert!(matches!(parse("^NWE$"), Ok(("", Seq(_)))));
    }

    #[test]
    fn parse_choice_test() {
        assert!(matches!(parse("^N(E|S|)$"), Ok(("", Seq(_)))));
    }

    #[test]
    fn test_parse_re_test() {
        let ex = "^ENWWW(NEEE|SSE(EE|N))$";
        let (_, r) = parse(ex).unwrap();
        if let Seq(next) = r {
            assert_eq!(next[0], Simple(E));
            assert_eq!(next[1], Simple(N));
            assert!(matches!(next[5], Branch(_)));
        } else {
            panic!("wtf")
        }
    }
}
