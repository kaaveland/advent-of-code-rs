use anyhow::{anyhow, Context, Result};

#[cfg(test)]
pub mod tests {
    use super::Move::*;
    use super::*;
    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE).unwrap(),
            vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2),]
        )
    }

    #[test]
    fn test_solve_1() {
        let ex = parse(EXAMPLE).unwrap();
        assert_eq!(solve_1(&ex), 150);
    }

    #[test]
    fn test_solve_2() {
        let ex = parse(EXAMPLE).unwrap();
        assert_eq!(solve_2(&ex), 900);
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Move {
    Down(i64),
    Up(i64),
    Forward(i64),
}

fn parse_line(line: &str) -> Result<Move> {
    use Move::*;
    let parts = line.split_once(' ').context("Bad formatting")?;
    let num = parts.1.parse::<i64>()?;
    match parts.0 {
        "forward" => Ok(Forward(num)),
        "down" => Ok(Down(num)),
        "up" => Ok(Up(num)),
        _ => Err(anyhow!("Unknown instruction: {}", parts.0)),
    }
}

fn parse(inp: &str) -> Result<Vec<Move>> {
    inp.lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

fn solve_1(moves: &[Move]) -> i64 {
    use Move::*;
    let (depth, hor_pos) = moves.iter().fold((0, 0), |(depth, hor_pos), mv| match mv {
        Down(d) => (depth + d, hor_pos),
        Up(d) => (depth - d, hor_pos),
        Forward(f) => (depth, hor_pos + f),
    });
    depth * hor_pos
}

fn solve_2(moves: &[Move]) -> i64 {
    use Move::*;
    let (depth, hor_pos, _) = moves
        .iter()
        .fold((0, 0, 0), |(depth, hor_pos, aim), mv| match mv {
            Down(d) => (depth, hor_pos, aim + d),
            Up(d) => (depth, hor_pos, aim - d),
            Forward(steps) => (depth + aim * steps, hor_pos + steps, aim),
        });
    depth * hor_pos
}

pub fn part_1(input: &str) -> Result<String> {
    let moves = parse(input)?;
    let sol = solve_1(&moves);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let moves = parse(input)?;
    let sol = solve_2(&moves);
    Ok(format!("{sol}"))
}
