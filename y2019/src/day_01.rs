use anyhow::Result;

pub fn part_1(input: &str) -> Result<String> {
    let fuel: u64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .map(|w: u64| w / 3 - 2)
        .sum();
    Ok(format!("{fuel}"))
}

fn fuel_required(mut weight: i64) -> i64 {
    let mut fuel = 0;
    while weight >= 0 {
        weight = weight / 3 - 2;
        fuel += weight.max(0);
    }
    fuel
}

pub fn part_2(input: &str) -> Result<String> {
    let fuel: i64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok())
        .map(fuel_required)
        .sum();
    Ok(format!("{fuel}"))
}
