use shared::Answer::{NotImplementedYet, Solution};
use shared::{not_implemented, Answer};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
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

pub const SOLUTIONS: [Answer; 25] = [
    Solution {
        part_1: day_01::part_1,
        part_2: day_01::part_2,
    },
    Solution {
        part_1: day_02::part_1,
        part_2: day_02::part_2,
    },
    Solution {
        part_1: day_03::part_1,
        part_2: day_03::part_2,
    },
    Solution {
        part_1: day_04::part_1,
        part_2: day_04::part_2,
    },
    Solution {
        part_1: day_05::part_1,
        part_2: day_05::part_2,
    },
    Solution {
        part_1: day_06::part_1,
        part_2: day_06::part_2,
    },
    Solution {
        part_1: day_07::part_1,
        part_2: day_07::part_2,
    },
    Solution {
        part_1: day_08::part_1,
        part_2: day_08::part_2,
    },
    Solution {
        part_1: day_09::part_1,
        part_2: day_09::part_2,
    },
    Solution {
        part_1: day_10::part_1,
        part_2: day_10::part_2,
    },
    Solution {
        part_1: day_11::part_1,
        part_2: day_11::part_2,
    },
    Solution {
        part_1: day_12::part_1,
        part_2: day_12::part_2,
    },
    Solution {
        part_1: day_13::part_1,
        part_2: day_13::part_2,
    },
    Solution {
        part_1: day_14::part_1,
        part_2: day_14::part_2,
    },
    Solution {
        part_1: day_15::part_1,
        part_2: day_15::part_2,
    },
    Solution {
        part_1: day_16::part_1,
        part_2: day_16::part_2,
    },
    Solution {
        part_1: day_17::part_1,
        part_2: day_17::part_2,
    },
    Solution {
        part_1: day_18::part_1,
        part_2: day_18::part_2,
    },
    Solution {
        part_1: day_19::part_1,
        part_2: day_19::part_2,
    },
    Solution {
        part_1: day_20::part_1,
        part_2: day_20::part_2,
    },
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
];
