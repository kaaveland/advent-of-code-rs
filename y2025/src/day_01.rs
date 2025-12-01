#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    L,
    R,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Move {
    dir: Dir,
    times: i16,
}

fn parse(lines: &str) -> Vec<Move> {
    lines
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            let dir = match l.chars().next() {
                Some('L') => Some(Dir::L),
                Some('R') => Some(Dir::R),
                _ => None,
            }?;
            let times = l[1..].parse().ok()?;
            Some(Move { dir, times })
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Position {
    dial: i16,
}

fn scan_positions(
    start: i16,
    instructions: impl Iterator<Item = Move>,
) -> impl Iterator<Item = Position> {
    instructions.scan(start, |prev_dial, m| {
        let sig = if matches!(m.dir, Dir::L) { -1 } else { 1 };
        let next = *prev_dial + sig * m.times;
        let dial = next.rem_euclid(100);
        *prev_dial = dial;
        Some(Position { dial })
    })
}

fn count_zeros(input: &str) -> usize {
    let moves = parse(input);
    scan_positions(50, moves.into_iter())
        .filter(|p| p.dial == 0)
        .count()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", count_zeros(s)))
}

fn count_passing_zeros(input: &str) -> usize {
    let moves = parse(input);
    scan_positions(
        50,
        moves.into_iter().flat_map(|m| {
            std::iter::repeat_n(
                Move {
                    dir: m.dir,
                    times: 1,
                },
                m.times as usize,
            )
        }),
    )
    .filter(|p| p.dial == 0)
    .count()
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", count_passing_zeros(s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_parse() {
        let moves = parse(EX);
        assert_eq!(
            moves[0],
            Move {
                dir: Dir::L,
                times: 68
            }
        );
        assert_eq!(
            moves[4],
            Move {
                dir: Dir::R,
                times: 60
            }
        );
    }

    #[test]
    fn test_ex_1() {
        assert_eq!(3, count_zeros(EX));
    }

    #[test]
    fn test_ex_2() {
        assert_eq!(6, count_passing_zeros(EX));
    }
}
