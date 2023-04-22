use crate::intcode::{Output, Program};
use anyhow::{Context, Result};
use itertools::Itertools;

pub fn part_1(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Missing line in input")?)?;
    (0..5)
        .permutations(5)
        .filter_map(|mut signals| {
            let mut input_signal = 0;
            while let Some(phase_signal) = signals.pop() {
                let mut amp = prog.clone();
                amp.input(phase_signal);
                amp.input(input_signal);
                amp.exec().ok()?;
                input_signal = amp.output()[0];
            }
            Some(input_signal)
        })
        .max()
        .context("No combination found")
        .map(|n| format!("{n}"))
}

fn feedback_loop(prog: &Program, phase_signals: &[i64]) -> Result<i64> {
    let mut programs = phase_signals
        .iter()
        .copied()
        .map(|phase_signal| {
            let mut this = prog.clone();
            this.input(phase_signal);
            this
        })
        .collect_vec();
    let mut current_signal = 0;
    let mut current_prog = 0;

    loop {
        programs[current_prog].input(current_signal);
        if let Output::Value(next_signal) = programs[current_prog].produce_output()? {
            current_signal = next_signal;
            current_prog = (current_prog + 1) % programs.len();
        } else {
            return programs[programs.len() - 1]
                .output()
                .last()
                .copied()
                .context("Missing output");
        }
    }
}

pub fn part_2(input: &str) -> Result<String> {
    let prog = Program::parse(input.lines().next().context("Missing line in input")?)?;
    (5..=9)
        .permutations(5)
        .filter_map(|phase_signals| feedback_loop(&prog, &phase_signals).ok())
        .max()
        .context("No combination found")
        .map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_ex() {
        let phase_signals = vec![9, 8, 7, 6, 5];
        let prog = Program::parse(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        )
        .unwrap();
        assert_eq!(feedback_loop(&prog, &phase_signals).unwrap(), 139629729);
    }
    #[test]
    fn test_second_ex() {
        let phase_signals = vec![9, 7, 8, 5, 6];
        let prog = Program::parse(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
        ).unwrap();
        assert_eq!(feedback_loop(&prog, &phase_signals).unwrap(), 18216);
    }
}
