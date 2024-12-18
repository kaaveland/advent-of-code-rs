use anyhow::{anyhow, Result};
use fxhash::FxHashSet;
use itertools::Itertools;
use std::collections::VecDeque;

type Pos = (i32, i32);

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum Tile {
    Wall,
    Open,
    Goblin,
    Elf,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum Team {
    Elves,
    Goblin,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Unit {
    team: Team,
    attack_points: i32,
    hit_points: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum Place {
    Open,
    Wall,
    Unit(Unit),
}

fn parse(input: &str) -> impl Iterator<Item = (Pos, Option<Tile>)> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, ch)| {
                (
                    (x as i32, y as i32),
                    match ch {
                        '#' => Some(Tile::Wall),
                        '.' => Some(Tile::Open),
                        'G' => Some(Tile::Goblin),
                        'E' => Some(Tile::Elf),
                        _ => None,
                    },
                )
            })
        })
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game {
    map: Vec<Place>,
    height: i32,
    width: i32,
}

const DIRS: [Pos; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let height = value.lines().count() as i32;
        let width = value.lines().next().unwrap().len() as i32;
        let mut map = vec![];
        for (pos, tile) in parse(value) {
            if let Some(tile) = tile {
                match tile {
                    Tile::Wall => {
                        map.push(Place::Wall);
                    }
                    Tile::Open => {
                        map.push(Place::Open);
                    }
                    team => {
                        map.push(Place::Unit(Unit {
                            attack_points: 3,
                            hit_points: 200,
                            team: match team {
                                Tile::Goblin => Team::Goblin,
                                Tile::Elf => Team::Elves,
                                _ => unreachable!(),
                            },
                        }));
                    }
                }
            } else {
                return Err(anyhow!("Invalid tile: at {pos:?}, {tile:?}"));
            }
        }
        Ok(Self { map, height, width })
    }
}

