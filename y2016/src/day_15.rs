use regex::Regex;
use std::sync::LazyLock;

#[derive(Copy, Clone)]
struct Disc {
    positions: u8,
    current_position: u8,
}

static DISC_PAT: &str = r"^Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).$";
static DISC_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(DISC_PAT).unwrap());

fn parse(s: &str) -> Vec<Disc> {
    s.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let caps = DISC_RE.captures(line).unwrap();
            let positions = caps.get(1).unwrap().as_str().parse().unwrap();
            let current_position = caps.get(2).unwrap().as_str().parse().unwrap();
            Disc {
                positions,
                current_position,
            }
        })
        .collect()
}

// Problem statement is to find the time such that all discs are in
// position 0 when the capsule passes them at time t + 1 + disc index
// I believe this is equivalent to a set of modular equations:
// Disc #1 has 5 positions; at time=0, it is at position 4.
// Disc #2 has 2 positions; at time=0, it is at position 1.
// (4 + t + 1) % 5 == 0
// (1 + t + 2) % 2 == 0
struct LinearCongruenceSystem {
    coefficients: Vec<usize>,
    moduli: Vec<usize>,
}

impl From<Vec<Disc>> for LinearCongruenceSystem {
    fn from(value: Vec<Disc>) -> Self {
        let mut coefficients = Vec::with_capacity(value.len());
        let mut moduli = Vec::with_capacity(value.len());

        for (i, disc) in value.into_iter().enumerate() {
            coefficients.push(i + 1 + usize::from(disc.current_position));
            moduli.push(usize::from(disc.positions));
        }

        LinearCongruenceSystem {
            coefficients,
            moduli,
        }
    }
}

fn solve(system: &LinearCongruenceSystem) -> usize {
    let mut t = 0;
    let mut t_jump = 1;
    let mut eq = 0;

    while eq < system.moduli.len() {
        if (system.coefficients[eq] + t).is_multiple_of(system.moduli[eq]) {
            // It worked!
            t_jump *= system.moduli[eq];
            eq += 1;
        } else {
            t += t_jump;
        }
    }

    t
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let inp = parse(s).into();
    let ans = solve(&inp);
    Ok(format!("{ans}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let mut discs = parse(s);
    discs.push(Disc {
        positions: 11,
        current_position: 0,
    });
    let ans = solve(&discs.into());
    Ok(format!("{ans}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let system = LinearCongruenceSystem {
            coefficients: vec![5, 3],
            moduli: vec![5, 2],
        };
        assert_eq!(solve(&system), 5);
    }
}
