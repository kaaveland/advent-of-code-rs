use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;
use std::time::Instant;

pub mod year_2019;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;

pub mod dl_data;
pub mod point_2d;

pub struct Solution {
    day_no: u8,
    part_1: fn(&str) -> Result<String>,
    part_2: fn(&str) -> Result<String>,
}

const YEARS: [(u16, [Solution; 25]); 3] = [
    (2020, year_2020::SOLUTIONS),
    (2021, year_2021::SOLUTIONS),
    (2022, year_2022::SOLUTIONS),
];

pub fn available_years() -> Vec<u16> {
    YEARS.iter().map(|(y, _)| y).copied().sorted().collect()
}

pub fn not_implemented(_: &str) -> Result<String> {
    Ok("Not implemented yet".to_string())
}

fn elapsed_string(now: Instant) -> String {
    if now.elapsed().as_millis() > 2 {
        format!("{}ms", now.elapsed().as_millis())
    } else {
        format!("{}μs", now.elapsed().as_micros())
    }
}
pub fn timed_solution(year: u16, day: u8) -> Result<String> {
    let path = format!("./input/{year}/day_{day:0>2}/input");
    let have_it = fs::read_to_string(path.as_str());
    let content = if let Ok(found) = have_it {
        found
    } else {
        dl_data::single_day(year, day)?;
        fs::read_to_string(path.as_str())?
    };

    let solution_set = &YEARS
        .iter()
        .find(|(y, _)| *y == year)
        .with_context(|| anyhow!("No solutions for {year} yet"))?
        .1;

    let candidate = solution_set
        .iter()
        .find(|sol| sol.day_no == day)
        .context(format!("Error: no solution for day: {day}"))?;
    let now = Instant::now();

    let p1_sol = (candidate.part_1)(content.as_str())?;
    let p1_ts = elapsed_string(now);
    let now = Instant::now();
    let p2_sol = (candidate.part_2)(content.as_str())?;
    let p2_ts = elapsed_string(now);
    Ok(format!(
        "Day {day} part 1: {p1_ts}\n{p1_sol}\nDay {day} part 2: {p2_ts}\n{p2_sol}\n"
    ))
}

pub fn timed_all_solutions(year: u16) -> Result<()> {
    let now = Instant::now();
    let solution_set = &YEARS
        .iter()
        .find(|(y, _)| *y == year)
        .with_context(|| anyhow!("No solutions for {year} yet"))?
        .1;

    println!("Run all implemented solutions for {year}");
    let mut outputs = vec![];

    solution_set
        .into_par_iter()
        .map(|sol| (sol.day_no, timed_solution(year, sol.day_no)))
        .collect_into_vec(&mut outputs);

    let ts = elapsed_string(now);
    let r: Result<Vec<_>> = outputs
        .into_iter()
        .sorted_by_key(|tup| tup.0)
        .map(|tup| tup.1)
        .collect();
    let r = r?;
    print!("{}", r.join(""));

    println!("All implemented solutions took: {}", ts);
    Ok(())
}