impl Game {
    fn contains(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.width) && (y >= 0 && y < self.height)
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if self.contains(x, y) {
            Some(x as usize + (y as usize * self.width as usize))
        } else {
            None
        }
    }

    fn swap(&mut self, pos1: Pos, pos2: Pos) {
        assert!(self.contains(pos1.0, pos1.1));
        assert!(self.contains(pos2.0, pos2.1));
        let idx1 = self.idx(pos1.0, pos1.1).unwrap();
        let idx2 = self.idx(pos2.0, pos2.1).unwrap();
        self.map.swap(idx1, idx2);
    }

    fn at(&self, pos: Pos) -> Option<&Place> {
        self.idx(pos.0, pos.1).and_then(|idx| self.map.get(idx))
    }

    fn at_mut(&mut self, pos: Pos) -> Option<&mut Place> {
        self.idx(pos.0, pos.1).and_then(|idx| self.map.get_mut(idx))
    }

    fn unit_locations(&self) -> Vec<Pos> {
        let mut locs: Vec<_> = (0..self.height)
            .cartesian_product(0..self.width)
            .filter_map(|(y, x)| {
                if let Some(Place::Unit(_)) = self.at((x, y)) {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect();
        locs.reverse();
        locs
    }

    fn round(&mut self) -> bool {
        let mut units = self.unit_locations();
        while let Some(pos) = units.pop() {
            // This unit may have died
            if let Some(Place::Unit(Unit { team, .. })) = self.at(pos) {
                if self.unit_round(pos, *team) {
                    // No more combat is possible
                    return true;
                }
            }
        }
        false
    }

    fn play(&mut self) -> i32 {
        let mut rounds = 0;
        while !self.round() {
            rounds += 1;
        }
        rounds
    }

    fn is_enemy(&self, pos: Pos, my_team: Team) -> bool {
        matches!(self.at(pos), Some(Place::Unit(Unit { team, .. })) if *team != my_team)
    }

    fn elves(&self) -> usize {
        self.map
            .iter()
            .filter(|place| match place {
                Place::Unit(Unit { team, .. }) => *team == Team::Elves,
                _ => false,
            })
            .count()
    }

    fn set_elf_ap(&mut self, ap: i32) {
        self.map.iter_mut().for_each(|place| match place {
            Place::Unit(Unit {
                team,
                attack_points,
                ..
            }) if *team == Team::Elves => {
                *attack_points = ap;
            }
            _ => {}
        })
    }

    fn can_move_to(&self, pos: Pos) -> bool {
        matches!(self.at(pos), Some(Place::Open))
    }

    fn find_next_move_for_unit(&self, me: Pos, team: Team) -> Pos {
        // do not move if enemies are within range
        if DIRS
            .iter()
            .any(|(dx, dy)| self.is_enemy((me.0 + dx, me.1 + dy), team))
        {
            return me;
        }

        let adjacent_to_enemies = (0..self.height)
            .cartesian_product(0..self.width)
            .filter(|(y, x)| self.is_enemy((*x, *y), team))
            .flat_map(|(y, x)| DIRS.iter().map(move |dir| (x + dir.0, y + dir.1)))
            .filter(|&pos| self.can_move_to(pos))
            .collect_vec();

        if let Some((_, _, first)) = adjacent_to_enemies
            .into_iter()
            .filter_map(|pos| {
                self.first_step_by_steps_needed(me, pos)
                    .map(|(steps, first)| (steps, (pos.1, pos.0), (first.1, first.0)))
            })
            .min()
            .map(|(steps, pos, first)| (steps, (pos.1, pos.0), (first.1, first.0)))
        {
            assert_eq!(self.at(first), Some(&Place::Open));
            first
        } else {
            me
        }
    }

    fn attack(&mut self, me: Pos, team: Team) {
        if let Some(target) = self.find_target(me, team) {
            if let Some(&Place::Unit(Unit { attack_points, .. })) = self.at(me) {
                if let Some(place) = self.at_mut(target) {
                    match place {
                        Place::Unit(Unit { hit_points, .. }) if *hit_points < attack_points => {
                            *place = Place::Open;
                        }
                        Place::Unit(Unit { hit_points, .. }) => {
                            *hit_points -= attack_points;
                        }
                        _ => panic!("Not a unit"),
                    }
                }
            } else {
                panic!("Not a unit");
            }
        }
    }

    fn unit_round(&mut self, me: Pos, team: Team) -> bool {
        // If this unit has no enemies, we end combat
        if (0..self.height)
            .cartesian_product(0..self.width)
            .any(|pos| self.is_enemy(pos, team))
        {
            let dest = self.find_next_move_for_unit(me, team);
            // Let the unit move
            if dest != me {
                self.swap(me, dest);
            }
            let me = dest;
            self.attack(me, team);
            false
        } else {
            true
        }
    }

    fn find_target(&self, me: Pos, my_team: Team) -> Option<Pos> {
        DIRS.iter()
            .map(|dir| (me.0 + dir.0, me.1 + dir.1))
            .filter_map(|pos| match self.at(pos) {
                Some(&Place::Unit(Unit {
                    team, hit_points, ..
                })) if team != my_team => Some((hit_points, (pos.1, pos.0))),
                _ => None,
            })
            .min()
            .map(|(_, pos)| (pos.1, pos.0))
    }

    fn first_step_by_steps_needed(&self, start_pos: Pos, end_pos: Pos) -> Option<(usize, Pos)> {
        let mut visited = FxHashSet::default();
        let mut work: VecDeque<(Pos, usize, Option<Pos>)> = VecDeque::new();
        work.push_back((start_pos, 0, None));
        while let Some((pos, steps, first_step)) = work.pop_front() {
            if pos == end_pos {
                // This is one shortest path. Since we're doing BFS, if there are shortest paths
                // othen than the one we found, they must have the same step count
                // take the minimum one by reading order
                work.push_front((pos, steps, first_step));
                return work
                    .into_iter()
                    .take_while(|(_, stepcount, _)| *stepcount == steps)
                    .filter(|(pos, _, _)| *pos == end_pos)
                    .min_by_key(|((x, y), _, first)| (*y, *x, first.unwrap().1, first.unwrap().0))
                    .map(|(_, stepcount, first)| (stepcount, first.unwrap()));
            }
            if visited.insert((pos, first_step)) {
                for dir in DIRS {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if self.can_move_to(new_pos) {
                        work.push_back((new_pos, steps + 1, first_step.or(Some(new_pos))));
                    }
                }
            }
        }
        None
    }

    fn remaining_hitpoints(&self) -> i32 {
        self.map
            .iter()
            .map(|place| match place {
                Place::Unit(Unit { hit_points, .. }) => *hit_points,
                _ => 0,
            })
            .sum()
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let mut game = Game::try_from(input)?;
    let rounds = game.play();
    let outcome = rounds * game.remaining_hitpoints();
    Ok(format!("{outcome}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let game = Game::try_from(input)?;
    let elves = game.elves();
    for ap in 4.. {
        let mut current_game = game.clone();
        current_game.set_elf_ap(ap);
        let mut rounds = 0;
        while !current_game.round() && current_game.elves() == elves {
            rounds += 1;
        }
        if current_game.elves() == elves {
            let outcome = rounds * current_game.remaining_hitpoints();
            return Ok(format!("{outcome}"));
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_order() {
        let ex = "#######
#.G.E.#
#E.G.E#
#.G.E.#
#######";
        let game = Game::try_from(ex).unwrap();
        assert_eq!(
            game.unit_locations(),
            vec![(4, 3), (2, 3), (5, 2), (3, 2), (1, 2), (4, 1), (2, 1)]
        );
    }

    #[test]
    fn test_find_first_step() {
        let game = Game::try_from(
            "#######
#.E...#
#.....#
#...G.#
#######",
        )
        .unwrap();
        let elf = (2, 2);
        assert_eq!(game.find_next_move_for_unit(elf, Team::Elves), (3, 2));
    }

    #[test]
    fn test_target_selection() {
        let mut game = Game::try_from(
            "G....
..G..
..EG.
..G..
...G.",
        )
        .unwrap();
        fn set_hp(game: &mut Game, pos: Pos, hp: i32) {
            let unit = game.at_mut(pos).unwrap();
            match unit {
                Place::Unit(Unit { hit_points, .. }) => *hit_points = hp,
                _ => panic!("Not a unit"),
            }
        }
        set_hp(&mut game, (0, 0), 9);
        set_hp(&mut game, (2, 1), 4);
        set_hp(&mut game, (3, 2), 2);
        set_hp(&mut game, (2, 3), 2);
        set_hp(&mut game, (3, 4), 1);
        assert_eq!(game.find_target((2, 2), Team::Elves), Some((3, 2)));
    }

    #[test]
    fn test_play() {
        let mut game = Game::try_from(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
        )
        .unwrap();
        assert_eq!(game.play(), 47);
    }

    #[test]
    fn test_p1() {
        let ex = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(part_1(ex).unwrap(), "27730");
    }

    #[test]
    fn test_first_bug() {
        let game = Game::try_from(
            "#######
#..G..#
#...EG#
#.#G#G#
#...#E#
#.....#
#######",
        )
        .unwrap();
        // goblin at 3, 1 should move to 4, 1
        assert_eq!(game.find_next_move_for_unit((3, 1), Team::Goblin), (4, 1));
    }

    #[test]
    fn test_example() {
        let ex = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        assert_eq!(part_1(ex).unwrap(), "18740");
    }
}
