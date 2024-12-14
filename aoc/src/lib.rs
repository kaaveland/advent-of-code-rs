use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use shared::{elapsed_string, Solution};
use std::fs;
use std::ops::Add;
use std::time::Instant;
use time::{Date, Duration, Month, OffsetDateTime};

pub mod dl_data;

const YEARS: [(u16, [Solution; 25]); 7] = [
    (2018, y2018::SOLUTIONS),
    (2019, y2019::SOLUTIONS),
    (2020, y2020::SOLUTIONS),
    (2021, y2021::SOLUTIONS),
    (2022, y2022::SOLUTIONS),
    (2023, y2023::SOLUTIONS),
    (2024, y2024::SOLUTIONS),
];

pub fn available_years() -> Vec<u16> {
    YEARS.iter().map(|(y, _)| y).copied().sorted().collect()
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
    let today = OffsetDateTime::now_utc().add(Duration::hours(4)).date();

    let solution_set = &YEARS
        .iter()
        .find(|(y, _)| *y == year)
        .with_context(|| anyhow!("No solutions for {year} yet"))?
        .1;

    println!("Run all implemented solutions for {year}");
    let mut outputs = vec![];

    let solution_set: Vec<_> = solution_set
        .iter()
        .filter(|sol| {
            Date::from_calendar_date(year as i32, Month::December, sol.day_no).unwrap() < today
        })
        .collect();

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
