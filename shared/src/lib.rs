use anyhow::Result;
use std::time::Instant;

pub enum Answer {
    Solution {
        part_1: fn(&str) -> Result<String>,
        part_2: fn(&str) -> Result<String>,
    },
    NotImplementedYet,
}

pub fn elapsed_string(now: Instant) -> String {
    if now.elapsed().as_millis() > 2 {
        format!("{}ms", now.elapsed().as_millis())
    } else {
        format!("{}μs", now.elapsed().as_micros())
    }
}

pub fn grid_parser<'a, T, F, N>(input: &'a str, f: &'a F) -> impl Iterator<Item = ((N, N), T)> + 'a
where
    F: Fn(char) -> Option<T>,
    N: From<i32>,
{
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                f(ch).map(|r| (((x as i32).into(), (y as i32).into()), r))
            })
        })
}
