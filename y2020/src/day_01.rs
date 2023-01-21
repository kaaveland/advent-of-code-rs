use anyhow::{anyhow, Context};
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let nums: Result<HashSet<_>, _> = input.lines().map(str::parse::<i32>).collect();
    let nums = nums?;
    let x = nums
        .iter()
        .find(|&&n| nums.contains(&(2020 - n)))
        .with_context(|| anyhow!("Unable to find x"))?;
    let answer = *x * (2020 - *x);
    Ok(format!("{answer}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let nums: Result<HashSet<_>, _> = input.lines().map(str::parse::<i32>).collect();
    let nums = nums?;
    let (&x, &y) = nums
        .iter()
        .cartesian_product(nums.iter())
        .find(|&(x, y)| nums.contains(&(2020 - *x - *y)))
        .with_context(|| anyhow!("Unable to find x, y"))?;
    let z = 2020 - x - y;
    Ok(format!("{}", x * y * z))
}
