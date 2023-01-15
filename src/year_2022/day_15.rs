use anyhow::{Context, Result};
use fxhash::FxHashSet as HashSet;
use regex::Regex;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Location(i32, i32);
impl Location {
    fn x(&self) -> i32 {
        match self {
            Location(x, _) => *x,
        }
    }
    fn y(&self) -> i32 {
        match self {
            Location(_, y) => *y,
        }
    }
}

fn manhattan_dist(left: &Location, right: &Location) -> i32 {
    (left.x() - right.x()).abs() + (left.y() - right.y()).abs()
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Input(Location, Location);
type Map = Vec<Input>;

impl From<(i32, i32)> for Location {
    fn from(tup: (i32, i32)) -> Self {
        let (x, y) = tup;
        Location(x, y)
    }
}
impl From<Location> for (i32, i32) {
    fn from(loc: Location) -> Self {
        (loc.x(), loc.y())
    }
}

fn parse_lines<T: AsRef<str>>(input: T) -> Result<Map> {
    let re = Regex::new(
        r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )?;

    input
        .as_ref()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let caps = re.captures(l).context("Expected match")?;
            let x1s = caps.get(1).context("Expected x1")?;
            let y1s = caps.get(2).context("Expected y1")?;
            let x2s = caps.get(3).context("Expected x2")?;
            let y2s = caps.get(4).context("Expected y2")?;
            let x1 = x1s.as_str().parse()?;
            let y1 = y1s.as_str().parse()?;
            let x2 = x2s.as_str().parse()?;
            let y2 = y2s.as_str().parse()?;
            Ok(Input(Location(x1, y1), Location(x2, y2)))
        })
        .collect()
}

fn solve_problem_one(inputs: &Map, row: i32) -> usize {
    let mut ranges = vec![];
    for Input(sensor, beacon) in inputs.iter() {
        let distance = manhattan_dist(sensor, beacon);
        let remaining = distance - (sensor.y() - row).abs();
        let intersect_x = sensor.x();

        if remaining <= 0 {
            continue;
        }
        let x1 = intersect_x - remaining;
        let x2 = intersect_x + remaining;
        ranges.push(x1..=x2);
    }
    ranges.sort_by_key(|range| *range.start());
    let mut it = ranges.iter().cloned();
    let mut curr = it.next().unwrap();
    let mut range_set = vec![];

    for next in it {
        if curr.end() + 1 >= *next.start() {
            curr = RangeInclusive::new(*curr.start(), *next.end());
        } else {
            range_set.push(curr);
            curr = next;
        }
    }
    range_set.push(curr);
    let total_range = range_set.iter().fold((0, 0), |acc, range| {
        (min(acc.0, *range.start()), max(acc.1, *range.end()))
    });
    (total_range.1 - total_range.0) as usize
}

// Find intersection point of y = -x + b_neg and y = x + b_pos
fn intersect_lines(b_neg: i32, b_pos: i32) -> (i32, i32) {
    // Add equations => 2 y = b_neg + b_pos
    let y = (b_neg + b_pos) / 2;
    // Solve either one for x
    let x = y - b_pos;
    (x, y)
}

fn find_distress_beacon(map: &Map) -> Option<Location> {
    let (xmin, xmax) = map.iter().fold((0, 0), |(xmin, xmax), input| match input {
        Input(Location(x, _), _) if *x < xmin => (*x, xmax),
        Input(Location(x, _), _) if *x > xmax => (xmin, *x),
        _ => (xmin, xmax),
    });
    let (ymin, ymax) = map.iter().fold((0, 0), |(ymin, ymax), input| match input {
        Input(Location(_, y), _) if *y < ymin => (*y, ymax),
        Input(Location(_, y), _) if *y > ymax => (ymin, *y),
        _ => (ymin, ymax),
    });

    // The distress beacon is in some point that is 1 outside of a sensor range
    // and where there is an intersection between "circles" just outside sensor range
    // Solve the y = ax + b equations and find the intersects to narrow the search space
    let mut pos_slope_intersects = vec![];
    let mut neg_slope_intersects = vec![];

    for Input(sensor, beacon) in map.iter() {
        let dist = manhattan_dist(sensor, beacon);
        let outside = dist + 1;

        // 4 lines of form: y = ax + b, y is fixed, it's sensor y
        // x can be -1 or 1, find b
        let y = sensor.1;
        let x = sensor.0;
        let west = x - outside;
        let east = x + outside;
        pos_slope_intersects.push(
            // Originating in west with positive slope y = ax + b, so b = y - a when x is 1, nw
            y - west,
        );
        pos_slope_intersects.push(
            // Originating in east with positive slope y = ax + b, so b = y - a when x is 1, se
            y - east,
        );
        neg_slope_intersects.push(
            // Originating in west with negative slope y = ax + b so b = y + a when x is -1, sw
            y + west,
        );
        neg_slope_intersects.push(
            // Originating in east with negative slope y = ax + b so b = y + a when x is -1, ne
            y + east,
        );
    }

    let mut intersects = HashSet::default();
    for neg_slope in neg_slope_intersects.iter() {
        for pos_slope in pos_slope_intersects.iter() {
            let point = intersect_lines(*neg_slope, *pos_slope);
            intersects.insert(Location(point.0, point.1));
        }
    }

    intersects.retain(|&Location(x, y)| x >= xmin && x <= xmax && y >= ymin && y <= ymax);

    for Input(sensor, beacon) in map.iter() {
        let dist = manhattan_dist(sensor, beacon);
        intersects.retain(|loc| manhattan_dist(sensor, loc) > dist);
    }

    println!("Found {} after filter", intersects.len());

    if intersects.len() != 1 {
        None
    } else {
        intersects.iter().next().cloned()
    }
}

fn tuning_distance(loc: &Location) -> i64 {
    let x = loc.x() as i64;
    let y = loc.y() as i64;
    x * 4000000 + y
}

pub fn part_1(input: &str) -> Result<String> {
    let map = parse_lines(input)?;
    let solution = solve_problem_one(&map, 2000000);
    Ok(format!("{solution}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let map = parse_lines(input)?;
    let distress_beacon = find_distress_beacon(&map).context("Unable to find 1 point")?;
    let solution_part_2 = tuning_distance(&distress_beacon);
    Ok(format!("{solution_part_2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_manhattan_dist() {
        let origin = Location(0, 0);
        let left = Location(-1, 0);
        let diag_right = Location(1, 1);

        assert_eq!(manhattan_dist(&origin, &left), 1);
        assert_eq!(manhattan_dist(&diag_right, &origin), 2);
    }

    #[test]
    fn test_parsing() {
        let inputs = parse_lines(EXAMPLE).unwrap();
        let first = &inputs[0];
        assert_eq!(first, &Input(Location(2, 18), Location(-2, 15)));
        assert_eq!(&inputs[1], &Input(Location(9, 16), Location(10, 16)));
    }

    #[test]
    fn test_solve_problem_1_example() {
        let map = parse_lines(EXAMPLE).unwrap();
        let score = solve_problem_one(&map, 10);
        assert_eq!(score, 26);
    }

    #[test]
    fn test_solve_problem_2_example() {
        let map = parse_lines(EXAMPLE).unwrap();
        let loc = find_distress_beacon(&map).unwrap();
        assert_eq!(loc, Location(14, 11));
    }
}
