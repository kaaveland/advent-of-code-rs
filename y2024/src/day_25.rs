use fxhash::FxHashSet;
use itertools::Itertools;

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let locks_keys = s
        .split("\n\n")
        .map(|l| {
            l.lines()
                .enumerate()
                .flat_map(move |(y, l)| {
                    l.chars()
                        .enumerate()
                        .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
                })
                .collect::<FxHashSet<(usize, usize)>>()
        })
        .collect_vec();

    let lock_places = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)];

    let is_lock = |maybe_lock: &FxHashSet<(usize, usize)>| {
        lock_places.iter().all(|lock| maybe_lock.contains(lock))
    };

    let (locks, keys): (Vec<_>, Vec<_>) = locks_keys.into_iter().partition(is_lock);
    let n = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(l, k)| l.intersection(k).count() == 0)
        .count();
    Ok(n.to_string())
}
