use anyhow::{anyhow, Context};
use fxhash::FxHashSet;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State {
    mana_spent: i32,
    player_hp: i32,
    boss_hp: i32,
    boss_damage: i32,
    player_mana: i32,
    shield_effect: i32,
    poison_effect: i32,
    recharge_effect: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Status {
    PlayerWon(i32),
    Fighting(State),
    BossWon,
}

impl Status {
    fn boss_turn(&self) -> Self {
        match self {
            Status::Fighting(s) => s.boss_turn(),
            s => *s,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn mana_cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

impl State {
    fn apply_effects(&self) -> Self {
        let poison_active = self.poison_effect > 0;
        let recharge_active = self.recharge_effect > 0;

        Self {
            player_mana: if recharge_active {
                self.player_mana + 101
            } else {
                self.player_mana
            },
            boss_hp: if poison_active {
                self.boss_hp - 3
            } else {
                self.boss_hp
            },
            shield_effect: 0.max(self.shield_effect - 1),
            poison_effect: 0.max(self.poison_effect - 1),
            recharge_effect: 0.max(self.recharge_effect - 1),
            ..*self
        }
    }
    fn boss_turn(&self) -> Status {
        let mut next = self.apply_effects();
        if next.boss_hp <= 0 {
            Status::PlayerWon(self.mana_spent)
        } else {
            let shield_active = next.shield_effect > 0;
            next.player_hp -= if shield_active {
                1.max(self.boss_damage - 7)
            } else {
                self.boss_damage
            };
            if next.player_hp <= 0 {
                Status::BossWon
            } else {
                Status::Fighting(next)
            }
        }
    }

    fn apply_spell<F>(&self, mana_cost: i32, effect: F) -> Status
    where
        F: Fn(&State) -> State,
    {
        let next = State {
            mana_spent: self.mana_spent + mana_cost,
            player_mana: self.player_mana - mana_cost,
            ..effect(self)
        };
        if next.boss_hp <= 0 {
            Status::PlayerWon(next.mana_spent)
        } else {
            Status::Fighting(next)
        }
    }

    fn cast_spell(&self, spell: Spell) -> Option<Status> {
        use Spell::*;

        let mana_cost = spell.mana_cost();

        if self.player_mana >= mana_cost {
            match spell {
                MagicMissile => Some(self.apply_spell(mana_cost, |st| State {
                    boss_hp: st.boss_hp - 4,
                    ..*st
                })),
                Drain => Some(self.apply_spell(mana_cost, |st| State {
                    boss_hp: st.boss_hp - 2,
                    player_hp: st.player_hp + 2,
                    ..*st
                })),
                Shield if self.shield_effect < 1 => Some(self.apply_spell(mana_cost, |st| State {
                    shield_effect: 6,
                    ..*st
                })),
                Poison if self.poison_effect < 1 => Some(self.apply_spell(mana_cost, |st| State {
                    poison_effect: 6,
                    ..*st
                })),
                Recharge if self.recharge_effect < 1 => {
                    Some(self.apply_spell(mana_cost, |st| State {
                        recharge_effect: 5,
                        ..*st
                    }))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn turn(&self, hard_mode: bool) -> impl Iterator<Item = Status> {
        use Spell::*;

        let mut next = Vec::with_capacity(5);
        let mut now = self.apply_effects();
        if hard_mode {
            now.player_hp -= 1;
        }
        if now.player_hp <= 0 {
            next.push(Status::BossWon)
        } else if now.boss_hp <= 0 {
            next.push(Status::PlayerWon(now.mana_spent))
        } else {
            let spells = [MagicMissile, Drain, Shield, Poison, Recharge];
            for spell in spells {
                if let Some(casted) = now.cast_spell(spell) {
                    next.push(casted);
                }
            }
        }
        // The player couldn't afford to cast any spells, which is a loss condition
        if next.is_empty() {
            next.push(Status::BossWon)
        }

        next.into_iter().map(|st| st.boss_turn())
    }
}

fn initial_state(s: &str) -> anyhow::Result<State> {
    let re = Regex::new(
        r"Hit Points: (\d+)
Damage: (\d+)",
    )?;
    let m = re.captures(s).context("No match")?;
    Ok(State {
        mana_spent: 0,
        player_hp: 50,
        boss_hp: m.get(1).unwrap().as_str().parse()?,
        boss_damage: m.get(2).unwrap().as_str().parse()?,
        player_mana: 500,
        shield_effect: 0,
        poison_effect: 0,
        recharge_effect: 0,
    })
}

fn find_cheapest_victory(initial: State, hard_mode: bool) -> Option<i32> {
    let mut work = BinaryHeap::new();
    let mut seen = FxHashSet::default();
    work.push(Reverse(Status::Fighting(initial)));

    while let Some(Reverse(current)) = work.pop() {
        match current {
            Status::PlayerWon(cost) => {
                return Some(cost);
            }
            Status::BossWon => {}
            Status::Fighting(st) => {
                for next in st.turn(hard_mode) {
                    if seen.insert(next) {
                        work.push(Reverse(next));
                    }
                }
            }
        }
    }

    None
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    if let Some(cost) = find_cheapest_victory(initial_state(s)?, false) {
        Ok(cost.to_string())
    } else {
        Err(anyhow!("No victory found"))
    }
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    if let Some(cost) = find_cheapest_victory(initial_state(s)?, true) {
        Ok(cost.to_string())
    } else {
        Err(anyhow!("No victory found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_examples() {
        let initial = State {
            mana_spent: 0,
            player_hp: 10,
            boss_hp: 13,
            boss_damage: 8,
            player_mana: 250,
            shield_effect: 0,
            poison_effect: 0,
            recharge_effect: 0,
        };
        let next = initial.cast_spell(Spell::Poison);
        assert!(next.is_some());
        let next = next.unwrap().boss_turn();
        assert_eq!(
            next,
            Status::Fighting(State {
                mana_spent: Spell::Poison.mana_cost(),
                player_hp: 2,
                boss_hp: 10,
                boss_damage: 8,
                player_mana: 77,
                shield_effect: 0,
                poison_effect: 5,
                recharge_effect: 0,
            })
        );
        if let Status::Fighting(state) = next {
            let next = state.apply_effects().cast_spell(Spell::MagicMissile);
            assert!(next.is_some());
            let next = next.unwrap();
            assert_eq!(
                next,
                Status::Fighting(State {
                    mana_spent: Spell::Poison.mana_cost() + Spell::MagicMissile.mana_cost(),
                    player_hp: 2,
                    boss_hp: 3,
                    boss_damage: 8,
                    player_mana: 24,
                    shield_effect: 0,
                    poison_effect: 4,
                    recharge_effect: 0
                })
            );
            assert_eq!(
                next.boss_turn(),
                Status::PlayerWon(Spell::Poison.mana_cost() + Spell::MagicMissile.mana_cost())
            );
        } else {
            unreachable!()
        }
    }
}
