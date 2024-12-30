use anyhow::anyhow;
use fxhash::FxHashSet;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, none_of, space0};
use nom::combinator::{map, map_res, recognize, success};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Group<'a> {
    units: i32,
    hit_points: i32,
    weakness: FxHashSet<String>,
    immunities: FxHashSet<String>,
    damage_type: &'a str,
    damage: i32,
    initiative: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum Kind {
    Immune,
    Weak,
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct ImmunityList<'a>(Kind, Vec<&'a str>);

fn parse_immunity_list(s: &str) -> IResult<&str, ImmunityList> {
    let (s, kind) = alt((
        map(tag("immune"), |_| Kind::Immune),
        map(tag("weak"), |_| Kind::Weak),
    ))(s)?;
    let (s, list) = preceded(
        tag(" to "),
        separated_list1(tag(", "), recognize(many1(none_of(";,)")))),
    )(s)?;
    Ok((s, ImmunityList(kind, list)))
}

struct ImmunityBlock<'a>(Vec<ImmunityList<'a>>);
fn parse_immunity_block(s: &str) -> IResult<&str, ImmunityBlock> {
    let block = alt((
        delimited(
            char('('),
            separated_list1(tag("; "), parse_immunity_list),
            char(')'),
        ),
        success(vec![]),
    ));
    map(block, ImmunityBlock)(s)
}

impl ImmunityBlock<'_> {
    fn take(&self, kind: &Kind) -> FxHashSet<String> {
        let mut out = FxHashSet::default();
        for list in &self.0 {
            if kind == &list.0 {
                out.extend(list.1.iter().map(|s| s.to_string()))
            }
        }
        out
    }
}

fn parse_group(s: &str) -> IResult<&str, Group> {
    let (s, units) = map_res(digit1, |n: &str| n.parse())(s)?;
    let (s, _) = tag(" units each with ")(s)?;
    let (s, hit_points) = map_res(digit1, |n: &str| n.parse())(s)?;
    let (s, _) = tag(" hit points ")(s)?;
    let (s, block) = parse_immunity_block(s)?;
    let weakness = block.take(&Kind::Weak);
    let immunities = block.take(&Kind::Immune);
    let (s, _) = preceded(space0, tag("with an attack that does "))(s)?;
    let (s, damage) = map_res(digit1, |n: &str| n.parse())(s)?;
    let (s, damage_type) = preceded(space0, recognize(many1(none_of(" "))))(s)?;
    let (s, initiative) = preceded(
        tag(" damage at initiative "),
        map_res(digit1, |n: &str| n.parse()),
    )(s)?;
    Ok((
        s,
        Group {
            units,
            hit_points,
            weakness,
            immunities,
            damage,
            damage_type,
            initiative,
        },
    ))
}

#[derive(Debug, Clone)]
struct Armies<'a> {
    immune_system: Vec<Group<'a>>,
    infection: Vec<Group<'a>>,
}

fn parse(s: &str) -> anyhow::Result<Armies> {
    let mut parser = separated_pair(
        preceded(
            tag("Immune System:\n"),
            separated_list1(char('\n'), parse_group),
        ),
        tag("\n\n"),
        preceded(
            tag("Infection:\n"),
            separated_list1(char('\n'), parse_group),
        ),
    );
    parser(s)
        .map_err(|err| anyhow!("{err}"))
        .map(|(_, (immune_system, infection))| Armies {
            immune_system,
            infection,
        })
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum GroupId {
    ImmuneSystem(usize),
    Infection(usize),
}

impl Group<'_> {
    fn effective_power(&self) -> i32 {
        0.max(self.units * self.damage)
    }
    fn damage_potential(&self, other: &Group) -> i32 {
        if other.immunities.contains(self.damage_type) {
            0
        } else if other.weakness.contains(self.damage_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }
}

impl Armies<'_> {
    fn ix(&self, group_id: &GroupId) -> &Group {
        match group_id {
            GroupId::Infection(ix) => &self.infection[*ix],
            GroupId::ImmuneSystem(ix) => &self.immune_system[*ix],
        }
    }
}

fn target_selection(armies: &Armies) -> Vec<(GroupId, GroupId)> {
    let immune_system = (0..armies.immune_system.len())
        .map(GroupId::ImmuneSystem)
        .collect_vec();
    let infection = (0..armies.infection.len())
        .map(GroupId::Infection)
        .collect_vec();

    let mut all_by_power = immune_system
        .iter()
        .chain(infection.iter())
        .sorted_by_key(|ix| {
            let g = armies.ix(ix);
            (g.effective_power(), g.initiative)
        })
        .collect_vec();

    let mut chosen = vec![];

    while let Some(attacker) = all_by_power.pop() {
        let a = armies.ix(attacker);
        let targets = if matches!(attacker, GroupId::Infection(_)) {
            &immune_system
        } else {
            &infection
        };
        let target = targets
            .iter()
            // Exclude targets that have already been selected
            .filter(|target| !chosen.iter().any(|(_, claimed)| claimed == *target))
            // Exclute if we can not damage the target
            .filter(|target| a.damage_potential(armies.ix(target)) > 0)
            // Which means maybe there's no target
            .max_by_key(|target| {
                let g = armies.ix(target);
                (a.damage_potential(g), g.effective_power(), g.initiative)
            });

        if let Some(claim) = target {
            if !chosen.iter().any(|(_attacker, target)| target == claim) {
                chosen.push((*attacker, *claim));
            }
        }
    }

    chosen
        .into_iter()
        .sorted_by_key(|(attackers, _)| -armies.ix(attackers).initiative)
        .collect()
}

