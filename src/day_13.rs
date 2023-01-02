use anyhow::{Context, Result};
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Fold {
    Row(i64),
    Col(i64),
}

type Input = (HashSet<(i64, i64)>, Vec<Fold>);
fn parse(input: &str) -> Result<Input> {
    let (points, folds) = input
        .split_once("\n\n")
        .context("Missing delim: 2 blanks")?;
    let points: Result<HashSet<_>> = points
        .lines()
        .map(|p| {
            let (x, y) = p.split_once(',').context("Missing delim: ,")?;
            let x: i64 = x.parse()?;
            let y: i64 = y.parse()?;
            Ok((x, y))
        })
        .collect();
    let points = points?;
    let fold_re = Regex::new(r"fold along ([xy])=(-?[0-9]+)")?;
    let folds: Result<Vec<_>> = folds
        .lines()
        .filter(|l| !l.is_empty())
        .map(|f| {
            let caps = fold_re.captures(f).context("No match")?;
            let xy = caps.get(1).unwrap().as_str();
            let coord = caps.get(2).unwrap().as_str();
            let coord: i64 = coord.parse()?;
            if xy == "x" {
                Ok(Fold::Col(coord))
            } else {
                Ok(Fold::Row(coord))
            }
        })
        .collect();
    let folds = folds?;
    Ok((points, folds))
}

fn row_fold(row: i64, (x, y): (i64, i64)) -> (i64, i64) {
    assert_ne!(row, y);
    if y > row {
        (x, y - row)
    } else {
        (x, row - y)
    }
}
fn col_fold(col: i64, (x, y): (i64, i64)) -> (i64, i64) {
    assert_ne!(col, x);
    if col > x {
        (col - x, y)
    } else {
        (x - col, y)
    }
}

fn folded_locations(
    points: &HashSet<(i64, i64)>,
    fold: &Fold,
    xmin: i64,
    ymin: i64,
) -> HashSet<(i64, i64)> {
    points
        .iter()
        .map(|(x, y)| match fold {
            Fold::Row(row) => row_fold(*row, (*x - xmin, *y - ymin)),
            Fold::Col(col) => col_fold(*col, (*x - xmin, *y - ymin)),
        })
        .collect()
}

fn solve_1(input: &Input) -> usize {
    folded_locations(&input.0, &input.1[0], 0, 0).len()
}

pub fn part_1(input: &str) -> Result<String> {
    let input = parse(input)?;
    let sol = solve_1(&input);
    Ok(format!("{sol}"))
}

fn bounds(hs: &HashSet<(i64, i64)>) -> (i64, i64, i64, i64) {
    hs.iter()
        .fold((1, 1, 1, 1), |(xmin, xmax, ymin, ymax), (x, y)| {
            (min(*x, xmin), max(*x, xmax), min(*y, ymin), max(*y, ymax))
        })
}

pub fn part_2(input: &str) -> Result<String> {
    let input = parse(input)?;
    let mut hs = input.0;
    let (mut xmin, mut xmax, mut ymin, mut ymax) = bounds(&hs);
    for fold in input.1.iter() {
        hs = folded_locations(&hs, fold, xmin, ymin);
        (xmin, xmax, ymin, ymax) = bounds(&hs);
    }

    let mut out = String::new();

    for row in (ymin..=ymax).rev() {
        for col in (xmin..=xmax).rev() {
            if hs.contains(&(col, row)) {
                out += "#";
            } else {
                out += " ";
            }
        }
        out += "\n";
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let input = parse(EXAMPLE).unwrap();
        let sol = solve_1(&input);
        assert_eq!(sol, 17);
    }
    #[test]
    fn test_parse() {
        let input = parse(EXAMPLE).unwrap();
        assert_eq!(input.1.len(), 2);
        assert_eq!(input.0.len(), 18);
    }

    #[test]
    fn test_part_2() {
        let s = part_2(EXAMPLE).unwrap();
        println!("{s}");
    }

    const EXAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
}
