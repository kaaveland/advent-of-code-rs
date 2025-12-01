use anyhow::{anyhow, Result};
use nom::character::complete::{char, digit1, hex_digit1, line_ending, one_of, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::str::FromStr;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Direction {
    R,
    D,
    L,
    U,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct Instruction<'a> {
    direction: Direction,
    count: i64,
    color: &'a str,
}

impl Direction {
    fn dxdy(self) -> (i64, i64) {
        use Direction::*;
        match self {
            D => (0, 1),
            U => (0, -1),
            L => (-1, 0),
            R => (1, 0),
        }
    }
}

fn add(lhs: (i64, i64), rhs: (i64, i64)) -> (i64, i64) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
fn mul(lhs: (i64, i64), rhs: (i64, i64)) -> (i64, i64) {
    (lhs.0 * rhs.0, lhs.1 * rhs.1)
}

type Polygon = Vec<(i64, i64)>;

fn parse_dir(s: &str) -> IResult<&str, Direction> {
    let (s, ch): (&str, char) = one_of("DULR")(s)?;
    match ch {
        'L' => Ok((s, Direction::L)),
        'R' => Ok((s, Direction::R)),
        'U' => Ok((s, Direction::U)),
        'D' => Ok((s, Direction::D)),
        _ => unreachable!(),
    }
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction<'_>> {
    let (s, dir) = parse_dir(s)?;
    let (s, count): (&str, i64) = preceded(space1, map_res(digit1, FromStr::from_str))(s)?;
    let (s, rgb) = preceded(
        space1,
        delimited(char('('), preceded(char('#'), hex_digit1), char(')')),
    )(s)?;
    Ok((
        s,
        Instruction {
            direction: dir,
            count,
            color: rgb,
        },
    ))
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction<'_>>> {
    Ok(separated_list1(line_ending, parse_instruction)(s)
        .map_err(|err| anyhow!("{err}"))?
        .1)
}

fn draw_polygon(instr: &[Instruction], use_hex: bool) -> Polygon {
    let mut polygon = Polygon::default();
    let mut pos = (0, 0);
    polygon.push(pos);
    for i in instr {
        let count = if use_hex {
            i64::from_str_radix(&i.color[..5], 16).unwrap()
        } else {
            i.count
        };
        let dir = if use_hex {
            match i.color.chars().last().unwrap() {
                '0' => Direction::R,
                '1' => Direction::D,
                '2' => Direction::L,
                '3' => Direction::U,
                _ => panic!("Unexpected digit: {}", i.color),
            }
        } else {
            i.direction
        };
        pos = add(pos, mul((count, count), dir.dxdy()));
        polygon.push(pos);
    }
    polygon
}

fn shoelace_area(polygon: &[(i64, i64)]) -> i64 {
    assert_eq!(polygon[0], polygon[polygon.len() - 1]);
    polygon
        .iter()
        .zip(polygon.iter().skip(1))
        .map(|(&(x1, y1), &(x2, y2))| x1 * y2 - x2 * y1)
        .sum::<i64>()
        / 2
}

fn inner_by_picks_theorem(polygon: &Polygon) -> i64 {
    let area = shoelace_area(polygon);
    area - circumference(polygon) / 2 + 1
}

fn circumference(polygon: &Polygon) -> i64 {
    polygon
        .iter()
        .zip(polygon.iter().skip(1))
        .map(|(&(x1, y1), &(x2, y2))| (x2 - x1).abs() + (y2 - y1).abs())
        .sum()
}

fn cubic_meters_of_lava(s: &str, use_hex: bool) -> Result<i64> {
    let i = parse_instructions(s)?;
    let p = draw_polygon(&i, use_hex);
    Ok(inner_by_picks_theorem(&p) + circumference(&p))
}

pub fn part_1(input: &str) -> Result<String> {
    Ok(cubic_meters_of_lava(input, false)?.to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    Ok(cubic_meters_of_lava(input, true)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_instr() {
        let (_, r) = parse_instruction("R 6 (#70c710)").unwrap();
        assert_eq!(
            r,
            Instruction {
                direction: Direction::R,
                count: 6,
                color: "70c710"
            }
        );
    }
    const EX: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_ex() {
        assert_eq!(cubic_meters_of_lava(EX, false).unwrap(), 62);
        assert_eq!(cubic_meters_of_lava(EX, true).unwrap(), 952408144115);
    }
}
