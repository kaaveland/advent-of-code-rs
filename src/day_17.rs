use anyhow::{anyhow, Result};
use rayon::prelude::*;
use regex::Regex;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

#[derive(Eq, PartialEq, Debug)]
struct Area {
    xrange: RangeInclusive<i32>,
    yrange: RangeInclusive<i32>,
}

fn parse_area(input: &str) -> Result<Area> {
    let re = Regex::new(r"(-?[0-9]+)+")?;
    let caps = re.captures_iter(input);
    let r: Result<Vec<_>, _> = caps
        .map(|cap| cap.get(1).unwrap().as_str().parse::<i32>())
        .collect();
    let r = r?;
    if r.len() != 4 {
        Err(anyhow!("Illegal input: {r:?}, needed to match 4 numbers"))
    } else {
        Ok(Area {
            xrange: r[0]..=r[1],
            yrange: r[2]..=r[3],
        })
    }
}

fn visits(area: &Area, mut dx: i32, mut dy: i32) -> bool {
    let (mut x, mut y) = (0, 0);
    let ymin = *min(area.yrange.start(), area.yrange.end());
    while y >= ymin {
        if area.xrange.contains(&x) && area.yrange.contains(&y) {
            return true;
        }
        x += dx;
        dx = max(0, dx - 1);
        y += dy;
        dy -= 1;
    }
    false
}

fn count_distinct_velocities(area: &Area) -> usize {
    let y_span = max(area.yrange.start().abs(), area.yrange.end().abs());
    let y_span = -y_span..=y_span;
    y_span
        .into_par_iter()
        .map(move |dy| {
            (1..=*area.xrange.end())
                .map(move |dx| (dx, dy))
                .filter(|(dx, dy)| visits(area, *dx, *dy))
                .count()
        })
        .sum()
}

pub fn part_1(input: &str) -> Result<String> {
    let area = parse_area(input)?;
    let ymin = *area.yrange.start();
    let max_y = (ymin * ymin + ymin) / 2;
    Ok(format!("{max_y}"))
}
pub fn part_2(input: &str) -> Result<String> {
    let area = parse_area(input)?;
    let sol = count_distinct_velocities(&area);
    Ok(format!("{sol}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const EXAMPLE: Area = Area {
        xrange: 20..=30,
        yrange: -10..=-5,
    };

    #[test]
    fn test_example() {
        let ans = count_distinct_velocities(&EXAMPLE);
        assert_eq!(ans, 112);
    }
}
