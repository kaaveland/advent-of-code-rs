use shared::Answer;
use shared::Answer::SolvedBoth;
use std::string::ToString;

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
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod elflang;

pub const SOLUTIONS: [Answer; 25] = [
    SolvedBoth {
        part_1: day_01::part_1,
        part_2: day_01::part_2,
    },
    SolvedBoth {
        part_1: day_02::part_1,
        part_2: day_02::part_2,
    },
    SolvedBoth {
        part_1: day_03::part_1,
        part_2: day_03::part_2,
    },
    SolvedBoth {
        part_1: day_04::part_1,
        part_2: day_04::part_2,
    },
    SolvedBoth {
        part_1: day_05::part_1,
        part_2: day_05::part_2,
    },
    SolvedBoth {
        part_1: day_06::part_1,
        part_2: day_06::part_2,
    },
    SolvedBoth {
        part_1: day_07::part_1,
        part_2: day_07::part_2,
    },
    SolvedBoth {
        part_1: day_08::part_1,
        part_2: day_08::part_2,
    },
    SolvedBoth {
        part_1: day_09::part_1,
        part_2: day_09::part_2,
    },
    SolvedBoth {
        part_1: day_10::part_1,
        part_2: day_10::part_2,
    },
    SolvedBoth {
        part_1: day_11::part_1,
        part_2: day_11::part_2,
    },
    SolvedBoth {
        part_1: day_12::part_1,
        part_2: day_12::part_2,
    },
    SolvedBoth {
        part_1: day_13::part_1,
        part_2: day_13::part_2,
    },
    SolvedBoth {
        part_1: day_14::part_1,
        part_2: day_14::part_2,
    },
    SolvedBoth {
        part_1: day_15::part_1,
        part_2: day_15::part_2,
    },
    SolvedBoth {
        part_1: day_16::part_1,
        part_2: day_16::part_2,
    },
    SolvedBoth {
        part_1: day_17::part_1,
        part_2: day_17::part_2,
    },
    SolvedBoth {
        part_1: day_18::part_1,
        part_2: day_18::part_2,
    },
    SolvedBoth {
        part_1: day_19::part_1,
        part_2: day_19::part_2,
    },
    SolvedBoth {
        part_1: day_20::part_1,
        part_2: day_20::part_2,
    },
    SolvedBoth {
        part_1: day_21::part_1,
        part_2: day_21::part_2,
    },
    SolvedBoth {
        part_1: day_22::part_1,
        part_2: day_22::part_2,
    },
    SolvedBoth {
        part_1: day_23::part_1,
        part_2: day_23::part_2,
    },
    SolvedBoth {
        part_1: day_24::part_1,
        part_2: day_24::part_2,
    },
    SolvedBoth {
        part_1: day_25::part_1,
        part_2: |_: &str| Ok("Collect stars!".to_string()),
    },
];
