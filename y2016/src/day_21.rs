use anyhow::anyhow;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

fn posint(s: &str) -> IResult<&str, usize> {
    map_res(digit1, |n: &str| n.parse())(s)
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    /// swap position X with position Y
    SwapPos(usize, usize),
    /// swap letter X with letter Y
    Swap(char, char),
    /// rotate left X steps
    RotLeft(usize),
    /// rotate right X steps
    RotRight(usize),
    /// rotate based on position of letter X
    RotRightPos(char),
    /// reverse positions X through Y
    Reverse(usize, usize),
    /// move position X to position Y
    Move(usize, usize),
}
type Prog = Vec<Op>;

fn parse(s: &str) -> anyhow::Result<Prog> {
    separated_list1(tag("\n"), parse_op)(s)
        .map_err(|e| anyhow!("{e}"))
        .map(|(_, ops)| ops)
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    alt((
        parse_move,
        parse_reverse,
        parse_rot_right_pos,
        parse_rot_right,
        parse_rot_left,
        parse_swap,
        parse_swap_pos,
    ))(s)
}

fn parse_move(s: &str) -> IResult<&str, Op> {
    let (s, _) = tag("move position ")(s)?;
    let (s, x) = posint(s)?;
    let (s, _) = tag(" to position ")(s)?;
    let (s, y) = posint(s)?;
    Ok((s, Op::Move(x, y)))
}

fn parse_reverse(s: &str) -> IResult<&str, Op> {
    let (s, _) = tag("reverse positions ")(s)?;
    let (s, x) = posint(s)?;
    let (s, _) = tag(" through ")(s)?;
    let (s, y) = posint(s)?;
    Ok((s, Op::Reverse(x, y)))
}

fn parse_rot_right_pos(s: &str) -> IResult<&str, Op> {
    let (s, _) = tag("rotate based on position of letter ")(s)?;
    let (s, ch) = anychar(s)?;
    Ok((s, Op::RotRightPos(ch)))
}

fn parse_rot_right(s: &str) -> IResult<&str, Op> {
    let (s, _) = tag("rotate right ")(s)?;
    let (s, x) = posint(s)?;
    let (s, _) = tag(" step")(s)?;
    if x != 1 {
        let (s, _) = anychar(s)?;
        Ok((s, Op::RotRight(x)))
    } else {
        Ok((s, Op::RotRight(x)))
    }
}

fn parse_rot_left(s: &str) -> IResult<&str, Op> {
    let (s, _) = tag("rotate left ")(s)?;
    let (s, x) = posint(s)?;
    let (s, _) = tag(" step")(s)?;
    if x != 1 {
        let (s, _) = anychar(s)?;
        Ok((s, Op::RotLeft(x)))
    } else {
        Ok((s, Op::RotLeft(x)))
    }
}

fn parse_swap(s: &str) -> IResult<&str, Op> {
    let (s, _) = tag("swap letter ")(s)?;
    let (s, x) = anychar(s)?;
    let (s, _) = tag(" with letter ")(s)?;
    let (s, y) = anychar(s)?;
    Ok((s, Op::Swap(x, y)))
}

fn parse_swap_pos(s: &str) -> IResult<&str, Op> {
    let (s, _) = tag("swap position ")(s)?;
    let (s, x) = posint(s)?;
    let (s, _) = tag(" with position ")(s)?;
    let (s, y) = posint(s)?;
    Ok((s, Op::SwapPos(x, y)))
}

impl Op {
    fn exec(&self, s: &mut [char]) {
        match self {
            Op::SwapPos(x, y) => {
                s.swap(*x, *y);
            }
            Op::Swap(x, y) => s.iter_mut().for_each(|ch| {
                if *ch == *x {
                    *ch = *y;
                } else if *ch == *y {
                    *ch = *x;
                }
            }),
            Op::RotLeft(x) => s.rotate_left(*x % s.len()),
            Op::RotRight(x) => s.rotate_right(*x % s.len()),
            Op::RotRightPos(ch) => {
                let i = s
                    .iter()
                    .enumerate()
                    .find(|(_, c)| *c == ch)
                    .map(|(i, _)| i)
                    .unwrap();
                let rots = 1 + i + if i >= 4 { 1 } else { 0 };
                let rots = rots % s.len();
                s.rotate_right(rots);
            }
            Op::Reverse(x, y) => {
                s[*x..=*y].reverse();
            }
            Op::Move(x, y) => {
                if *x < *y {
                    s[*x..=*y].rotate_left(1);
                } else {
                    s[*y..=*x].rotate_right(1);
                }
            }
        }
    }

