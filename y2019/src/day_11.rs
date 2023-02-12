use crate::intcode::{Output, Program};
use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;

type Panel = [i32; 2];
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Paint {
    Black,
    White,
}
type Hull = HashMap<Panel, Paint>;
impl From<Paint> for i64 {
    fn from(value: Paint) -> Self {
        match value {
            Paint::Black => 0,
            Paint::White => 1,
        }
    }
}
const HEADINGS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn paint_hull(program: &Program, initial: Paint) -> Result<Hull> {
    use Paint::*;
    let mut prog = program.clone();
    let mut hull = Hull::default();
    let mut heading = 0;
    let mut loc = [0, 0];
    let mut first = true;

    loop {
        let here = hull.get(&loc).copied().unwrap_or(Black);
        if first {
            prog.input(initial.into());
            first = false;
        } else {
            prog.input(here.into());
        }
        let paint_no = prog.produce_output()?;
        if let Output::Value(paint_no) = paint_no {
            let paint = match paint_no {
                0 => Ok(Black),
                1 => Ok(White),
                _ => Err(anyhow!("Unexpected output: {paint_no}")),
            }?;
            hull.insert(loc, paint);
            if let Output::Value(mut dir_change) = prog.produce_output()? {
                if dir_change == 0 {
                    dir_change = -1;
                }
                heading += dir_change;
                let [dx, dy] = HEADINGS[heading.rem_euclid(HEADINGS.len() as i64) as usize];
                loc[0] += dx;
                loc[1] += dy;
            } else {
                return Ok(hull);
            }
        } else {
            return Ok(hull);
        }
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Empty input")?)?;
    let hull = paint_hull(&prog, Paint::Black)?;
    Ok(format!("{}", hull.len()))
}

pub fn part_2(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Empty input")?)?;
    let hull = paint_hull(&prog, Paint::White)?;
    let ymax = hull.keys().map(|c| c[1]).max().context("No paint")?;
    let ymin = hull.keys().map(|c| c[1]).min().context("No paint")?;
    let xmax = hull.keys().map(|c| c[0]).max().context("No paint")?;
    let xmin = hull.keys().map(|c| c[0]).min().context("No paint")?;
    let ylen = (ymax - ymin + 1) as usize;
    let xlen = (xmax - xmin + 1) as usize;
    let mut display = vec![vec![' '; xlen]; ylen];
    for ([x, y], paint) in hull.into_iter() {
        let x = (x - xmin) as usize;
        let y = y.unsigned_abs() as usize;
        let ch = match paint {
            Paint::White => '#',
            Paint::Black => ' ',
        };
        display[y][x] = ch;
    }
    let display: Vec<String> = display
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect_vec();
    Ok(display.join("\n"))
}
