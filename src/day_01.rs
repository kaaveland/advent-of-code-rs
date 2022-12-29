use anyhow::Result;
use itertools::Itertools;
use std::num::ParseIntError;

#[cfg(test)]
pub mod tests {
    use super::*;
    const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn test_parse() {
        let depths = parse(EXAMPLE).unwrap();
        assert_eq!(depths[0], 199);
        assert_eq!(depths[1], 200);
    }

    #[test]
    fn test_solve_1() {
        let depths = parse(EXAMPLE).unwrap();
        assert_eq!(solve_1(&depths), 7);
    }

    #[test]
    fn test_solve_2() {
        let depths = parse(EXAMPLE).unwrap();
        assert_eq!(solve_2(&depths), 5);
    }
}

fn parse(input: &str) -> Result<Vec<i32>> {
    let r: Result<Vec<_>, ParseIntError> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::parse::<i32>)
        .collect();
    let v = r?;
    Ok(v)
}

fn solve_1(depths: &[i32]) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(&before, &now)| now > before)
        .count()
}

fn solve_2(depths: &[i32]) -> usize {
    let sums = depths
        .iter()
        .zip(depths.iter().skip(1))
        .map(|(&before, &now)| before + now)
        .zip(depths.iter().skip(2))
        .map(|(before, &now)| before + now)
        .collect_vec();
    sums.iter()
        .zip(sums.iter().skip(1))
        .filter(|(&before, &now)| now > before)
        .count()
}

pub fn part_1(input: &str) -> Result<()> {
    let depths = parse(input)?;
    let sol = solve_1(&depths);
    println!("{sol}");
    Ok(())
}

pub fn part_2(input: &str) -> Result<()> {
    let depths = parse(input)?;
    let sol = solve_2(&depths);
    println!("{sol}");
    Ok(())
}
