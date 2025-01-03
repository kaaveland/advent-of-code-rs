use anyhow::Context;

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(s.chars()
        .fold(0, |floor, ch| {
            floor + i32::from(ch == '(') - i32::from(ch == ')')
        })
        .to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    s.chars()
        .scan(1i32, |floor, ch| {
            *floor += i32::from(ch == '(') - i32::from(ch == ')');
            Some(*floor)
        })
        .enumerate()
        .find(|(_, floor)| *floor < 0)
        .map(|(ix, _)| ix.to_string())
        .context("Unable to find basement")
}
