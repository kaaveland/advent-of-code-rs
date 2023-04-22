use crate::intcode::ParameterMode::Immediate;
use crate::intcode::Program;
use anyhow::{anyhow, Context, Result};
use fxhash::FxHashSet as HashSet;
use std::fmt::Debug;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Robot {
    pos: (i32, i32),
    orientation: Orientation,
}
type Scaffold = HashSet<(i32, i32)>;

fn initial_state(input: &str) -> Result<(Scaffold, Robot)> {
    let mut prog = Program::parse(input.lines().next().context("Empty input")?)?;
    prog.exec()?;
    let useful = "#<>^v";
    fn to_robot(pos: (i32, i32), ch: char) -> Option<Robot> {
        use Orientation::*;
        match ch {
            '^' => Some(Robot {
                pos,
                orientation: North,
            }),
            'v' => Some(Robot {
                pos,
                orientation: South,
            }),
            '<' => Some(Robot {
                pos,
                orientation: West,
            }),
            '>' => Some(Robot {
                pos,
                orientation: East,
            }),
            _ => None,
        }
    }

    let s: String = prog
        .output()
        .iter()
        .copied()
        .map(|i| (i as u8) as char)
        .collect();

    let robot = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| to_robot((x as i32, y as i32), ch))
        })
        .next()
        .context("Robot not found")?;

    let scaffold: HashSet<(i32, i32)> = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| useful.contains(*ch))
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    Ok((scaffold, robot))
}

pub fn part_1(input: &str) -> Result<String> {
    let (scaffold, _) = initial_state(input)?;

    let intersections = scaffold.iter().filter(|(x, y)| {
        let (x, y) = (*x, *y);
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .all(|n| scaffold.contains(n))
    });
    let n: i32 = intersections.copied().map(|(x, y)| x * y).sum();
    Ok(format!("{n}"))
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Rotate {
    Left,
    Right,
}
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Action {
    rotate: Rotate,
    forwards: usize,
}

impl Orientation {
    fn left(self) -> Orientation {
        use Orientation::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    fn right(self) -> Orientation {
        use Orientation::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn forward(self, pos: (i32, i32)) -> (i32, i32) {
        use Orientation::*;
        let (x, y) = pos;
        match self {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        }
    }
}

impl Robot {
    fn look_left(self) -> (i32, i32) {
        self.orientation.left().forward(self.pos)
    }
    fn look_right(self) -> (i32, i32) {
        self.orientation.right().forward(self.pos)
    }
    fn turn_left(&mut self) {
        self.orientation = self.orientation.left();
    }
    fn turn_right(&mut self) {
        self.orientation = self.orientation.right();
    }
    fn front(self) -> (i32, i32) {
        self.orientation.forward(self.pos)
    }
    fn forward(&mut self) {
        self.pos = self.front();
    }
}

fn navigate(mut robot: Robot, scaffold: &Scaffold) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::default();
    visited.insert(robot.pos);
    while visited.len() < scaffold.len() {
        // When we are here, the robot is facing "off" the map, so it must
        // either turn left or right (all corners are 90 degrees and the maze is linear):
        // .#.
        // ###
        // .#.
        let rotate = if scaffold.contains(&robot.look_left()) {
            robot.turn_left();
            Rotate::Left
        } else if scaffold.contains(&robot.look_right()) {
            robot.turn_right();
            Rotate::Right
        } else {
            return Err(anyhow!(
                "Invariant broken: {robot:?} not finding scaffold by turning L/R"
            ));
        };

        let mut forwards = 0;
        while scaffold.contains(&robot.front()) {
            forwards += 1;
            robot.forward();
            visited.insert(robot.pos);
        }
        actions.push(Action { rotate, forwards });
    }
    Ok(actions)
}

pub fn part_2(input: &str) -> Result<String> {
    let (scaffold, robot) = initial_state(input)?;
    let _actions = navigate(robot, &scaffold)?;
    // The boring answer here is that I solved it manually in emacs instead of coding it,
    // after trying to program it for a few afternoons, but stumbling to formulate the problem correctly.
    let main = "A,A,C,B,C,A,B,C,B,A\n";
    let prog_a = "L,6,R,12,L,6,L,8,L,8\n"; // Longest one, at 20 lines, max permitted
    let prog_b = "L,4,L,4,L,6\n";
    let prog_c = "L,6,R,12,R,8,L,8\n";
    let no_live_video = "n\n";
    let mut prog = Program::parse(input.lines().next().context("Empty input")?)?;
    prog.write_addr(0, 2, Immediate);
    // Now we need to input all the above as ascii codes:
    for routine in [main, prog_a, prog_b, prog_c, no_live_video] {
        for ch in routine.bytes() {
            prog.input(ch as i64);
        }
    }
    prog.exec()?;
    Ok(format!("{}", prog.output().last().unwrap()))
}
