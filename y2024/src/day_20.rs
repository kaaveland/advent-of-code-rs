use anyhow::Context;
use fxhash::FxHashSet;
use std::collections::VecDeque;

type Pos = (i32, i32);

struct Maze {
    walls: FxHashSet<Pos>,
    height: i32,
    width: i32,
}

fn parse(input: &str) -> anyhow::Result<(Maze, Pos, Pos)> {
    let by_idx = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, ch)| (x as i32, y as i32, ch))
    });
    let height = input.lines().filter(|l| !l.is_empty()).count() as i32;
    let width = input.lines().map(|l| l.len()).max().unwrap() as i32;
    let start = by_idx
        .clone()
        .find_map(|(x, y, ch)| if ch == 'S' { Some((x, y)) } else { None })
        .context("Unable to find start loc")?;
    let end = by_idx
        .clone()
        .find_map(|(x, y, ch)| if ch == 'E' { Some((x, y)) } else { None })
        .context("Unable to find start loc")?;
    let walls = by_idx
        .filter_map(|(x, y, ch)| if ch == '#' { Some((x, y)) } else { None })
        .collect();
    Ok((
        Maze {
            walls,
            height,
            width,
        },
        start,
        end,
    ))
}

fn find_distances_from_pos(maze: &Maze, pos: Pos) -> (Vec<i32>, Vec<Vec<Option<Pos>>>) {
    let mut distances = vec![0; (maze.height * maze.width) as usize];
    let mut parents = vec![vec![None; maze.height as usize]; maze.width as usize];
    let mut work = VecDeque::new();
    let mut visited = FxHashSet::default();
    work.push_back((pos, 0, None));
    while let Some((pos, dist, parent)) = work.pop_front() {
        if !maze.walls.contains(&pos) && visited.insert(pos) {
            let (x, y) = pos;
            distances[(x + y * maze.width) as usize] = dist;
            if let Some(parent) = parent {
                parents[y as usize][x as usize] = Some(parent);
            }
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                work.push_back(((nx, ny), dist + 1, Some(pos)));
            }
        }
    }
    (distances, parents)
}

fn path_between(start: Pos, end: Pos, parents: &[Vec<Option<Pos>>]) -> Vec<Pos> {
    let mut path = vec![];
    let mut pos = end;
    while let Some(parent) = parents[pos.1 as usize][pos.0 as usize] {
        path.push(pos);
        pos = parent;
    }
    assert_eq!(pos, start);
    path.push(pos);
    path
}

fn manhattan_offsets(max_cheat_len: i32) -> Vec<(i32, i32, i32)> {
    let mut offsets = vec![];
    for dx in -max_cheat_len..=max_cheat_len {
        let dymax = max_cheat_len - dx.abs();
        for dy in -dymax..=dymax {
            offsets.push((dx, dy, dx.abs() + dy.abs()));
        }
    }
    offsets
}

fn enumerate_cheats(maze: &Maze, start: Pos, end: Pos, max_cheat_len: i32, mingain: i32) -> i32 {
    // Each tile reachable from end has the distance from the end in distances[y][x]
    let (distances, parents) = find_distances_from_pos(maze, end);
    let ybounds = 0..maze.height;
    let xbounds = 0..maze.width;
    let mut count = 0;
    let offsets = manhattan_offsets(max_cheat_len);

    for (start_x, start_y) in path_between(end, start, &parents) {
        let remaining_dist = distances[(start_x + maze.width * start_y) as usize];
        // Cheating means we can go to any tile within cheat_len manhattan distance
        // instead. If it is a reachable tile with a path to goal, there will be a
        // length in distances[y][x] that is less than i32::max and we can choose
        // to use that instead of whatever we would get, at a cost of cheat len
        if remaining_dist >= mingain {
            for (dx, dy, cheat_cost) in offsets.iter() {
                let (nx, ny) = (start_x + dx, start_y + dy);
                if xbounds.contains(&nx) && ybounds.contains(&ny) {
                    let dist = distances[(nx + ny * maze.width) as usize];
                    if dist == i32::MAX {
                        continue;
                    }
                    let cheat_gain = dist - cheat_cost - remaining_dist;
                    if cheat_gain >= mingain {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn count_cheats_of_size(
    input: &str,
    max_cheat_len: i32,
    cheat_gain_cutoff: i32,
) -> anyhow::Result<i32> {
    let (maze, start, end) = parse(input)?;
    Ok(enumerate_cheats(
        &maze,
        start,
        end,
        max_cheat_len,
        cheat_gain_cutoff,
    ))
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let great_cheats = count_cheats_of_size(input, 2, 100)?;
    Ok(format!("{}", great_cheats))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let great_cheats = count_cheats_of_size(input, 20, 100)?;
    Ok(format!("{}", great_cheats))
}
