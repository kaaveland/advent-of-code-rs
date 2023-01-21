use anyhow::Result;
use aoc::{available_years, dl_data, timed_all_solutions, timed_solution};
use clap::{arg, value_parser, Command};

fn cli() -> Command {
    let max_solution: i64 = 25;
    let ymin = *available_years().iter().min().unwrap() as i64;
    let ymax = *available_years().iter().max().unwrap() as i64;

    Command::new("aoc")
        .about("Advent of Code toolset")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("day-data")
                .about("Get data for day (dump to input/year/day_nn/input")
                .arg(arg!(<day> "Day number to fetch data for")
                    .required(true)
                    .value_parser(value_parser!(u8).range(1..max_solution))
                )
                .arg(arg!([year] "Year to fetch data for")
                    .default_value("2021")
                    .value_parser(value_parser!(u16).range(ymin..=ymax)))
        )
        .subcommand(
            Command::new("data")
                .about("Get data for all days")
                .arg(arg!([year] "Year to fetch data for")
                    .default_value("2021")
                    .value_parser(value_parser!(u16).range(ymin..=ymax)))
        )
        .subcommand(
            Command::new("run")
                .about("Run solution, both parts, with timing")
                .arg(arg!(<day> "Day number to run. Caches data locally in input/day_nn/input if not present")
                    .value_parser(value_parser!(u8).range(1..max_solution))
                )
                .arg(arg!([year] "Year to fetch data for")
                    .default_value("2021")
                    .value_parser(value_parser!(u16).range(ymin..=ymax)))
        )
        .subcommand(
            Command::new("runall")
                .about("Run all known solutions, with individual and total timing")
                .arg(arg!([year] "Year to fetch data for")
                    .default_value("2021")
                    .value_parser(value_parser!(u16).range(ymin..=ymax)))
        )
}

fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("day-data", sub_matches)) => {
            let day = *sub_matches.get_one::<u8>("day").unwrap();
            let year = *sub_matches.get_one::<u16>("year").unwrap();
            dl_data::single_day(year, day)
        }
        Some(("data", sub_matches)) => {
            let year = *sub_matches.get_one::<u16>("year").unwrap();
            dl_data::all_days(year)
        }
        Some(("run", sub_matches)) => {
            let day = *sub_matches.get_one::<u8>("day").unwrap();
            let year = *sub_matches.get_one::<u16>("year").unwrap();
            let rep = timed_solution(year, day)?;
            println!("{}", rep);
            Ok(())
        }
        Some(("runall", sub_matches)) => {
            let year = *sub_matches.get_one::<u16>("year").unwrap();
            timed_all_solutions(year)
        }
        _ => unreachable!(),
    }
}
