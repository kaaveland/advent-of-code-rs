use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::cmp::max;

type ResourceKind = usize;
type ResourceAmount = i32;

const RESOURCE_KINDS: ResourceKind = 4;
const ORE: ResourceKind = 0;
const CLAY: ResourceKind = 1;
const OBSIDIAN: ResourceKind = 2;
const GEODE: ResourceKind = 3;
const RESOURCE_NAMES: [ResourceKind; RESOURCE_KINDS] = [ORE, CLAY, OBSIDIAN, GEODE];

type Resources = [ResourceAmount; RESOURCE_KINDS];
type Blueprint = [Resources; RESOURCE_KINDS];
type CanBuild = [Option<ResourceAmount>; RESOURCE_KINDS];

fn new_resources() -> Resources {
    [0; RESOURCE_KINDS]
}

fn new_blueprint() -> Blueprint {
    [[0; RESOURCE_KINDS]; RESOURCE_KINDS]
}

fn new_can_build() -> CanBuild {
    [None; RESOURCE_KINDS]
}

fn resource_kind(name: ResourceKind, count: ResourceAmount) -> Resources {
    let mut resources: Resources = new_resources();
    resources[name] = count;
    resources
}
fn ore(count: ResourceAmount) -> Resources {
    resource_kind(ORE, count)
}

fn clay(count: ResourceAmount) -> Resources {
    resource_kind(CLAY, count)
}

fn obsidian(count: ResourceAmount) -> Resources {
    resource_kind(OBSIDIAN, count)
}

fn add(left: &Resources, right: &Resources) -> Resources {
    let mut out = new_resources();
    for name in RESOURCE_NAMES {
        out[name] = left[name] + right[name];
    }
    out
}

fn mul(resources: &Resources, factor: ResourceAmount) -> Resources {
    let mut out = new_resources();
    for name in RESOURCE_NAMES {
        out[name] = resources[name] * factor;
    }
    out
}

fn sub(resources: &Resources, amount: &Resources) -> Resources {
    add(resources, &mul(amount, -1))
}

fn produce(bank: &Resources, bots: &Resources, ticks: ResourceAmount) -> Resources {
    add(bank, &mul(bots, ticks))
}

fn blueprint(ore: Resources, clay: Resources, obsidian: Resources, geode: Resources) -> Blueprint {
    let mut bp = new_blueprint();
    bp[ORE] = ore;
    bp[CLAY] = clay;
    bp[OBSIDIAN] = obsidian;
    bp[GEODE] = geode;
    bp
}

fn parse_blueprint(bp: &str) -> Result<Blueprint> {
    let re = Regex::new(r"([0-9]+)")?;

    let matches: Result<Vec<_>, _> = re
        .captures_iter(bp)
        .map(|m| m.get(1).expect("Expected match"))
        .map(|m| m.as_str().parse::<i32>())
        .collect();
    let numbers = matches?;
    if numbers.len() != 6 {
        Err(anyhow!("Wrong number of resources: {:?}", numbers))
    } else {
        Ok(blueprint(
            ore(numbers[0]),
            ore(numbers[1]),
            add(&ore(numbers[2]), &clay(numbers[3])),
            add(&ore(numbers[4]), &obsidian(numbers[5])),
        ))
    }
}

fn parse_bp_line(bp: &str) -> Result<(i32, Blueprint)> {
    let mut parts = bp.split(':');
    let name_part = parts.next().context("Needed blueprint name")?;
    let id_s = name_part
        .split(' ')
        .last()
        .context("Needed blueprint name")?;
    let id = id_s.parse()?;
    let bp_part = parts.next().context("Needed blueprint values part")?;
    let bp = parse_blueprint(bp_part)?;
    Ok((id, bp))
}

fn parse_bps(bps: &str) -> Result<Vec<(i32, Blueprint)>> {
    let bps: Result<Vec<_>> = bps.lines().map(parse_bp_line).collect();
    bps
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    bank: Resources,
    bots: Resources,
    ticks: i32,
}

fn max_opt(
    exists: Option<ResourceAmount>,
    candidate: Option<ResourceAmount>,
) -> Option<ResourceAmount> {
    match exists {
        None => None,
        Some(rounds) => candidate.map(|c_rounds| max(rounds, c_rounds)),
    }
}

fn rounds_to_afford(bp: &Blueprint, bank: &Resources, bots: &Resources) -> CanBuild {
    let mut out = new_can_build();
    for bot_name in RESOURCE_NAMES {
        let mut can_build = Some(0);
        for component in RESOURCE_NAMES {
            // Component not required for this bot, or we can already afford it
            if bp[bot_name][component] == 0 || bank[component] > bp[bot_name][component] {
                can_build = max_opt(can_build, Some(0));
            } else if bots[component] > 0 {
                let missing_funds = bp[bot_name][component] - bank[component];
                let rounds = if missing_funds % bots[component] > 0 {
                    missing_funds / bots[component] + 1
                } else {
                    missing_funds / bots[component]
                };
                can_build = max_opt(can_build, Some(rounds));
            } else {
                can_build = None;
            }
        }
        out[bot_name] = can_build;
    }
    out
}

