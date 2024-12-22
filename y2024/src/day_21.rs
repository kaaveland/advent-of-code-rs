use fxhash::FxHashMap;
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::once;

const KEYPAD: &str = "789
456
123
#0A";

const DIRPAD: &str = "#^A
<v>";

const DIRS: [(char, (i32, i32)); 4] =
    [('v', (0, 1)), ('^', (0, -1)), ('<', (-1, 0)), ('>', (1, 0))];

fn calc_neighbour_list(inp: &str) -> FxHashMap<char, Vec<(char, char)>> {
    let mut charmap = FxHashMap::default();
    for (y, line) in inp.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '#' {
                charmap.insert((x as i32, y as i32), ch);
            }
        }
    }
    let mut neighbour_list = FxHashMap::default();
    for ((x, y), ch) in charmap.iter() {
        for (dir, (dx, dy)) in DIRS {
            let (nx, ny) = (x + dx, y + dy);
            if let Some(neighbour) = charmap.get(&(nx, ny)) {
                neighbour_list
                    .entry(*ch)
                    .or_insert(vec![])
                    .push((*neighbour, dir));
            }
        }
    }
    neighbour_list
}

fn shortest_paths(
    neighbour_list: &FxHashMap<char, Vec<(char, char)>>,
    start: char,
    end: char,
) -> Vec<String> {
    let mut work = VecDeque::new();
    work.push_back((start, String::new()));
    let mut paths = vec![];
    let mut found = false;
    let mut path_len = 0;
    while let Some((location, path)) = work.pop_front() {
        if found && path.len() > path_len {
            break;
        } else if location == end {
            found = true;
            path_len = path.as_str().chars().count();
            paths.push(path);
        } else {
            for (neighbour, dir) in neighbour_list.get(&location).unwrap() {
                let mut new_path = path.clone();
                new_path.push(*dir);
                work.push_back((*neighbour, new_path));
            }
        }
    }
    paths
}

fn all_pairs_shortest_paths(
    neighbour_list: &FxHashMap<char, Vec<(char, char)>>,
) -> FxHashMap<(char, char), Vec<String>> {
    let mut paths = FxHashMap::default();
    for (start, end) in neighbour_list
        .keys()
        .cartesian_product(neighbour_list.keys())
    {
        paths.insert((*start, *end), shortest_paths(neighbour_list, *start, *end));
    }
    paths
}

fn min_keypresses(
    depth: usize,
    desired_output: String,
    keypad: &FxHashMap<(char, char), Vec<String>>,
    dirpad: &FxHashMap<(char, char), Vec<String>>,
    cache: &mut FxHashMap<(String, usize, bool), usize>,
    use_keypad: bool,
) -> usize {
    // TODO: Surely it's possible to look up without giving away ownership of the String??
    if let Some(ans) = cache.get(&(desired_output.clone(), depth, use_keypad)) {
        *ans
    } else {
        let chars = once('A').chain(desired_output.chars());
        // This kind of seems upside-down: depth = 2 or depth = 25 is the keypad
        let path_cache = if use_keypad { keypad } else { dirpad };
        // Recur with the cache for all pairs of buttons, ensuring we only ever look at short
        // paths (one segment at a time at all recursion levels)
        let ans = chars
            .clone()
            .zip(chars.skip(1))
            .map(|(a, b)| {
                let shortest_paths = path_cache.get(&(a, b)).unwrap();
                if depth == 0 {
                    // This is the top level, nobody needs to recur our shortest paths, they only
                    // need to know the length
                    shortest_paths[0].len() + 1
                } else {
                    shortest_paths
                        .iter()
                        .cloned()
                        .map(|mut path| {
                            path.push('A');
                            min_keypresses(depth - 1, path, keypad, dirpad, cache, false)
                        })
                        .min()
                        .unwrap()
                }
            })
            .sum();
        cache.insert((desired_output, depth, use_keypad), ans);
        ans
    }
}

fn complexity_score(
    code: &str,
    depth: usize,
    keypad: &FxHashMap<(char, char), Vec<String>>,
    dirpad: &FxHashMap<(char, char), Vec<String>>,
    cache: &mut FxHashMap<(String, usize, bool), usize>,
) -> anyhow::Result<usize> {
    let len = min_keypresses(depth, code.to_string(), keypad, dirpad, cache, true);
    let num: usize = code[..code.len() - 1].parse()?;
    Ok(num * len)
}

fn calc(inp: &str, depth: usize) -> anyhow::Result<usize> {
    let keypad = calc_neighbour_list(KEYPAD);
    let kpad = all_pairs_shortest_paths(&keypad);
    let dirpad = calc_neighbour_list(DIRPAD);
    let dpad = all_pairs_shortest_paths(&dirpad);
    let mut cache = FxHashMap::default();

    inp.lines()
        .filter(|line| !line.is_empty())
        .map(|line| complexity_score(line, depth, &kpad, &dpad, &mut cache))
        .try_fold(0, |a, b| Ok(a + b?))
}

pub fn part_1(inp: &str) -> anyhow::Result<String> {
    Ok(format!("{}", calc(inp, 2)?))
}

pub fn part_2(inp: &str) -> anyhow::Result<String> {
    Ok(format!("{}", calc(inp, 25)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_p1() {
        let keypad = calc_neighbour_list(KEYPAD);
        let kpad = all_pairs_shortest_paths(&keypad);
        let dirpad = calc_neighbour_list(DIRPAD);
        let dpad = all_pairs_shortest_paths(&dirpad);
        let a29 = min_keypresses(
            2,
            "029A".to_string(),
            &kpad,
            &dpad,
            &mut FxHashMap::default(),
            true,
        );
        assert_eq!(a29, 68);
        let a980 = min_keypresses(
            2,
            "980A".to_string(),
            &kpad,
            &dpad,
            &mut FxHashMap::default(),
            true,
        );
        assert_eq!(a980, 60);
    }

    #[test]
    fn verify_shortest_path() {
        let keypad = calc_neighbour_list(KEYPAD);
        let five_to_a = shortest_paths(&keypad, '5', 'A');
        assert_eq!(five_to_a.len(), 3);
        assert!(five_to_a.contains(&"vv>".to_string()));
        assert!(five_to_a.contains(&">vv".to_string()));
        assert!(five_to_a.contains(&"v>v".to_string()));
    }

    #[test]
    fn check_keypad() {
        let keypad = calc_neighbour_list(KEYPAD);
        let five_n = keypad.get(&'5').unwrap();
        assert!(five_n.contains(&('8', '^')));
        assert!(five_n.contains(&('2', 'v')));
        assert!(five_n.contains(&('4', '<')));
        assert!(five_n.contains(&('6', '>')));
        let zero_n = keypad.get(&'0').unwrap();
        assert_eq!(zero_n.len(), 2);
        assert!(zero_n.contains(&('A', '>')));
        assert!(zero_n.contains(&('2', '^')));
    }

    #[test]
    fn check_dirpad() {
        let dirpad = calc_neighbour_list(DIRPAD);
        let hat_n = dirpad.get(&'^').unwrap();
        assert_eq!(hat_n.len(), 2);
        assert!(hat_n.contains(&('v', 'v')));
        assert!(hat_n.iter().contains(&('A', '>')));
    }
}
