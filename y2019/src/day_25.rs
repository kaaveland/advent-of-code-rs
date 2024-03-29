use crate::intcode::Program;
use anyhow::Result;
use itertools::Itertools;

const PLAYTHROUGH: &str = "south
west
south
take shell
north
north
take weather machine
west
south
east
take candy cane
west
north
east
south
east
east
south
take hypercube
south
south
east
north
north
north
south
south
south
east
east";

pub fn part_1(input: &str) -> Result<String> {
    let mut game = Program::parse(input.trim_end())?;
    game.ascii_input(PLAYTHROUGH);
    let _ = game.require_ascii_output();
    let history: String = game.output().iter().map(|ch| (*ch as u8) as char).collect();
    let post = history.lines().rev().take(3).collect_vec();
    let out = post.iter().rev().join("\n");
    Ok(out)
}

pub fn part_2(_input: &str) -> Result<String> {
    Ok("Enter the solutions, collect stars".to_string())
}