fn upper_bound(
    mut geodes_found: ResourceAmount,
    mut geode_bots: ResourceAmount,
    remaining_time: i32,
) -> ResourceAmount {
    for _ in 0..remaining_time {
        geodes_found += geode_bots;
        geode_bots += 1;
    }
    geodes_found
}

fn search(bp: &Blueprint, ticks: i32) -> i32 {
    let initial = State {
        bank: new_resources(),
        bots: ore(1),
        ticks: 0,
    };
    let mut stack = vec![initial];
    let mut best = 0;
    let mut max_cost_by_resource = new_resources();
    for bot_kind in RESOURCE_NAMES {
        for component in RESOURCE_NAMES {
            max_cost_by_resource[component] =
                max(max_cost_by_resource[component], bp[bot_kind][component]);
        }
    }
    while let Some(state) = stack.pop() {
        if state.ticks > ticks {
            panic!("Encountered expired state {:?}", state);
        }
        let remainder = ticks - state.ticks;
        let rounds = rounds_to_afford(bp, &state.bank, &state.bots);

        // Calculate the no more choices approach directly
        let guaranteed_geodes = state.bank[GEODE] + remainder * state.bots[GEODE];
        if guaranteed_geodes > best {
            best = guaranteed_geodes;
        }
        if remainder == 0 || upper_bound(state.bank[GEODE], state.bots[GEODE], remainder) < best {
            continue;
        }

        // Attempt to build some bots -- we can only build 1 at a time
        for &bot_kind in RESOURCE_NAMES.iter().rev() {
            if let Some(time_needed) = rounds[bot_kind] {
                if time_needed < remainder
                    && (state.bots[bot_kind] < max_cost_by_resource[bot_kind] || bot_kind == GEODE)
                {
                    let income = produce(&state.bank, &state.bots, time_needed + 1);
                    let expense = &bp[bot_kind];
                    stack.push(State {
                        bank: sub(&income, expense),
                        bots: add(&state.bots, &resource_kind(bot_kind, 1)),
                        ticks: state.ticks + time_needed + 1,
                    })
                }
            }
        }
    }

    best
}

pub fn part_1(input: &str) -> Result<String> {
    let blueprints = parse_bps(input)?;
    let qualities: i32 = blueprints
        .iter()
        .map(|&(id, bp)| search(&bp, 24) * id)
        .sum();
    Ok(format!("{qualities}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let blueprints = parse_bps(input)?;
    let qualities: i32 = blueprints
        .iter()
        .take(3)
        .map(|&(_, bp)| search(&bp, 32))
        .product();
    Ok(format!("{qualities}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_parse_bp() {
        let sample = "Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 4 ore and 19 obsidian.";
        let bp = parse_blueprint(sample).unwrap();

        assert_eq!(bp[ORE][ORE], 4);
        assert_eq!(bp[CLAY][ORE], 4);
        assert_eq!(bp[OBSIDIAN][ORE], 4);
        assert_eq!(bp[OBSIDIAN][CLAY], 12);
        assert_eq!(bp[GEODE][ORE], 4);
        assert_eq!(bp[GEODE][OBSIDIAN], 19);
    }

    #[test]
    fn test_parse_bps() {
        let bp = parse_bps(EXAMPLE).unwrap();
        assert_eq!(bp.len(), 2);
        assert_eq!(bp[0].0, 1);
    }

    #[test]
    fn test_rounds_to_afford() {
        let sample = "Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 4 ore and 19 obsidian.";
        let bp = parse_blueprint(sample).unwrap();
        let rounds = rounds_to_afford(&bp, &new_resources(), &add(&ore(1), &clay(1)));
        assert_eq!(rounds[ORE], Some(4));
        assert_eq!(rounds[CLAY], Some(4));
        assert_eq!(rounds[OBSIDIAN], Some(12));
        let rounds = rounds_to_afford(&bp, &new_resources(), &add(&ore(1), &obsidian(2)));
        assert_eq!(rounds[GEODE], Some(10));
    }

    #[test]
    fn test_example() {
        let blueprints = parse_bps(EXAMPLE).unwrap();
        let result = search(&blueprints[0].1, 24);
        assert_eq!(result, 9);
        let result = search(&blueprints[1].1, 24);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_example_part_2() {
        let blueprints = parse_bps(EXAMPLE).unwrap();
        let result = search(&blueprints[0].1, 32);
        assert_eq!(result, 56);
        let result = search(&blueprints[1].1, 32);
        assert_eq!(result, 62);
    }
}
