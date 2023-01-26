use anyhow::{anyhow, Result};
use fxhash::{FxHashMap as HashMap, FxHashMap, FxHashSet as HashSet, FxHashSet};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, space0};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{Finish, IResult};

fn parse_food(i: &str) -> IResult<&str, &str> {
    delimited(space0, alpha1, space0)(i)
}
fn parse_allergen(i: &str) -> IResult<&str, &str> {
    let terminator = pair(complete::char(','), complete::char(' '));
    terminated(alpha1, many0(terminator))(i)
}

fn parse_allergen_list(i: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        complete::char('('),
        preceded(tag("contains "), many1(parse_allergen)),
        complete::char(')'),
    )(i)
}

fn parse_food_list(i: &str) -> IResult<&str, Vec<&str>> {
    many1(parse_food)(i)
}

fn parse_line(i: &str) -> Result<Food> {
    pair(parse_food_list, parse_allergen_list)(i)
        .finish()
        .map_err(|e| anyhow!("Unable to parse line due to {e:?}"))
        .map(|(_, (ingredients, allergens))| Food {
            ingredients: ingredients.into_iter().collect(),
            allergens: allergens.into_iter().collect(),
        })
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

fn eliminate(possibilities: &mut HashMap<&str, HashSet<&str>>, allergen: &str, ingredient: &str) {
    // Base case: ingredient is already eliminated as a source of allergen, it's not contained here
    // and we can just return
    if possibilities
        .get(allergen)
        .map(|set| set.contains(ingredient))
        == Some(true)
    {
        if let Some(ingredient_list) = possibilities.get_mut(allergen) {
            ingredient_list.retain(|source| *source != ingredient);
        }
        // We eliminated this choice; ingredient_list could be a single element now, in which case
        // we can eliminated it everywhere else
        if possibilities
            .get(allergen)
            .map(|ingredient_list| ingredient_list.len())
            == Some(1)
        {
            let choice = possibilities
                .get(allergen)
                .and_then(|ingredient_list| ingredient_list.iter().next().copied())
                .unwrap();
            for other_allergen in possibilities
                .keys()
                .copied()
                .filter(|other_allergen| *other_allergen != allergen)
                .collect_vec()
            {
                eliminate(possibilities, other_allergen, choice);
            }
        }
    }
}

fn solve_constraints<'a>(foods: &'a Vec<Food>) -> FxHashMap<&'a str, FxHashSet<&'a str>> {
    let mut allergens_to_sources: HashMap<&str, HashSet<&str>> = HashMap::default();

    // Establish candidates for all allergens
    for food in foods {
        for allergen in food.allergens.iter().copied() {
            if !allergens_to_sources.contains_key(allergen) {
                allergens_to_sources.insert(allergen, food.ingredients.iter().copied().collect());
            }
        }
    }

    // Eliminate impossible candidates
    for food in foods {
        for allergen in food.allergens.iter().copied() {
            // The ingredients that could've caused allergen, but are not present for this instance of
            // the allergen can't be the source
            let impossible = allergens_to_sources
                .get(allergen)
                .unwrap()
                .iter()
                .copied()
                .filter(|ingredient| !food.ingredients.contains(ingredient))
                .collect_vec();
            for impossible_source in impossible {
                eliminate(&mut allergens_to_sources, allergen, impossible_source);
            }
        }
    }

    allergens_to_sources
}

fn parse(input: &str) -> Result<Vec<Food>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

fn solve_1(input: &str) -> Result<usize> {
    let foods = parse(input)?;
    let allergen_sources = solve_constraints(&foods);
    let allergen_sources: HashSet<_> = allergen_sources
        .values()
        .flat_map(|ingredient_list| ingredient_list.iter().copied())
        .collect();

    let n = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|ingredient| !allergen_sources.contains(**ingredient))
        .count();
    Ok(n)
}

pub fn part_1(input: &str) -> Result<String> {
    solve_1(input).map(|n| format!("{n}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let foods = parse(input)?;
    let allergen_sources = solve_constraints(&foods);
    let mut allergen_sources = allergen_sources
        .iter()
        .sorted_by_key(|(k, _v)| **k)
        .flat_map(|(_k, v)| v.iter().copied());
    Ok(allergen_sources.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let n = solve_1(example).unwrap();
        assert_eq!(n, 5);
        assert_eq!(
            part_2(example).unwrap(),
            String::from("mxmxvkd,sqjhc,fvjkl")
        );
    }

    #[test]
    fn parse_input() {
        let line = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)";
        let result = parse_line(line);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result.ingredients,
            vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]
                .into_iter()
                .collect()
        );
        assert_eq!(
            result.allergens,
            vec!["dairy", "fish"].into_iter().collect()
        );
    }
}
