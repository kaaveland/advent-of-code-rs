use std::num::ParseIntError;
use fxhash::FxHashSet;
use itertools::Itertools;

fn parse_line(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split_whitespace().map(|n|n.parse::<i32>()).collect()
}

fn parse(input: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    input.lines().filter(|n|!n.is_empty()).map(parse_line).collect()
}

fn signum(n: i32) -> i32 {
    i32::from(n > 0) - i32::from(n < 0)
}

fn safe(report: &[i32]) -> bool {
    let diffs = report.iter().zip(report[1..].iter()).map(|(l, r)| l - r);
    let signs: FxHashSet<i32> = diffs.clone().map(signum).collect();
    let max = diffs.map(i32::abs).max().unwrap_or(0);
    let monotonic = signs.len() == 1 && (signs.contains(&1) || signs.iter().contains(&-1));
    monotonic && max <= 3
}

fn safe_with_one_drop(report: &[i32]) -> bool {
    (0..report.len()).any(|drop| {
        let subreport: Vec<_> = report.iter().enumerate()
            .filter_map(|(i,value)| if i != drop { Some(*value) } else { None })
            .collect();
        safe(&subreport)
    })
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let reports = parse(input)?;
    let n = reports.iter().filter(|report| safe(report)).count();
    Ok(format!("{n}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let reports = parse(input)?;
    let n = reports.iter().filter(|report| safe_with_one_drop(report)).count();
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_safe_reports() {
        assert_eq!(part_1(EXAMPLE).unwrap().as_str(), "2");
        assert_eq!(part_2(EXAMPLE).unwrap().as_str(), "4");
    }

}
