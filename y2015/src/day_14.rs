use anyhow::Context;
use regex::Regex;

#[derive(Debug, Default)]
struct Reindeer {
    velocity: i64,
    work_seconds: i64,
    rest_seconds: i64,
}

fn distance(reindeer: &Reindeer, time: i64) -> i64 {
    let cycle_length = reindeer.work_seconds + reindeer.rest_seconds;
    let complete_cycles = time / cycle_length;
    let remainder = time.rem_euclid(cycle_length).min(reindeer.work_seconds);
    complete_cycles * reindeer.velocity * reindeer.work_seconds + remainder * reindeer.velocity
}

fn parse_reindeers(s: &str) -> anyhow::Result<Vec<Reindeer>> {
    let re = Regex::new(r"\d+").context("Invalid regex")?;
    let mut reindeers = Vec::new();
    for line in s.lines() {
        let mut reindeer = Reindeer::default();
        let slots = [
            &mut reindeer.velocity,
            &mut reindeer.work_seconds,
            &mut reindeer.rest_seconds,
        ];
        for (i, n) in re.find_iter(line).enumerate() {
            let n = n.as_str().parse().context("Invalid int")?;
            *slots[i] = n;
        }
        reindeers.push(reindeer);
    }
    Ok(reindeers)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let reindeers = parse_reindeers(s)?;
    let answer = reindeers.iter().map(|r| distance(r, 2503)).max();
    answer.context("No reindeers parsed").map(|n| n.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let reindeers = parse_reindeers(s)?;
    let mut scores = vec![0; reindeers.len()];

    for time in 1..=2503 {
        let distances: Vec<_> = reindeers.iter().map(|r| distance(r, time)).collect();
        let most = *distances.iter().max().context("No reindeers parsed")?;
        for (ix, dist) in distances.into_iter().enumerate() {
            if dist == most {
                scores[ix] += 1;
            }
        }
    }
    scores
        .into_iter()
        .max()
        .context("No reindeers parsed")
        .map(|n| n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let comet = Reindeer {
            velocity: 14,
            work_seconds: 10,
            rest_seconds: 127,
        };
        let dancer = Reindeer {
            velocity: 16,
            work_seconds: 11,
            rest_seconds: 162,
        };
        assert_eq!(distance(&comet, 1), 14);
        assert_eq!(distance(&dancer, 1), 16);
        assert_eq!(distance(&comet, 15), 140);
        assert_eq!(distance(&dancer, 15), 11 * 16);
        assert_eq!(distance(&comet, 1000), 1120);
        assert_eq!(distance(&dancer, 1000), 1056);
    }
}
