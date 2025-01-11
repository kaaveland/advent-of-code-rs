use shared::Answer;
use shared::Answer::{NotImplementedYet, Solution};

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
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
    NotImplementedYet,
];
