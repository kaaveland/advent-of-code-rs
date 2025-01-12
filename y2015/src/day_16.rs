use anyhow::{anyhow, Context};

#[derive(Default, Copy, Clone)]
struct Sue {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizlas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

fn compatible_sue(sue: &Sue) -> bool {
    sue.children.unwrap_or(3) == 3
        && sue.cats.unwrap_or(7) == 7
        && sue.samoyeds.unwrap_or(2) == 2
        && sue.pomeranians.unwrap_or(3) == 3
        && sue.akitas.unwrap_or(0) == 0
        && sue.vizlas.unwrap_or(0) == 0
        && sue.goldfish.unwrap_or(5) == 5
        && sue.trees.unwrap_or(3) == 3
        && sue.cars.unwrap_or(2) == 2
        && sue.perfumes.unwrap_or(1) == 1
}

fn parse(s: &str) -> anyhow::Result<Vec<Sue>> {
    let mut sues = Vec::with_capacity(500);
    for line in s.lines() {
        let mut sue = Sue::default();
        let (_, owns) = line.split_once(": ").context("Malformed sue")?;
        for item_count in owns.split(", ") {
            let (label, count) = item_count.split_once(": ").context("Malformed items")?;
            let count = Some(count.parse::<u8>()?);
            match label {
                "children" => sue.children = count,
                "cats" => sue.cats = count,
                "samoyeds" => sue.samoyeds = count,
                "pomeranians" => sue.pomeranians = count,
                "akitas" => sue.akitas = count,
                "vizslas" => sue.vizlas = count,
                "goldfish" => sue.goldfish = count,
                "trees" => sue.trees = count,
                "cars" => sue.cars = count,
                "perfumes" => sue.perfumes = count,
                _ => return Err(anyhow!("Unknown item: {label}")),
            }
        }
        sues.push(sue);
    }
    Ok(sues)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    for (index, sue) in parse(s)?.into_iter().enumerate() {
        if compatible_sue(&sue) {
            return Ok((index + 1).to_string());
        }
    }
    Err(anyhow!("Failed to find aunt sue"))
}

fn real_aunt_sue(sue: &Sue) -> bool {
    sue.children.unwrap_or(3) == 3
        && sue.cats.map(|cats| cats > 7).unwrap_or(true)
        && sue.samoyeds.unwrap_or(2) == 2
        && sue
            .pomeranians
            .map(|pomeranians| pomeranians < 3)
            .unwrap_or(true)
        && sue.akitas.unwrap_or(0) == 0
        && sue.vizlas.unwrap_or(0) == 0
        && sue.goldfish.map(|goldfish| goldfish < 5).unwrap_or(true)
        && sue.trees.map(|trees| trees > 3).unwrap_or(true)
        && sue.cars.unwrap_or(2) == 2
        && sue.perfumes.unwrap_or(1) == 1
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    for (index, sue) in parse(s)?.into_iter().enumerate() {
        if real_aunt_sue(&sue) {
            return Ok((index + 1).to_string());
        }
    }
    Err(anyhow!("Failed to find aunt sue"))
}
