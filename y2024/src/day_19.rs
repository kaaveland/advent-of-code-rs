use anyhow::{Context, Result};
use fxhash::FxHashMap;

fn parse(input: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let (towels, orders) = input.split_once("\n\n").context("No orders")?;
    let towels = towels.split(", ").collect();
    let orders = orders.lines().filter(|line| !line.is_empty()).collect();
    Ok((towels, orders))
}

fn can_make<'a>(
    order: &'a str,
    towels: &'a [&'a str],
    cache: &mut FxHashMap<&'a str, bool>,
) -> bool {
    if let Some(result) = cache.get(order) {
        *result
    } else if order.is_empty() {
        true
    } else {
        let result = towels
            .iter()
            .any(|&t| order.starts_with(t) && can_make(&order[t.len()..], towels, cache));
        cache.insert(order, result);
        result
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let (towels, orders) = parse(input)?;
    let mut cache = FxHashMap::default();
    let p1 = orders
        .iter()
        .filter(|order| can_make(order, &towels, &mut cache))
        .count();
    Ok(format!("{p1}"))
}

fn possible_designs<'a>(
    order: &'a str,
    towels: &'a [&'a str],
    cache: &mut FxHashMap<&'a str, usize>,
) -> usize {
    if let Some(result) = cache.get(order) {
        *result
    } else if order.is_empty() {
        1
    } else {
        let result = towels
            .iter()
            .filter(|&t| order.starts_with(t))
            .map(|t| possible_designs(&order[t.len()..], towels, cache))
            .sum();
        cache.insert(order, result);
        result
    }
}

pub fn part_2(input: &str) -> Result<String> {
    let (towels, orders) = parse(input)?;
    let mut cache = FxHashMap::default();
    let p2: usize = orders
        .iter()
        .map(|order| possible_designs(order, &towels, &mut cache))
        .sum();
    Ok(format!("{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_parse() {
        let (towels, orders) = parse(EXAMPLE).unwrap();
        assert_eq!(towels, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert_eq!(
            orders,
            vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]
        );
    }
}
