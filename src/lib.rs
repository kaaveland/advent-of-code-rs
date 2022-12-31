extern crate core;

use anyhow::{Context, Result};
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
pub mod dl_data;

pub struct Solution {
    day_no: u8,
    part_1: fn(&str) -> Result<()>,
    part_2: fn(&str) -> Result<()>,
}

pub fn not_implemented(_: &str) -> Result<()> {
    println!("Not implemented yet");
    Ok(())
}

pub fn timed_solution(day: u8) -> Result<()> {
    let path = format!("./input/day_{day:0>2}/input");
    let have_it = fs::read_to_string(path.as_str());
    let content = if let Ok(found) = have_it {
        found
    } else {
        dl_data::single_day(day)?;
        fs::read_to_string(path.as_str())?
    };

    let now = Instant::now();
    let candidate = SOLUTIONS
        .iter()
        .find(|sol| sol.day_no == day)
        .context(format!("Error: no solution for day: {day}"))?;
    println!("Run part 1 of day {day}");
    (candidate.part_1)(content.as_str())?;
    println!("Took {}ms", now.elapsed().as_millis());
    let now = Instant::now();
    println!("Run part 2 of day {day}");
    (candidate.part_2)(content.as_str())?;
    println!("Took {}ms", now.elapsed().as_millis());
    Ok(())
}

pub fn timed_all_solutions() -> Result<()> {
    let now = Instant::now();
    println!("Run all implemented solutions");

    for sol in SOLUTIONS.iter() {
        timed_solution(sol.day_no)?;
    }
    println!(
        "All implemented solutions took: {}ms",
        now.elapsed().as_millis()
    );
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
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 10,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 11,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 12,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 13,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 14,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 15,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 16,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 17,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 18,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 19,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 20,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 21,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 22,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 23,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 24,
        part_1: not_implemented,
        part_2: not_implemented,
    },
    Solution {
        day_no: 25,
        part_1: not_implemented,
        part_2: not_implemented,
    },
];