fn attacking(armies: &mut Armies, targets: &[(GroupId, GroupId)]) {
    for (attacker, target) in targets {
        let damage = armies
            .ix(attacker)
            .damage_potential(armies.ix(target))
            .max(0);
        let hp_per_unit = armies.ix(target).hit_points;
        let dead = damage / hp_per_unit;
        match target {
            GroupId::Infection(i) => {
                armies.infection[*i].units = 0.max(armies.infection[*i].units - dead)
            }
            GroupId::ImmuneSystem(i) => {
                armies.immune_system[*i].units = 0.max(armies.immune_system[*i].units - dead)
            }
        }
    }
}

fn living_units(armies: &Armies) -> (i32, i32) {
    (
        armies.infection.iter().map(|g| g.units).sum(),
        armies.immune_system.iter().map(|g| g.units).sum(),
    )
}

fn fight_loop(armies: &Armies, boost: i32) -> (i32, i32) {
    let mut armies = armies.clone();
    let mut last_units = 0;
    armies
        .immune_system
        .iter_mut()
        .for_each(|group| group.damage += boost);
    loop {
        let (infection, immune_system) = living_units(&armies);
        // Apparently we can get into a situation where nobody can damage each other
        if infection + immune_system == last_units {
            return (-1, -1);
        } else {
            last_units = infection + immune_system;
        }
        if infection == 0 || immune_system == 0 {
            return (infection, immune_system);
        } else {
            let targets = target_selection(&armies);
            attacking(&mut armies, &targets);
            armies.immune_system.retain(|g| g.units > 0);
            armies.infection.retain(|g| g.units > 0);
        }
    }
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let armies = parse(s)?;
    let (infection, immune_system) = fight_loop(&armies, 0);
    Ok(infection.max(immune_system).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let armies = parse(s)?;
    for boost in 1.. {
        let (infection, immune_system) = fight_loop(&armies, boost);
        if immune_system > 0 && infection <= 0 {
            return Ok(immune_system.to_string());
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
";

    #[test]
    fn test_p1() {
        assert_eq!(part_1(EX).unwrap().as_str(), "5216");
    }

    #[test]
    fn test_target_selection() {
        let mut armies = parse(EX).unwrap();
        let targets = target_selection(&armies);
        assert_eq!(
            &targets,
            &vec![
                (GroupId::Infection(1), GroupId::ImmuneSystem(1)),
                (GroupId::ImmuneSystem(1), GroupId::Infection(0)),
                (GroupId::ImmuneSystem(0), GroupId::Infection(1)),
                (GroupId::Infection(0), GroupId::ImmuneSystem(0)),
            ]
        );
        attacking(&mut armies, &targets);
        assert_eq!(armies.ix(&GroupId::ImmuneSystem(0)).units, 0);
        assert_eq!(armies.ix(&GroupId::ImmuneSystem(1)).units, 905);
        assert_eq!(armies.ix(&GroupId::Infection(0)).units, 797);
        assert_eq!(armies.ix(&GroupId::Infection(1)).units, 4434);
    }

    #[test]
    fn test_parser() {
        assert!(parse(EX).is_ok());
    }

    #[test]
    fn test_parse_group() {
        let ex = "1173 units each with 32300 hit points (weak to cold, slashing) with an attack that does 53 bludgeoning damage at initiative 19";
        let (_, g) = parse_group(ex).unwrap();
        assert_eq!(g.initiative, 19);
        assert_eq!(
            g.weakness,
            FxHashSet::from_iter(vec!["cold".to_string(), "slashing".to_string()])
        );
        assert!(g.immunities.is_empty());
        let ex = "7006 units each with 11084 hit points with an attack that does 2 fire damage at initiative 2";
        let (_, g) = parse_group(ex).unwrap();
        assert!(g.weakness.is_empty());
        assert!(g.immunities.is_empty());
        let ex = "3712 units each with 12148 hit points (immune to cold; weak to slashing) with an attack that does 5 slashing damage at initiative 17";
        let (_, g) = parse_group(ex).unwrap();
        assert!(!g.weakness.is_empty());
        assert!(!g.immunities.is_empty());
    }
}
