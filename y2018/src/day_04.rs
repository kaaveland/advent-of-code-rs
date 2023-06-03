use anyhow::{anyhow, Result};
use chrono::{NaiveDateTime, Timelike};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Event {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

struct LogEntry {
    timestamp: NaiveDateTime,
    event: Event,
}

fn parse_log_entry(input: &str) -> IResult<&str, LogEntry> {
    let begin_shift = map(
        delimited(
            tag("Guard #"),
            map_res(digit1, str::parse::<u32>),
            tag(" begins shift"),
        ),
        Event::BeginShift,
    );
    let wake = map(tag("wakes up"), |_| Event::WakeUp);
    let sleep = map(tag("falls asleep"), |_| Event::FallAsleep);
    let event = alt((begin_shift, wake, sleep));
    let extract = tuple((
        delimited(
            char('['),
            map_res(take_until("]"), |s: &str| {
                NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M")
            }),
            tag("] "),
        ),
        event,
    ));
    map(extract, |(timestamp, event)| LogEntry { timestamp, event })(input)
}

fn parse_log(input: &str) -> Result<Vec<LogEntry>> {
    separated_list1(char('\n'), parse_log_entry)(input)
        .map_err(|e| anyhow!("{e}"))
        .map(|(_, v)| v)
}

fn tally_log(log: &[LogEntry]) -> HashMap<u32, [u32; 60]> {
    assert!(matches!(log[0].event, Event::BeginShift(_)));
    let mut active_guard: Option<u32> = None;
    let mut tally: HashMap<u32, [u32; 60]> = HashMap::default();
    for (prev, next) in log.iter().zip(log.iter().skip(1)) {
        if let Event::BeginShift(id) = prev.event {
            active_guard = Some(id);
        } else if matches!(prev.event, Event::FallAsleep) {
            assert!(matches!(next.event, Event::WakeUp));
            let entry = tally.entry(active_guard.unwrap()).or_insert([0; 60]);
            for i in prev.timestamp.minute()..next.timestamp.minute() {
                entry[i as usize] += 1;
            }
        }
    }
    tally
}

fn tallied_logs(input: &str) -> Result<HashMap<u32, [u32; 60]>> {
    let log: Vec<_> = parse_log(input)?
        .into_iter()
        .sorted_by_key(|entry| entry.timestamp)
        .collect();
    Ok(tally_log(&log))
}

pub fn part_1(input: &str) -> Result<String> {
    tallied_logs(input)?
        .into_iter()
        .max_by_key(|(_, v)| v.iter().sum::<u32>())
        .map(|(id, v)| {
            // Safe, v is [u32; 60]
            let (minute, _) = v.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
            id * minute as u32
        })
        .map(|v| v.to_string())
        .ok_or_else(|| anyhow!("No sleeping guard found"))
}

pub fn part_2(input: &str) -> Result<String> {
    tallied_logs(input)?
        .into_iter()
        .map(|(id, v)| {
            (
                v.iter()
                    .enumerate()
                    .map(|(minute, tally)| (*tally, minute))
                    .max()
                    .unwrap(),
                id,
            )
        })
        .max()
        .map(|((_, slept), id)| slept as u32 * id)
        .map(|v| v.to_string())
        .ok_or_else(|| anyhow!("No sleeping guard found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_LOG: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";
    #[test]
    fn happy_day_parsing() {
        let log = parse_log(EXAMPLE_LOG).unwrap();
        assert_eq!(log.len(), 17);
        assert!(log
            .iter()
            .any(|entry| matches!(entry.event, Event::BeginShift(10))));
    }

    #[test]
    fn happy_day_part_1() {
        assert_eq!(part_1(EXAMPLE_LOG).unwrap(), "240");
    }

    #[test]
    fn happy_day_part_2() {
        assert_eq!(part_2(EXAMPLE_LOG).unwrap(), "4455");
    }
}
