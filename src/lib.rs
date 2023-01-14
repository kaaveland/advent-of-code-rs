use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;
use std::time::Instant;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

pub mod dl_data;

pub struct Solution {
    day_no: u8,
    part_1: fn(&str) -> Result<String>,
    part_2: fn(&str) -> Result<String>,
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
pub fn timed_solution(day: u8) -> Result<String> {
    let path = format!("./input/day_{day:0>2}/input");
    let have_it = fs::read_to_string(path.as_str());
    let content = if let Ok(found) = have_it {
        found
    } else {
        dl_data::single_day(day)?;
        fs::read_to_string(path.as_str())?
    };

    let candidate = SOLUTIONS
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

pub fn timed_all_solutions() -> Result<()> {
    let now = Instant::now();
    println!("Run all implemented solutions");
    let mut outputs = vec![];

    SOLUTIONS
        .into_par_iter()
        .map(|sol| (sol.day_no, timed_solution(sol.day_no)))
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

pub const SOLUTIONS: [Solution; 25] = [
    Solution {
        day_no: 1,
        part_1: day_01::part_1,
        part_2: day_01::part_2,
    },
    Solution {
        day_no: 2,
        part_1: day_02::part_1,
        part_2: day_02::part_2,
    },
    Solution {
        day_no: 3,
        part_1: day_03::part_1,
        part_2: day_03::part_2,
    },
    Solution {
        day_no: 4,
        part_1: day_04::part_1,
        part_2: day_04::part_2,
    },
    Solution {
        day_no: 5,
        part_1: day_05::part_1,
        part_2: day_05::part_2,
    },
    Solution {
        day_no: 6,
        part_1: day_06::part_1,
        part_2: day_06::part_2,
    },
    Solution {
        day_no: 7,
        part_1: day_07::part_1,
        part_2: day_07::part_2,
    },
    Solution {
        day_no: 8,
        part_1: day_08::part_1,
        part_2: day_08::part_2,
    },
    Solution {
        day_no: 9,
        part_1: day_09::part_1,
        part_2: day_09::part_2,
    },
    Solution {
        day_no: 10,
        part_1: day_10::part_1,
        part_2: day_10::part_2,
    },
    Solution {
        day_no: 11,
        part_1: day_11::part_1,
        part_2: day_11::part_2,
    },
    Solution {
        day_no: 12,
        part_1: day_12::part_1,
        part_2: day_12::part_2,
    },
    Solution {
        day_no: 13,
        part_1: day_13::part_1,
        part_2: day_13::part_2,
    },
    Solution {
        day_no: 14,
        part_1: day_14::part_1,
        part_2: day_14::part_2,
    },
    Solution {
        day_no: 15,
        part_1: day_15::part_1,
        part_2: day_15::part_2,
    },
    Solution {
        day_no: 16,
        part_1: day_16::part_1,
        part_2: day_16::part_2,
    },
    Solution {
        day_no: 17,
        part_1: day_17::part_1,
        part_2: day_17::part_2,
    },
    Solution {
        day_no: 18,
        part_1: day_18::part_1,
        part_2: day_18::part_2,
    },
    Solution {
        day_no: 19,
        part_1: day_19::part_1,
        part_2: day_19::part_2,
    },
    Solution {
        day_no: 20,
        part_1: day_20::part_1,
        part_2: day_20::part_2,
    },
    Solution {
        day_no: 21,
        part_1: day_21::part_1,
        part_2: day_21::part_2,
    },
    Solution {
        day_no: 22,
        part_1: day_22::part_1,
        part_2: day_22::part_2,
    },
    Solution {
        day_no: 23,
        part_1: day_23::part_1,
        part_2: day_23::part_2,
    },
    Solution {
        day_no: 24,
        part_1: day_24::part_1,
        part_2: day_24::part_2,
    },
    Solution {
        day_no: 25,
        part_1: day_25::part_1,
        part_2: day_25::part_2,
    },
];
