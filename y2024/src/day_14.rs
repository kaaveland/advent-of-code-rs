use fxhash::FxHashSet;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Bot {
    p_x: i32,
    v_x: i32,
    p_y: i32,
    v_y: i32,
}

fn posint(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn int(input: &str) -> IResult<&str, i32> {
    alt((map(preceded(char('-'), posint), |n| -n), posint))(input)
}

fn parse_bot(input: &str) -> IResult<&str, Bot> {
    let (input, (p_x, p_y)) = preceded(tag("p="), separated_pair(int, char(','), int))(input)?;
    let (input, (v_x, v_y)) = preceded(tag(" v="), separated_pair(int, char(','), int))(input)?;
    Ok((input, Bot { p_x, p_y, v_x, v_y }))
}

fn parse(input: &str) -> anyhow::Result<Vec<Bot>> {
    separated_list1(char('\n'), parse_bot)(input)
        .map_err(|e| anyhow::anyhow!("{}", e))
        .map(|(_, bots)| bots)
}

fn bot_positions(
    bots: &[Bot],
    time: i32,
    height: i32,
    width: i32,
) -> impl Iterator<Item = (i32, i32)> + '_ {
    bots.iter().map(move |bot| {
        (
            (bot.p_x + time * bot.v_x).rem_euclid(width),
            (bot.p_y + time * bot.v_y).rem_euclid(height),
        )
    })
}

fn quadrants(pos: &[(i32, i32)], height: i32, width: i32) -> [i32; 4] {
    assert_eq!(height.rem_euclid(2), 1);
    assert_eq!(width.rem_euclid(2), 1);
    let xmid = width / 2;
    let ymid = height / 2;
    let mut counts = [0; 4];
    for (x, y) in pos {
        if *x == xmid || *y == ymid {
            continue;
        } else {
            let qx = *x / (xmid + 1);
            let qy = *y / (ymid + 1);
            counts[(qy * 2 + qx) as usize] += 1;
        }
    }
    counts
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let bots = parse(input)?;
    let pos = bot_positions(&bots, 100, 103, 101).collect_vec();
    let counts = quadrants(&pos, 103, 101);
    let score: i32 = counts.iter().product();
    Ok(format!("{score}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let bots = parse(input)?;
    for time in 0.. {
        let pos: FxHashSet<_> = bot_positions(&bots, time, 103, 101).collect();
        if pos.len() == bots.len() {
            return Ok(format!("{time}"));
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_bot_positions() {
        let bots = [Bot {
            p_x: 2,
            p_y: 4,
            v_x: 2,
            v_y: -3,
        }];
        let positions = bot_positions(&bots, 5, 7, 11).collect::<Vec<_>>();
        assert_eq!(positions, vec![(1, 3)]);
    }

    #[test]
    fn test_parse() {
        let bots = parse(EXAMPLE).unwrap();
        assert_eq!(bots.len(), 12);
        assert_eq!(
            bots[0],
            Bot {
                p_x: 0,
                p_y: 4,
                v_x: 3,
                v_y: -3
            }
        );
        assert_eq!(
            bots[11],
            Bot {
                p_x: 9,
                p_y: 5,
                v_x: -3,
                v_y: -3
            }
        );
    }

    #[test]
    fn test_quadrants() {
        let bots = parse(EXAMPLE).unwrap();
        let pos = bot_positions(&bots, 100, 7, 11).collect::<Vec<_>>();
        let counts = quadrants(&pos, 7, 11);
        let safety_score: i32 = counts.iter().product();
        assert_eq!(safety_score, 12);
    }
}
