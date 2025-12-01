use anyhow::{anyhow, Result};
use fxhash::FxHashMap as HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::{character::complete::digit1, IResult};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Reaction<'a> {
    cost: HashMap<&'a str, i64>,
    produces: &'a str,
    volume: i64,
}

fn parse_mat(i: &str) -> IResult<&str, (&str, i64)> {
    let (i, amount) = map_res(digit1, FromStr::from_str)(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, material) = alpha1(i)?;
    Ok((i, (material, amount)))
}

fn parse_cost(i: &str) -> IResult<&str, HashMap<&str, i64>> {
    let (i, cost) = separated_list1(tag(", "), parse_mat)(i)?;
    Ok((i, cost.into_iter().collect()))
}

fn parse_reaction(i: &str) -> IResult<&str, Reaction<'_>> {
    let (i, cost) = parse_cost(i)?;
    let (i, _) = tag(" => ")(i)?;
    let (i, (produces, volume)) = parse_mat(i)?;
    Ok((
        i,
        Reaction {
            cost,
            produces,
            volume,
        },
    ))
}

fn parse(i: &str) -> Result<HashMap<&str, Reaction<'_>>> {
    let (_, reactions) =
        separated_list1(tag("\n"), parse_reaction)(i).map_err(|e| anyhow!("{e}"))?;
    Ok(reactions.into_iter().map(|r| (r.produces, r)).collect())
}

fn order_fuel(recipes: &HashMap<&str, Reaction>, amount: u32) -> i64 {
    let mut have: HashMap<&str, i64> = HashMap::default();
    let mut orders: VecDeque<(&str, i64)> = VecDeque::new();
    orders.push_back(("FUEL", amount as i64));
    let mut ores_used = 0;

    while let Some((material, amount)) = orders.pop_front() {
        if material == "ORE" {
            ores_used += amount;
        } else if have.get(material).copied().unwrap_or(0) >= amount {
            *have.entry(material).or_default() -= amount;
        } else {
            let recipe = recipes.get(material).unwrap();
            let got = have.get(material).copied().unwrap_or(0);
            have.insert(material, 0);
            let need = amount - got;
            let new_orders = if need % recipe.volume == 0 {
                need / recipe.volume
            } else {
                need / recipe.volume + 1
            };
            let output = new_orders * recipe.volume;
            let surplus = output - need;
            *have.entry(material).or_default() += surplus;
            for (subcomponent, count) in recipe.cost.iter() {
                orders.push_back((*subcomponent, *count * new_orders));
            }
        }
    }
    ores_used
}

fn solve_fuel_cost(input: &str, amount: u32) -> Result<i64> {
    let recipes = parse(input)?;
    Ok(order_fuel(&recipes, amount))
}

pub fn part_1(input: &str) -> Result<String> {
    solve_fuel_cost(input, 1).map(|n| format!("{n}"))
}

fn spend_max_ores(recipes: &HashMap<&str, Reaction>, ore_fund: i64) -> i64 {
    let mut fuel_order = 1;

    // Find the upper bound
    while order_fuel(recipes, fuel_order) < ore_fund {
        fuel_order *= 2;
    }
    // Now it's a binary search between fuel_order / 2 and fuel_order
    let mut lo = fuel_order / 2;
    let mut hi = fuel_order;

    loop {
        let mid = (lo + hi) / 2;
        let spent = order_fuel(recipes, mid);
        if mid == lo || mid == hi || spent == ore_fund {
            return mid as i64;
        } else if spent < ore_fund {
            lo = mid;
        } else if spent > ore_fund {
            hi = mid;
        }
    }
}

fn solve_max_spend(input: &str, ore_fund: i64) -> Result<i64> {
    let recipes = parse(input)?;
    Ok(spend_max_ores(&recipes, ore_fund))
}

pub fn part_2(input: &str) -> Result<String> {
    solve_max_spend(input, 1000000000000).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spend_trillion_ore() {
        assert_eq!(
            solve_max_spend(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
",
                1000000000000
            )
            .unwrap(),
            5586022
        );
    }
    #[test]
    fn test_small_ex() {
        assert_eq!(
            solve_fuel_cost(
                "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
",
                1
            )
            .unwrap(),
            165
        );
    }

    #[test]
    fn test_big_ex() {
        assert_eq!(
            solve_fuel_cost(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
",
                1
            )
            .unwrap(),
            180697
        );
    }
}
