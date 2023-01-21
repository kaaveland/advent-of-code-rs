use crate::point_2d::Point2d;
use anyhow::{anyhow, Context, Result};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use std::cmp::max;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn vec(&self) -> Point2d<i32> {
        match self {
            Direction::North => Point2d::origin().north(),
            Direction::West => Point2d::origin().west(),
            Direction::South => Point2d::origin().south(),
            Direction::East => Point2d::origin().east(),
        }
    }
}
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Blizzard {
    pos: Point2d<i32>,
    dir: Direction,
}
impl Blizzard {
    fn new(dir: Direction, pos: Point2d<i32>) -> Self {
        Blizzard { pos, dir }
    }
}
#[derive(Eq, PartialEq, Hash, Debug)]
struct Board {
    walls: Vec<Point2d<i32>>,
    blizzards: Vec<Blizzard>,
    start: Point2d<i32>,
    goal: Point2d<i32>,
    bounds: (i32, i32),
}

fn parse_board(input: &str) -> Result<Board> {
    use Direction::*;
    let mut blizzards = Vec::new();
    let mut walls = Vec::new();
    let mut start = None;
    let mut goal = None;

    for (y, row) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let point = Point2d::new(x as i32, y as i32);
            match ch {
                '.' if start.is_some() => goal = Some(point),
                '.' => start = Some(point),
                '<' => blizzards.push(Blizzard::new(West, point)),
                '>' => blizzards.push(Blizzard::new(East, point)),
                '^' => blizzards.push(Blizzard::new(North, point)),
                'v' => blizzards.push(Blizzard::new(South, point)),
                '#' => {
                    walls.push(point);
                }
                _ => panic!("Unexpected"),
            }
        }
    }

    let start = start.context("Unable to locate start!")?;
    let goal = goal.context("Unable to locate goal!")?;
    let (xmax, ymax) = walls.iter().fold((0, 0), |(xmax, ymax), point| {
        (max(xmax, point.x + 1), max(ymax, point.y + 1))
    });

    Ok(Board {
        walls,
        blizzards,
        start,
        goal,
        bounds: (xmax, ymax),
    })
}

fn move_blizzard_out_of_walls(
    blizzard: &Blizzard,
    walls: &HashSet<&Point2d<i32>>,
    bounds: (i32, i32),
) -> Blizzard {
    let mut blizz = *blizzard;
    let (xmax, ymax) = bounds;
    blizz.pos = (blizz.pos + blizz.dir.vec()).wrap(xmax, ymax);
    while walls.contains(&blizz.pos) {
        blizz.pos = (blizz.pos + blizz.dir.vec()).wrap(xmax, ymax);
    }
    blizz
}

fn bfs_to_goal(board: &Board, inital_time: i32) -> Option<i32> {
    let mut queue = VecDeque::from([(inital_time, board.start)]);
    let (xmax, ymax) = board.bounds;
    let mut blizzards = board.blizzards.iter().cloned().collect_vec();
    let mut blizzpos: HashSet<_> = blizzards.iter().map(|blizz| &blizz.pos).cloned().collect();
    let walls: HashSet<_> = board.walls.iter().collect();
    let mut currtime = 0;
    let mut visited = HashSet::default();
    visited.insert((0, Point2d { x: 0, y: 0 }));

    let xbound = 0..xmax;
    let ybound = 0..ymax;

    while let Some((time, place)) = queue.pop_front() {
        let reposition = currtime < time;
        while currtime < time {
            currtime += 1;
            blizzards = blizzards
                .into_iter()
                .map(|blizz| move_blizzard_out_of_walls(&blizz, &walls, (xmax, ymax)))
                .collect();
        }
        if reposition {
            blizzpos = blizzards.iter().map(|blizz| blizz.pos).collect();
        }
        if place == board.goal {
            return Some(time - 1);
        }
        let options = [
            place.north(),
            place.west(),
            place.south(),
            place.east(),
            place,
        ]
        .into_iter()
        .filter(|opt| {
            !blizzpos.contains(opt)
                && !walls.contains(opt)
                && xbound.contains(&opt.x)
                && ybound.contains(&opt.y)
        });

        for point in options {
            if !visited.contains(&(time + 1, point)) {
                queue.push_back((time + 1, point));
                visited.insert((time + 1, point));
            }
        }
    }
    None
}

fn bfs_roundtrip(inp: &str) -> Option<(i32, i32)> {
    let mut prob = parse_board(inp).ok()?;
    let steps = bfs_to_goal(&prob, 0)?;
    let start = prob.start;
    let goal = prob.goal;
    prob.goal = start;
    prob.start = goal;
    let next_time = bfs_to_goal(&prob, steps)?;
    prob.goal = goal;
    prob.start = start;
    let last_time = bfs_to_goal(&prob, next_time)?;
    Some((steps, last_time))
}

pub fn part_1(input: &str) -> Result<String> {
    let prob = parse_board(input)?;
    let steps = bfs_to_goal(&prob, 0).with_context(|| anyhow!("Unable to solve"))?;
    Ok(format!("{steps}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let steps = bfs_roundtrip(input).with_context(|| anyhow!("Unable to solve"))?;
    Ok(format!("{}", steps.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
    #[test]
    fn test_parse() {
        let board = parse_board(EXAMPLE).unwrap();
        let (xmax, ymax) = board.bounds;
        assert_eq!(xmax, 8);
        assert_eq!(ymax, 6);
    }

    #[test]
    fn test_bfs() {
        let board = parse_board(EXAMPLE).unwrap();
        let steps = bfs_to_goal(&board, 0);
        assert_eq!(steps, Some(18));
    }

    #[test]
    fn test_bfs_part2() {
        assert_eq!(bfs_roundtrip(EXAMPLE), Some((18, 54)));
    }
}
