use anyhow::Result;
use std::num::ParseIntError;

fn horizontal_distance(pos: i64, crab: i64) -> i64 {
    (pos - crab).abs()
}

fn fuel_cost(pos: i64, crab: i64) -> i64 {
    let dist = horizontal_distance(pos, crab);
    (dist * dist + dist) / 2
}

fn solve(input: &str, cost_fn: fn(i64, i64) -> i64) -> Result<i64> {
    let positions: Result<Vec<_>, ParseIntError> =
        input.trim().split(',').map(str::parse::<i64>).collect();
    let positions = positions?;
    let cost = |pos: i64| {
        positions
            .iter()
            .map(|&crab| cost_fn(crab, pos))
            .sum::<i64>()
    };
    let min = *positions.iter().min().unwrap_or(&0);
    let max = *positions.iter().max().unwrap_or(&0);
    Ok((min..=max).map(cost).min().unwrap_or(0))
}

pub fn part_1(input: &str) -> Result<String> {
    let fuel = solve(input, horizontal_distance)?;
    Ok(format!("{fuel}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let fuel = solve(input, fuel_cost)?;
    Ok(format!("{fuel}"))
}
