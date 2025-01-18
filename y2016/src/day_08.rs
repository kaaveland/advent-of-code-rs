use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

struct Display {
    rows: Vec<Vec<bool>>,
}

impl Display {
    fn new(height: usize, width: usize) -> Self {
        let mut rows = Vec::with_capacity(height);
        for _ in 0..height {
            rows.push(vec![false; width]);
        }
        Self { rows }
    }

    fn rect(&mut self, a: usize, b: usize) {
        for row in 0..b {
            for pos in 0..a {
                self.rows[row][pos] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, b: usize) {
        let mut new_row = vec![false; self.rows[0].len()];
        for ix in 0..self.rows[0].len() {
            let new_ix = (ix + b) % self.rows[0].len();
            new_row[new_ix] = self.rows[row][ix];
        }
        std::mem::swap(&mut new_row, &mut self.rows[row]);
    }

    fn rotate_column(&mut self, col: usize, b: usize) {
        let current_column: Vec<_> = self.rows.iter().map(|row| row[col]).collect();
        for ix in 0..self.rows.len() {
            let new_ix = (ix + b) % self.rows.len();
            self.rows[new_ix][col] = current_column[ix];
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

fn parse_one(s: &str) -> IResult<&str, Instruction> {
    fn posint(s: &str) -> IResult<&str, usize> {
        map_res(digit1, |n: &str| n.parse())(s)
    }

    fn parse_rect(s: &str) -> IResult<&str, Instruction> {
        let (s, (a, b)) = preceded(tag("rect "), separated_pair(posint, tag("x"), posint))(s)?;
        Ok((s, Instruction::Rect(a, b)))
    }
    fn parse_rotate_column(s: &str) -> IResult<&str, Instruction> {
        let (s, (a, b)) = preceded(
            tag("rotate column x="),
            separated_pair(posint, tag(" by "), posint),
        )(s)?;
        Ok((s, Instruction::RotateColumn(a, b)))
    }
    fn parse_rotate_row(s: &str) -> IResult<&str, Instruction> {
        let (s, (a, b)) = preceded(
            tag("rotate row y="),
            separated_pair(posint, tag(" by "), posint),
        )(s)?;
        Ok((s, Instruction::RotateRow(a, b)))
    }
    alt((parse_rect, parse_rotate_row, parse_rotate_column))(s)
}

fn parse(s: &str) -> anyhow::Result<Vec<Instruction>> {
    let (_, i) = separated_list1(tag("\n"), parse_one)(s).map_err(|err| anyhow!("{err}"))?;
    Ok(i)
}

fn draw_display(s: &str) -> anyhow::Result<Display> {
    let mut display = Display::new(6, 50);
    for i in parse(s)? {
        match i {
            Instruction::Rect(a, b) => {
                display.rect(a, b);
            }
            Instruction::RotateColumn(col, a) => {
                display.rotate_column(col, a);
            }
            Instruction::RotateRow(row, a) => {
                display.rotate_row(row, a);
            }
        }
    }
    Ok(display)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let n: u32 = draw_display(s)?
        .rows
        .into_iter()
        .flat_map(|row| row.into_iter().map(u32::from))
        .sum();
    Ok(n.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let mut lines: Vec<String> = Vec::with_capacity(6);
    for row in draw_display(s)?.rows {
        lines.push(
            row.into_iter()
                .map(|on| if on { '#' } else { ' ' })
                .collect(),
        );
    }
    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut display = Display::new(3, 8);
        display.rect(3, 2);
        assert_eq!(
            display.rows[0],
            vec![true, true, true, false, false, false, false, false]
        );
        display.rotate_column(1, 1);
        assert_eq!(
            display.rows[0],
            vec![true, false, true, false, false, false, false, false]
        );
        assert_eq!(
            display.rows[2],
            vec![false, true, false, false, false, false, false, false]
        );
    }
}
