use anyhow::Context;
use fxhash::{FxHashMap, FxHashSet};
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

fn find_distances_from_pos(maze: &Maze, pos: Pos) -> (Vec<Vec<i32>>, Vec<Vec<Option<Pos>>>) {
    let mut distances = vec![vec![i32::MAX; maze.height as usize]; maze.width as usize];
    let mut parents = vec![vec![None; maze.height as usize]; maze.width as usize];
    let mut work = VecDeque::new();
    let mut visited = FxHashSet::default();
    work.push_back((pos, 0, None));
    while let Some((pos, dist, parent)) = work.pop_front() {
        if !maze.walls.contains(&pos) && visited.insert(pos) {
            let (x, y) = pos;
            distances[y as usize][x as usize] = dist;
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

fn enumerate_cheats(maze: &Maze, start: Pos, end: Pos, max_cheat_len: i32) -> FxHashMap<i32, i32> {
    // Each tile reachable from end has the distance from the end in distances[y][x]
    let (distances, parents) = find_distances_from_pos(maze, end);
    // These are the locations we can reach from end, so the location where a cheat can start
    let ybounds = 0..maze.height;
    let xbounds = 0..maze.width;
    let mut cheats_by_dist = FxHashMap::default();

    for (start_x, start_y) in path_between(end, start, &parents) {
        let remaining_dist = distances[start_y as usize][start_x as usize];
        // Cheating means we can go to any tile within cheat_len manhattan distance
        // instead. If it is a reachable tile with a path to goal, there will be a
        // length in distances[y][x] that is less than i32::max and we can choose
        // to use that instead of whatever we would get, at a cost of cheat len
        for dx in -max_cheat_len..=max_cheat_len {
            let allowed_dy = max_cheat_len - dx.abs();
            for dy in -allowed_dy..=allowed_dy {
                let (nx, ny) = (start_x + dx, start_y + dy);
                if xbounds.contains(&nx) && ybounds.contains(&ny) && !maze.walls.contains(&(nx, ny))
                {
                    let cheat_cost = manhattan_dist((start_x, start_y), (nx, ny));
                    let dist = distances[ny as usize][nx as usize];
                    let cheat_gain = remaining_dist - dist + cheat_cost;
                    *cheats_by_dist.entry(cheat_gain).or_insert(0) += 1;
                }
            }
        }
    }
    cheats_by_dist
}

fn count_cheats_of_size(
    input: &str,
    max_cheat_len: i32,
    cheat_gain_cutoff: i32,
) -> anyhow::Result<i32> {
    let (maze, start, end) = parse(input)?;
    let cheats = enumerate_cheats(&maze, start, end, max_cheat_len);
    let mut great_cheats = 0;
    for (saved, cheats_found) in cheats {
        let saved = -saved;
        if saved >= cheat_gain_cutoff {
            great_cheats += cheats_found;
        }
    }
    Ok(great_cheats)
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let great_cheats = count_cheats_of_size(input, 2, 100)?;
    Ok(format!("{}", great_cheats))
}

fn manhattan_dist(a: Pos, b: Pos) -> i32 {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let great_cheats = count_cheats_of_size(input, 20, 100)?;
    Ok(format!("{}", great_cheats))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn verify_bfs() -> anyhow::Result<()> {
        let (maze, start, end) = parse(EXAMPLE)?;
        let (dist, parents) = find_distances_from_pos(&maze, start);
        assert_eq!(dist[end.1 as usize][end.0 as usize], 84);
        // Path includes start, it visits 85 places
        assert_eq!(path_between(start, end, &parents).len(), 85);
        Ok(())
    }

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let (maze, start, end) = parse(EXAMPLE)?;
        let cheats = enumerate_cheats(&maze, start, end, 2);
        let cheats: FxHashMap<_, _> = cheats
            .into_iter()
            .filter(|(saved, _)| *saved < 0)
            .map(|(dist, cheats_found)| (-dist, cheats_found))
            .collect();
        let expect = vec![
            (64, 1),
            (40, 1),
            (38, 1),
            (36, 1),
            (20, 1),
            (12, 3),
            (10, 2),
            (8, 4),
            (6, 2),
            (4, 14),
            (2, 14),
        ]
        .into_iter()
        .collect();
        assert_eq!(cheats, expect);
        Ok(())
    }
}
