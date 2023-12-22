use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as Map;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, line_ending};
use nom::combinator::{success, value};
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum MachineKind {
    Broadcast,
    FlipFlop,
    Conj,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Machine<'a> {
    kind: MachineKind,
    name: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum MachineMemory<'a> {
    FlipFlop(bool),
    Conj(Map<&'a str, bool>),
    Broadcast,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MachineSimulation<'a> {
    name: &'a str,
    memory: MachineMemory<'a>,
    outputs: Vec<&'a str>,
}

impl<'a> MachineSimulation<'a> {
    fn receive_signal(&mut self, source: &'a str, high: bool) -> Option<bool> {
        match &mut self.memory {
            MachineMemory::FlipFlop(mem) if !(*mem) && !high => {
                // It's off and pulse is low, it turns on and sends high pulse
                *mem = true;
                Some(true)
            }
            MachineMemory::FlipFlop(mem) if !high => {
                // Pulse is low so it must be on, it turns off and sends low pulse
                *mem = false;
                Some(false)
            }
            MachineMemory::FlipFlop(_) => None,
            MachineMemory::Conj(mem) => {
                *mem.entry(source).or_default() = high;
                Some(!mem.values().all(|high| *high))
            }
            MachineMemory::Broadcast => Some(high),
        }
    }
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let conj = value(MachineKind::Conj, char('&'));
    let flip_flop = value(MachineKind::FlipFlop, char('%'));
    let broadcast = success(MachineKind::Broadcast);
    let (s, (kind, name)) = pair(alt((conj, flip_flop, broadcast)), alpha1)(s)?;
    Ok((s, Machine { kind, name }))
}
fn parse_line(s: &str) -> IResult<&str, (Machine, Vec<&str>)> {
    separated_pair(
        parse_machine,
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    )(s)
}

fn parse(s: &str) -> Result<Vec<(Machine, Vec<&str>)>> {
    Ok(separated_list1(line_ending, parse_line)(s)
        .map_err(|err| anyhow!("{err}"))?
        .1)
}

fn to_sim(s: &str) -> Result<Map<&str, MachineSimulation>> {
    let machines = parse(s)?;
    let mut map = Map::default();
    let mut dest_to_source: Map<_, Vec<_>> = Map::default();
    for (m, outputs) in machines.iter() {
        for o in outputs {
            dest_to_source.entry(*o).or_default().push(m.name);
        }
    }
    for (m, outputs) in machines.iter() {
        let mem = match m.kind {
            MachineKind::Broadcast => MachineMemory::Broadcast,
            MachineKind::FlipFlop => MachineMemory::FlipFlop(false),
            MachineKind::Conj => MachineMemory::Conj(
                // they initially default to remembering a low pulse
                dest_to_source
                    .get(m.name)
                    .unwrap()
                    .iter()
                    .map(|source| (*source, false))
                    .collect(),
            ),
        };
        map.insert(
            m.name,
            MachineSimulation {
                memory: mem,
                name: m.name,
                outputs: outputs.clone(),
            },
        );
    }

    Ok(map)
}

fn simulate(
    map: &mut Map<&str, MachineSimulation>,
    investigate: &str,
    lookfor: bool,
) -> (i32, i32, i32) {
    let mut work = VecDeque::new();
    work.push_back(("button", "broadcaster", false));
    let mut low = 0;
    let mut high = 0;
    let mut rx = 0;

    while let Some((source, destination, high_pulse)) = work.pop_front() {
        if high_pulse {
            high += 1;
        } else {
            low += 1;
        }
        if destination == investigate && high_pulse == lookfor {
            rx += 1;
        }
        if let Some(m) = map.get_mut(destination) {
            if let Some(next_high) = m.receive_signal(source, high_pulse) {
                for next in m.outputs.iter() {
                    work.push_back((destination, *next, next_high));
                }
            }
        }
    }

    (low, high, rx)
}

fn push_button(s: &str) -> Result<(i32, i32, i32)> {
    let mut sim = to_sim(s)?;
    Ok((0..1000).fold((0, 0, 0), |(lo, hi, rx), _| {
        let (next_lo, next_hi, next_rx) = simulate(&mut sim, "rx", false);
        (lo + next_lo, hi + next_hi, rx + next_rx)
    }))
}

pub fn part_1(s: &str) -> Result<String> {
    push_button(s)
        .map(|(lo, hi, _)| (lo as i64) * (hi as i64))
        .map(|n| n.to_string())
}

fn find_period(s: &str, node: &str, high: bool) -> Result<i64> {
    let mut sim = to_sim(s)?;
    (1..)
        .find(|_: &i64| {
            let (_, _, observed_node) = simulate(&mut sim, node, high);
            observed_node > 0
        })
        .context("Unable to find")
}

fn find_connectors_to_rx(s: &str) -> Result<Vec<i64>> {
    let m = to_sim(s)?;
    let connected_to_rx = m
        .iter()
        .filter(|(_, machine)| machine.outputs.contains(&"rx"))
        .collect_vec();
    assert_eq!(connected_to_rx.len(), 1);
    let source_for_rx = connected_to_rx[0];
    assert!(matches!(source_for_rx.1.memory, MachineMemory::Conj(_)));
    let sources = m
        .iter()
        .filter(|(_, machine)| machine.outputs.contains(source_for_rx.0))
        .map(|(name, _)| *name)
        .collect_vec();
    sources
        .into_par_iter()
        .map(|source| find_period(s, source, false))
        .collect()
}

pub fn part_2(s: &str) -> Result<String> {
    find_connectors_to_rx(s).map(|n| n.iter().product::<i64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    #[test]
    fn test_simple_ex() {
        let mut sim = to_sim(EX).unwrap();
        let (lo, hi, _) = simulate(&mut sim, "rx", false);
        assert_eq!(lo, 8);
        assert_eq!(hi, 4);
    }

    #[test]
    fn test_harder_ex() {
        let ex = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        let (lo, hi, _) = push_button(ex).unwrap();
        assert_eq!(lo, 4250);
        assert_eq!(hi, 2750);
    }

    #[test]
    fn test_parser() {
        assert!(parse(EX).is_ok());
        let v = parse(EX).unwrap();
        assert_eq!(v.len(), 5);
        assert_eq!(
            v.iter()
                .filter(|(m, _)| matches!(m.kind, MachineKind::FlipFlop))
                .count(),
            3
        );
    }
}
