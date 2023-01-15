use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SectionRange(u32, u32);

fn new_section(start: u32, end: u32) -> SectionRange {
    if start <= end {
        SectionRange(start, end)
    } else {
        SectionRange(end, start)
    }
}

fn container_contains(containee: SectionRange, container: SectionRange) -> bool {
    let SectionRange(containee_start, containee_end) = containee;
    let SectionRange(container_start, container_end) = container;
    container_start <= containee_start && container_end >= containee_end
}

fn one_is_fully_contained(left: SectionRange, right: SectionRange) -> bool {
    container_contains(left, right) || container_contains(right, left)
}

fn parse_section(section: &str) -> Option<SectionRange> {
    let split: Vec<&str> = section.splitn(2, '-').collect();
    if let [left, right] = split[..] {
        let start: u32 = left.parse().ok()?;
        let end: u32 = right.parse().ok()?;
        Some(new_section(start, end))
    } else {
        None
    }
}

fn parse_sections(line: &str) -> Option<(SectionRange, SectionRange)> {
    let split: Vec<&str> = line.splitn(2, ',').collect();
    if let [left, right] = split[..] {
        let left_sec = parse_section(left)?;
        let right_sec = parse_section(right)?;
        Some((left_sec, right_sec))
    } else {
        None
    }
}

fn part1_predicate(line: &str) -> Option<bool> {
    let (left_sec, right_sec) = parse_sections(line)?;
    Some(one_is_fully_contained(left_sec, right_sec))
}

fn predicate_count<'a, I, F>(lines: I, pred: F) -> u32
where
    I: IntoIterator<Item = &'a str>,
    F: Fn(&str) -> Option<bool>,
{
    let mut count = 0;
    for x in lines {
        if let Some(true) = pred(x) {
            count += 1;
        }
    }
    count
}

fn overlaps(left: SectionRange, right: SectionRange) -> bool {
    let SectionRange(left_start, left_end) = left;
    let SectionRange(right_start, right_end) = right;
    left_start <= right_end && left_end >= right_start
}

fn part2_predicate(line: &str) -> Option<bool> {
    let (left, right) = parse_sections(line)?;
    Some(overlaps(left, right))
}

pub fn part_1(input: &str) -> Result<String> {
    let sol = predicate_count(
        input.lines().filter(|line| !line.is_empty()),
        part1_predicate,
    );
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let sol = predicate_count(
        input.lines().filter(|line| !line.is_empty()),
        part2_predicate,
    );
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    #[test]
    fn test_new_section() {
        let sr = new_section(4, 2);
        let SectionRange(start, end) = sr;
        assert_eq!(start, 2);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_fully_contained() {
        assert!(!container_contains(new_section(2, 4), new_section(1, 3)),);
        assert!(container_contains(new_section(2, 4), new_section(2, 4)),);
        assert!(container_contains(new_section(2, 4), new_section(1, 5)),);
        assert!(!container_contains(new_section(1, 5), new_section(2, 4)),);
    }

    #[test]
    fn test_one_contains() {
        assert!(one_is_fully_contained(new_section(1, 5), new_section(2, 4)),);
        assert!(!one_is_fully_contained(
            new_section(1, 5),
            new_section(3, 10)
        ));
    }

    #[test]
    fn test_parse_section() {
        assert_eq!(parse_section("2-4"), Some(new_section(2, 4)));
        assert_eq!(parse_section("4-2"), Some(new_section(2, 4)));
        assert_eq!(parse_section("2-"), None);
        assert_eq!(parse_section("a-b"), None);
    }

    #[test]
    fn test_parse_sections() {
        assert_eq!(
            parse_sections("2-4,5-7"),
            Some((new_section(2, 4), new_section(5, 7)))
        );
        assert_eq!(parse_sections("2-"), None);
        assert_eq!(parse_sections("2,5"), None);
        assert_eq!(parse_section("2-4,5"), None);
    }

    #[test]
    fn test_part1_example() {
        let lines: Vec<&str> = EXAMPLE.split('\n').collect();
        assert_eq!(predicate_count(lines, part1_predicate), 2);
    }

    #[test]
    fn test_part2_predicate() {
        assert_eq!(part2_predicate("2-4,3-5"), Some(true));
        assert_eq!(part2_predicate("3-5,2-4"), Some(true));
        assert_eq!(part2_predicate("2-4,2-4"), Some(true));
        assert_eq!(part2_predicate("1-4,5-9"), Some(false));
        assert_eq!(part2_predicate("1-4,5"), None);
        assert_eq!(part2_predicate("1,3-5"), None);
        assert_eq!(part2_predicate(""), None);
    }

    #[test]
    fn test_part_2_example() {
        let lines: Vec<&str> = EXAMPLE.split('\n').collect();
        assert_eq!(predicate_count(lines, part2_predicate), 4);
    }
}
