use shared::{not_implemented, Solution};

pub mod intcode;

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
