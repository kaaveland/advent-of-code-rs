use crate::intcode::{ParameterMode, Program};
use anyhow::{Context, Result};
use fxhash::FxHashSet as HashSet;

pub fn part_1(input: &str) -> Result<String> {
    let mut prog = Program::parse(input.lines().next().context("Empty input")?)?;
    prog.exec()?;
    let mut blocks = HashSet::default();
    for chunk in prog.output().chunks(3) {
        if let [x, y, tile_id] = *chunk {
            if tile_id == 2 {
                blocks.insert((x, y));
            } else if tile_id == 4 {
                blocks.remove(&(x, y));
            }
        } else {
            panic!("{chunk:?}");
        }
    }
    let n = blocks.len();
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let mut prog = Program::parse(input.lines().next().context("Empty input")?)?;
    prog.write_addr(0, 2, ParameterMode::Immediate);
    let mut blocks = HashSet::default();
    let mut ball = None;
    let mut paddle = None;
    let mut score = 0;

    fn update_from_chunk(
        paddle: &mut Option<i64>,
        ball: &mut Option<i64>,
        blocks: &mut HashSet<(i64, i64)>,
        chunk: &[i64],
    ) {
        if let [x, y, tile_id] = *chunk {
            if tile_id == 2 {
                blocks.insert((x, y));
            } else if tile_id == 4 {
                *ball = Some(x);
                blocks.remove(&(x, y));
            } else if tile_id == 3 {
                *paddle = Some(x);
            } else if tile_id == 0 || tile_id == 1 {
                blocks.remove(&(x, y));
            }
        } else {
            panic!("{chunk:?}");
        }
    }

    // Find the ball / paddle by playing until the game expects input
    for chunk in prog.require_input(false)?.chunks(3) {
        update_from_chunk(&mut paddle, &mut ball, &mut blocks, chunk);
    }

    // Play until the blocks are down
    while !blocks.is_empty() {
        // Provide the input
        if let Some(dx) = ball.and_then(|bx| paddle.map(|px| (bx - px).signum())) {
            prog.input(dx);
        }
        // Play until input is required again
        for chunk in prog.require_input(true)?.chunks(3) {
            update_from_chunk(&mut paddle, &mut ball, &mut blocks, chunk);
            if chunk[0] == -1 {
                score = chunk[2];
            }
        }
    }

    Ok(format!("{score}"))
}
