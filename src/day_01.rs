use std::num::ParseIntError;
use anyhow::Result;

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
}

fn parse(input: &str) -> Result<Vec<i32>> {
    let r: Result<Vec<_>, ParseIntError> = input.lines().filter(|line| !line.is_empty())
        .map(str::parse::<i32>)
        .collect();
    let v = r?;
    Ok(v)
}

fn solve_1(depths: &Vec<i32>) -> usize {
    depths.iter().zip(depths.iter().skip(1))
        .filter(|(&before, &now)| now > before)
        .count()
}

pub fn part_1(input: &str) -> Result<()> {
    let depths = parse(input)?;
    let sol = solve_1(&depths);
    println!("{sol}");
    Ok(())
}