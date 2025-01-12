use anyhow::anyhow;
use regex::Regex;

type Item = [i32; 3];

const WEAPONS: [Item; 5] = [[8, 4, 0], [10, 5, 0], [25, 6, 0], [40, 7, 0], [74, 8, 0]];

const ARMOR: [Item; 6] = [
    [0, 0, 0],
    [13, 0, 1],
    [31, 0, 2],
    [53, 0, 3],
    [75, 0, 4],
    [102, 0, 5],
];

const RINGS: [Item; 6] = [
    [25, 1, 0],
    [50, 2, 0],
    [100, 3, 0],
    [20, 0, 1],
    [40, 0, 2],
    [80, 0, 3],
];

#[derive(Copy, Clone, Debug, Default)]
struct Character {
    hitpoints: i32,
    armor: i32,
    damage: i32,
}

fn parse_boss(s: &str) -> anyhow::Result<Character> {
    let numbers = Regex::new(r"\d+")?;
    let mut stats = [0; 3];
    for (g, m) in numbers.find_iter(s).enumerate() {
        stats[g] = m.as_str().parse().unwrap();
    }
    Ok(Character {
        hitpoints: stats[0],
        damage: stats[1],
        armor: stats[2],
    })
}

fn ring_selectors() -> impl Iterator<Item = usize> {
    (0..(1usize << RINGS.len())).filter(|selector| selector.count_ones() < 3)
}

fn collect_rings(selector: usize) -> Item {
    assert!(selector.count_ones() < 3);
    let mut out = [0; 3];
    for ix in 0..RINGS.len() {
        if selector & (1 << ix) > 0 {
            out.iter_mut()
                .zip(RINGS[ix])
                .for_each(|(stat, ring_stat)| *stat += ring_stat);
        }
    }
    out
}

fn all_loadouts() -> impl Iterator<Item = Item> {
    WEAPONS.into_iter().flat_map(|weapon| {
        ARMOR.into_iter().flat_map(move |armor| {
            ring_selectors().map(collect_rings).map(move |rings| {
                [
                    weapon[0] + armor[0] + rings[0],
                    weapon[1] + armor[1] + rings[1],
                    weapon[2] + armor[2] + rings[2],
                ]
            })
        })
    })
}

impl Character {
    fn new(hitpoints: i32, items: Item) -> Self {
        Self {
            hitpoints,
            damage: items[1],
            armor: items[2],
        }
    }
    fn wins(&self, other: &Character) -> bool {
        let my_damage_per_attack = (self.damage - other.armor).max(1);
        let mut my_turns_to_kill = other.hitpoints / my_damage_per_attack;
        if other.hitpoints > my_turns_to_kill * my_damage_per_attack {
            my_turns_to_kill += 1;
        }
        let their_damage_per_attack = (other.damage - self.armor).max(1);
        let mut their_turns_to_kill = self.hitpoints / their_damage_per_attack;
        if self.hitpoints > their_turns_to_kill * their_damage_per_attack {
            their_turns_to_kill += 1;
        }
        my_turns_to_kill <= their_turns_to_kill
    }
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let boss = parse_boss(s)?;
    if let Some(cost) = all_loadouts()
        .filter(|items| Character::new(100, *items).wins(&boss))
        .map(|items| items[0])
        .min()
    {
        Ok(cost.to_string())
    } else {
        Err(anyhow!("Can't win"))
    }
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let boss = parse_boss(s)?;
    if let Some(cost) = all_loadouts()
        .filter(|items| !Character::new(100, *items).wins(&boss))
        .map(|items| items[0])
        .max()
    {
        Ok(cost.to_string())
    } else {
        Err(anyhow!("Can't lose"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_rings_setup() {
        // no rings + 6 choose 1 = 6 + 6 choose 2 = 15 + 6
        assert_eq!(ring_selectors().count(), 1 + 6 + 15);
        assert_eq!(collect_rings(0), [0, 0, 0]);
        assert_eq!(collect_rings(1 | (1 << 1)), [75, 3, 0]);
    }

    #[test]
    fn example() {
        let boss = Character::new(12, [0, 7, 2]);
        let me = Character::new(8, [0, 5, 5]);
        assert!(me.wins(&boss));
    }
}
