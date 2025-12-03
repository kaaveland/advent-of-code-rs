use shared::Answer;

mod day_01;
mod day_02;
mod day_03;

pub const SOLUTIONS: &[Answer] = &[
    Answer::SolvedBoth {
        part_1: day_01::part_1,
        part_2: day_01::part_2,
    },
    Answer::SolvedBoth {
        part_1: day_02::part_1,
        part_2: day_02::part_2,
    },
    Answer::SolvedBoth {
        part_1: day_03::part_1,
        part_2: day_03::part_2,
    },
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
    Answer::NotImplementedYet,
];