    fn exec_inv(&self, s: &mut [char]) {
        match self {
            // These are their own inverse
            op @ Op::SwapPos(_, _) => op.exec(s),
            op @ Op::Swap(_, _) => op.exec(s),
            op @ Op::Reverse(_, _) => op.exec(s),
            // Trivially invertible
            Op::Move(x, y) => Op::Move(*y, *x).exec(s),
            Op::RotLeft(x) => Op::RotRight(*x).exec(s),
            Op::RotRight(x) => Op::RotLeft(*x).exec(s),
            // Hard to think of how to invert this, but for n=5 and n=8
            // it's safe to just generate all rotations of the input string and pick the one
            // that results in the provided output
            op @ Op::RotRightPos(_) => {
                let mut v = vec![' '; s.len()];
                let p = (0..=s.len())
                    .map(|n| {
                        let mut v = Vec::with_capacity(s.len());
                        v.extend(s.iter().copied());
                        v.rotate_left(n);
                        v
                    })
                    .find(|p| {
                        v.clear();
                        v.extend(p.iter().copied());
                        op.exec(&mut v);
                        v == s
                    })
                    .unwrap();
                s.copy_from_slice(&p[..s.len()]);
            }
        }
    }
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let prog = parse(s)?;
    assert_eq!(s.lines().count(), prog.len());
    let mut pw = "abcdefgh".chars().collect_vec();
    prog.iter().for_each(|op| op.exec(&mut pw));
    Ok(pw.iter().join(""))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let mut prog = parse(s)?;
    assert_eq!(s.lines().count(), prog.len());
    prog.reverse();
    let mut pw = "fbgdceah".chars().collect_vec();
    prog.iter().for_each(|op| op.exec_inv(&mut pw));
    Ok(pw.iter().join(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    const RECIPE: &str = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn test_example() {
        let mut pw = "abcde".chars().collect_vec();
        let prog = parse(RECIPE).unwrap();
        prog[0].exec(&mut pw);
        assert_eq!("ebcda".chars().collect_vec(), pw);
        prog[1].exec(&mut pw);
        assert_eq!("edcba".chars().collect_vec(), pw);
        prog[2].exec(&mut pw);
        assert_eq!("abcde".chars().collect_vec(), pw);
        prog[3].exec(&mut pw);
        assert_eq!("bcdea".chars().collect_vec(), pw);
        prog[4].exec(&mut pw);
        assert_eq!("bdeac".chars().collect_vec(), pw);
        prog[5].exec(&mut pw);
        assert_eq!("abdec".chars().collect_vec(), pw);
        prog[6].exec(&mut pw);
        assert_eq!("ecabd".chars().collect_vec(), pw);
        prog[7].exec(&mut pw);
        assert_eq!("decab".chars().collect_vec(), pw);
    }

    #[test]
    fn test_example_inv() {
        let prog = parse(RECIPE).unwrap();
        let mut pw = "decab".chars().collect_vec();
        prog[7].exec_inv(&mut pw);
        assert_eq!("ecabd".chars().collect_vec(), pw);
        prog[6].exec_inv(&mut pw);
        assert_eq!("abdec".chars().collect_vec(), pw);
        prog[5].exec_inv(&mut pw);
        assert_eq!("bdeac".chars().collect_vec(), pw);
        prog[4].exec_inv(&mut pw);
        assert_eq!("bcdea".chars().collect_vec(), pw);
        prog[3].exec_inv(&mut pw);
        assert_eq!("abcde".chars().collect_vec(), pw);
        prog[2].exec_inv(&mut pw);
        assert_eq!("edcba".chars().collect_vec(), pw);
        prog[1].exec_inv(&mut pw);
        assert_eq!("ebcda".chars().collect_vec(), pw);
        prog[0].exec_inv(&mut pw);
        assert_eq!("abcde".chars().collect_vec(), pw);
    }

    #[test]
    fn test_parse_swap() {
        let p = parse_swap_pos("swap position 7 with position 3");
        assert!(p.is_ok())
    }
}
