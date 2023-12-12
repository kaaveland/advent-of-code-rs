use anyhow::Result;
use fxhash::FxHashMap;
use itertools::Itertools;

fn calculate(examples: &Vec<(Vec<char>, Vec<usize>)>) -> usize {
    let mut cache = Cache::default();
    let mut s = 0;
    for (cs, groups) in examples {
        s += count(cs, groups, &mut cache);
    }
    s
}

fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, rest) = line.split_once(' ').unwrap();
            let mut char_str = vec![];
            char_str.extend(first.chars());
            let mut group = vec![];
            for n in rest.split(',') {
                group.push(n.parse().unwrap());
            }
            (char_str, group)
        })
        .collect()
}

pub fn part_1(input: &str) -> Result<String> {
    Ok(calculate(&parse(input)).to_string())
}

pub fn part_2(input: &str) -> Result<String> {
    let inp = parse(input)
        .into_iter()
        .map(|(ch, g)| {
            let s: String = ch.into_iter().collect();
            let vs = (0..5).map(|_| s.clone()).join("?");
            let ch: Vec<_> = vs.chars().collect();
            let mut new_g = Vec::new();
            for _ in 0..5 {
                g.iter().copied().for_each(|g| new_g.push(g));
            }
            (ch, new_g)
        })
        .collect();
    Ok(calculate(&inp).to_string())
}
type Cache<'a> = FxHashMap<(&'a [char], &'a [usize]), usize>;

fn count<'a>(mut cs: &'a [char], groups: &'a [usize], cache: &mut Cache<'a>) -> usize {
    // Skip any dots at the start
    while !cs.is_empty() && cs[0] == '.' {
        cs = &cs[1..];
    }

    if cache.contains_key(&(cs, groups)) {
        return *cache.get(&(cs, groups)).unwrap();
    }

    // The minimum required space to place all groups is at least 1 space between each
    // group of #, then their total length in addition
    if !groups.is_empty() && cs.len() < (groups.len() - 1 + groups.iter().sum::<usize>()) {
        cache.insert((cs, groups), 0);
        return 0;
    }

    // This is a mouthful:
    // if we have no group, that's still okay if charstring is empty or contains no #
    if groups.is_empty() && (cs.is_empty() || cs.iter().all(|ch| *ch != '#')) {
        cache.insert((cs, groups), 1);
        1
    } else if !groups.is_empty() && !cs.is_empty() {
        // We have at least 1 group and at least 1 input left
        let group_length = groups[0];
        if group_length > cs.len() {
            cache.insert((cs, groups), 0);
            return 0;
        }
        // Check if we can assign the group at the prefix of the bytestring first, we know
        // it must start with # or ?, since we skipped dots. Then, we must have either used
        // all our chars, or end find a terminator
        let assignable_here = cs[..group_length].iter().all(|ch| *ch != '.')
            && (group_length == cs.len() || cs[group_length] != '#');

        // It must be terminated by a . to be valid, and it's only valid if we can recurse
        // and assign the rest of the groups
        let mut found = if assignable_here && group_length == cs.len() {
            if groups.len() == 1 {
                1
            } else {
                0
            }
        } else if assignable_here {
            count(&cs[group_length + 1..], &groups[1..], cache)
        } else {
            0
        };

        if cs[0] == '?' {
            // We can also choose to _not_ assign here, treating it as a .
            found += count(&cs[1..], groups, cache)
        }
        cache.insert((cs, groups), found);
        found
    } else {
        cache.insert((cs, groups), 0);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EX).unwrap(), "21".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EX).unwrap(), "525152".to_string());
    }
}
