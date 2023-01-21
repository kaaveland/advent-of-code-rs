use anyhow::{anyhow, Result};

type Facing = i32;
const NORTH: Facing = 3;
const EAST: Facing = 0;
const SOUTH: Facing = 1;
const WEST: Facing = 2;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
struct ShipState {
    north: i32,
    south: i32,
    west: i32,
    east: i32,
    facing: i32,
}

impl ShipState {
    fn manhattan(&self) -> i32 {
        (self.north - self.south).abs() + (self.west - self.east).abs()
    }
}

fn solve_1(input: &str) -> Result<i32> {
    let mut ship = ShipState::default();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let first = line.chars().next().unwrap(); // Always ok, line not empty
        let rest = line[1..].parse::<i32>()?;
        match first {
            'N' => {
                ship.north += rest;
            }
            'E' => {
                ship.east += rest;
            }
            'W' => {
                ship.west += rest;
            }
            'S' => {
                ship.south += rest;
            }
            'R' => {
                ship.facing = (ship.facing + rest / 90).rem_euclid(4);
            }
            'L' => {
                ship.facing = (ship.facing - rest / 90).rem_euclid(4);
            }
            'F' => match ship.facing {
                NORTH => {
                    ship.north += rest;
                }
                EAST => {
                    ship.east += rest;
                }
                WEST => {
                    ship.west += rest;
                }
                SOUTH => {
                    ship.south += rest;
                }
                other => return Err(anyhow!("Illegal state facing: {other}")),
            },
            other => return Err(anyhow!("Illegal instruction: {other}")),
        }
    }

    Ok(ship.manhattan())
}

pub fn part_1(input: &str) -> Result<String> {
    solve_1(input).map(|n| format!("{n}"))
}

#[derive(Debug, Eq, PartialEq)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint { x: 10, y: 1 }
    }
}

impl Waypoint {
    fn clockwise(&mut self) {
        // rot90ccw(x, y) = (y, -x)
        std::mem::swap(&mut self.x, &mut self.y);
        self.y *= -1;
    }

    fn counter_clockwise(&mut self) {
        // rot90(x, y) = (-y, x)
        std::mem::swap(&mut self.x, &mut self.y);
        self.x *= -1;
    }
}

fn solve_2(input: &str) -> Result<i32> {
    let mut ship = ShipState::default();
    let mut waypoint = Waypoint::default();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let first = line.chars().next().unwrap(); // Always ok, line not empty
        let rest = line[1..].parse::<i32>()?;
        match first {
            'N' => {
                waypoint.y += rest;
            }
            'E' => {
                waypoint.x += rest;
            }
            'W' => {
                waypoint.x -= rest;
            }
            'S' => {
                waypoint.y -= rest;
            }
            'R' => {
                for _ in 0..(rest / 90) {
                    waypoint.clockwise();
                }
            }
            'L' => {
                for _ in 0..(rest / 90) {
                    waypoint.counter_clockwise()
                }
            }
            'F' => {
                for _ in 0..rest {
                    ship.east += waypoint.x;
                    ship.south += waypoint.y;
                }
            }
            other => return Err(anyhow!("Illegal instruction: {other}")),
        }
    }

    Ok(ship.manhattan())
}

pub fn part_2(input: &str) -> Result<String> {
    solve_2(input).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let n = solve_2(
            "F10
N3
F7
R90
F11
",
        )
        .unwrap();
        assert_eq!(n, 286);
    }

    #[test]
    fn test_1() {
        let n = solve_1(
            "F10
N3
F7
R90
F11
",
        )
        .unwrap();
        assert_eq!(n, 25);
    }
}
