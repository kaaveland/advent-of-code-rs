use anyhow::{anyhow, Result};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use PointSpec::*;
use Tile::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
enum PointSpec {
    One(u32),
    Many(u32, u32),
}

impl PointSpec {
    fn start(&self) -> u32 {
        match self {
            One(start) => *start,
            Many(start, _) => *start,
        }
    }
    fn stop(&self) -> u32 {
        match self {
            One(start) => *start,
            Many(_, start) => *start,
        }
    }
    fn iter(&self) -> impl Iterator<Item = u32> {
        self.start()..=self.stop()
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
struct Input {
    x: PointSpec,
    y: PointSpec,
}

fn posint(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |n: &str| n.parse::<u32>())(s)
}

fn parse_pointspec(s: &str) -> IResult<&str, PointSpec> {
    let single = map(posint, One);
    let many = map(separated_pair(posint, tag(".."), posint), |(s, e)| {
        Many(s, e)
    });
    alt((many, single))(s)
}

fn parse_input(s: &str) -> IResult<&str, Input> {
    fn parse_x(s: &str) -> IResult<&str, PointSpec> {
        preceded(tag("x="), parse_pointspec)(s)
    }
    fn parse_y(s: &str) -> IResult<&str, PointSpec> {
        preceded(tag("y="), parse_pointspec)(s)
    }
    fn y_first(s: &str) -> IResult<&str, Input> {
        map(separated_pair(parse_y, tag(", "), parse_x), |(y, x)| {
            Input { x, y }
        })(s)
    }
    fn x_first(s: &str) -> IResult<&str, Input> {
        map(separated_pair(parse_x, tag(", "), parse_y), |(x, y)| {
            Input { x, y }
        })(s)
    }
    alt((x_first, y_first))(s)
}

fn parse(s: &str) -> Result<Vec<Input>> {
    let r = separated_list1(tag("\n"), parse_input)(s).map_err(|err| anyhow!("{err}"));
    Ok(r?.1)
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
enum Tile {
    Unknown,
    Moving,
    Settled,
    Clay,
}

struct State {
    moving: u32,
    still: u32,
    tiles: Vec<Tile>,
    xmin: u32,
    width: u32,
    ymin: u32,
    ymax: u32,
}

impl State {
    fn try_new(s: &str) -> Result<State> {
        let inputs = parse(s)?;
        let (mut xmin, mut xmax) = inputs.iter().fold((u32::MAX, 0), |(xmin, xmax), p| {
            (p.x.start().min(xmin), xmax.max(p.x.stop()))
        });
        // Leave some space in case there's clay at the edge
        xmin -= 3;
        xmax += 3;
        let (ymin, ymax) = inputs.iter().fold((u32::MAX, 0), |(xmin, xmax), p| {
            (p.y.start().min(xmin), xmax.max(p.y.stop()))
        });
        let width = xmax + 1 - xmin;
        let height = ymax + 1 - ymin;
        let mut tiles = vec![Unknown; (width * height) as usize];
        for (x, y) in inputs.iter().flat_map(|points| {
            points
                .x
                .iter()
                .flat_map(|x| points.y.iter().map(move |y| (x, y)))
        }) {
            tiles[((y - ymin) * width + (x - xmin)) as usize] = Clay;
        }
        Ok(State {
            moving: 0,
            still: 0,
            xmin,
            width,
            ymin,
            ymax,
            tiles,
        })
    }

    #[inline]
    fn translate(&self, x: u32, y: u32) -> usize {
        ((y - self.ymin) * self.width + (x - self.xmin)) as usize
    }

    #[inline]
    fn set(&mut self, x: u32, y: u32, tile: Tile) {
        let loc = self.translate(x, y);
        self.tiles[loc] = tile;
        if (self.ymin..=self.ymax).contains(&y) {
            if tile == Moving {
                self.moving += 1;
            } else if tile == Settled {
                self.still += 1;
            }
        }
    }

    fn tile(&mut self, x: u32, y: u32) -> Tile {
        if y > self.ymax {
            Moving
        } else if y < self.ymin {
            self.tile(x, y + 1)
        } else {
            let loc = self.translate(x, y);
            if self.tiles[loc] != Unknown {
                self.tiles[loc]
            } else if self.tile(x, y + 1) == Moving {
                self.set(x, y, Moving);
                Moving
            } else if [Clay, Settled].contains(&self.tile(x, y + 1)) {
                // We should flow down the sides now, and figure out what is to our left and right
                let mut left = x;
                let mut right = x;
                // Write what's under us
                while self.tiles[self.translate(left - 1, y)] == Unknown
                    && [Clay, Settled].contains(&self.tile(left, y + 1))
                {
                    left -= 1;
                }
                while self.tiles[self.translate(right + 1, y)] == Unknown
                    && [Clay, Settled].contains(&self.tile(right, y + 1))
                {
                    right += 1;
                }

                // If the left and right are both clay, we're also settled,
                // otherwise, we're moving
                let settle = self.tiles[self.translate(right + 1, y)] == Clay
                    && self.tiles[self.translate(left - 1, y)] == Clay;

                for xt in left..=right {
                    self.set(xt, y, if settle { Settled } else { Moving });
                }

                if settle {
                    Settled
                } else {
                    Moving
                }
            } else {
                panic!("{x}, {y} has {:?} below", self.tile(x, y + 1));
            }
        }
    }
}

pub fn part_1(s: &str) -> Result<String> {
    let mut state = State::try_new(s)?;
    state.tile(500, 0);
    Ok(format!("{}", state.still + state.moving))
}

pub fn part_2(s: &str) -> Result<String> {
    let mut state = State::try_new(s)?;
    state.tile(500, 0);
    Ok(format!("{}", state.still))
}
#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";
    #[test]
    fn test_p1() -> Result<()> {
        assert_eq!(part_1(EX)?.as_str(), "57");
        Ok(())
    }

    #[test]
    fn parse_x_first() -> Result<()> {
        let r = parse("x=495, y=2..7")?;
        assert_eq!(
            r,
            vec![Input {
                x: One(495),
                y: Many(2, 7)
            }]
        );
        Ok(())
    }

    #[test]
    fn parse_y_first() -> Result<()> {
        let r = parse("y=7, x=495..501")?;
        assert_eq!(
            r,
            vec![Input {
                x: Many(495, 501),
                y: One(7)
            }]
        );
        Ok(())
    }
}
