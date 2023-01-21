use anyhow::{Context, Result};
use itertools::Itertools;
use std::num::ParseIntError;

fn parse_comma_sep(line: &str) -> Result<Vec<u16>> {
    let r: Result<Vec<u16>, ParseIntError> = line.split(',').map(|n| n.parse()).collect();
    let unwrapped = r?;
    Ok(unwrapped)
}

fn parse_row(line: &&str) -> Result<Vec<Cell>> {
    let r: Result<Vec<u16>, ParseIntError> =
        line.split_ascii_whitespace().map(|n| n.parse()).collect();
    let unwrapped = r?;
    Ok(unwrapped.into_iter().map(Cell::Unmarked).collect())
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Cell {
    Marked(u16),
    Unmarked(u16),
}

impl Cell {
    fn is_marked(&self) -> bool {
        matches!(self, Cell::Marked(_))
    }
    fn is_unmarked(&self) -> bool {
        !self.is_marked()
    }
    fn mark(&mut self, n: u16) {
        if let Cell::Unmarked(num) = self {
            if n == *num {
                *self = Cell::Marked(*num)
            }
        }
    }
    fn content(&self) -> u16 {
        match self {
            Cell::Marked(num) => *num,
            Cell::Unmarked(num) => *num,
        }
    }
}

type Board = Vec<Vec<Cell>>;

fn parse(input: &str) -> Result<(Vec<u16>, Vec<Board>)> {
    let mut lines = input.lines();
    let mut boards = vec![];
    let seq = lines
        .next()
        .context("Empty input")
        .and_then(parse_comma_sep)?;

    let rest = lines.filter(|l| !l.is_empty()).collect_vec();
    let mut i = 0;
    while i < rest.len() {
        let board: Result<Vec<Vec<_>>> = rest[i..i + 5].iter().map(parse_row).collect();
        let add = board?;
        boards.push(add);
        i += 5;
    }

    Ok((seq, boards))
}

fn is_winning_board(board: &Board) -> bool {
    for row_colno in 0..5 {
        let row = &board[row_colno].iter().all(|c| c.is_marked());
        let col = (0..5)
            .map(|row| &board[row][row_colno])
            .all(|c| c.is_marked());
        if *row || col {
            return true;
        }
    }
    false
}

fn winning_board(boards: &[Board]) -> Option<&Board> {
    boards.iter().find(|b| is_winning_board(b))
}

#[allow(clippy::ptr_arg)] // clippy leads to many more type annotations in this case
fn solve_1(boards: &Vec<Board>, seq: &Vec<u16>) -> u32 {
    let mut boards = boards.clone();

    for &n in seq {
        mark_boards(n, &mut boards);
        if let Some(winner) = winning_board(&boards) {
            return score_board(n, winner);
        }
    }
    0
}

fn mark_boards(n: u16, boards: &mut Vec<Board>) {
    for board in boards {
        board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                cell.mark(n);
            })
        });
    }
}

fn score_board(last_drawn: u16, winner: &Board) -> u32 {
    let unmarked = winner
        .iter()
        .map(|row| {
            row.iter()
                .filter(|c| c.is_unmarked())
                .map(|c| c.content())
                .sum::<u16>()
        })
        .sum::<u16>();
    (unmarked as u32) * (last_drawn as u32)
}

#[allow(clippy::ptr_arg)] // clippy leads to many more type annotations in this case
fn solve_2(boards: &Vec<Board>, seq: &Vec<u16>) -> u32 {
    let mut boards = boards.clone();
    let mut scores = vec![];

    for &n in seq {
        mark_boards(n, &mut boards);
        let (winners, losers): (Vec<_>, Vec<_>) = boards.into_iter().partition(is_winning_board);
        boards = losers;
        scores.extend(winners.into_iter().map(|b| score_board(n, &b)))
    }
    *scores.last().unwrap_or(&0)
}

pub fn part_1(input: &str) -> Result<String> {
    let (seq, boards) = parse(input)?;
    let sol = solve_1(&boards, &seq);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let (seq, boards) = parse(input)?;
    let sol = solve_2(&boards, &seq);
    Ok(format!("{sol}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (seq, boards) = parse(EXAMPLE).unwrap();
        assert_eq!(seq[0..3], vec![7, 4, 9]);
        assert_eq!(seq[seq.len() - 1], 1);

        let b1 = &boards[0];
        assert_eq!(
            b1[0],
            vec![22, 13, 17, 11, 0]
                .into_iter()
                .map(Cell::Unmarked)
                .collect_vec()
        );
        assert_eq!(boards.len(), 3);
    }

    #[test]
    fn test_part_1() {
        let (seq, boards) = parse(EXAMPLE).unwrap();
        assert_eq!(solve_1(&boards, &seq), 4512);
    }

    #[test]
    fn test_part_2() {
        let (seq, boards) = parse(EXAMPLE).unwrap();
        assert_eq!(solve_2(&boards, &seq), 1924);
    }
    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
}
