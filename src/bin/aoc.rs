use std::time::Instant;
use anyhow::Result;
use clap::{arg, Command, value_parser};
use itertools::Itertools;
use aoc::{dl_data, SOLUTIONS, timed_all_solutions, timed_solution};

fn cli() -> Command {
    let max_solution: i64= SOLUTIONS.len() as i64 + 1;
    Command::new("aoc")
        .about("Advent of code 2021 toolset")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("day-data")
                .about("Get data for day (dump to input/day_nn/input")
                .arg(arg!(<day> "Day number to fetch data for")
                    .required(true)
                    .value_parser(value_parser!(u8).range(1..max_solution))
                )
        )
        .subcommand(
            Command::new("data")
                .about("Get data for all days")
        )
        .subcommand(
            Command::new("run")
                .about("Run solution, both parts, with timing")
                .arg(arg!(<day> "Day number to run. Caches data locally in input/day_nn/input if not present")
                    .value_parser(value_parser!(u8).range(1..max_solution))
                )
        )
        .subcommand(
            Command::new("runall")
                .about("Run all known solutions, with individual and total timing")
        )
}

fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("day-data", sub_matches)) => {
            let day = *sub_matches.get_one::<u8>("day").unwrap();
            dl_data::single_day(day)
        }
        Some(("data", _)) => {
            dl_data::all_days()
        }
        Some(("run", sub_matches)) => {
            let day = *sub_matches.get_one::<u8>("day").unwrap();
            timed_solution(day)
        }
        Some(("runall", _)) => timed_all_solutions(),
        _ => unreachable!()
    }

}