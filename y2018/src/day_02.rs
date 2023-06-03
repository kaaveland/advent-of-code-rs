use anyhow::Result;
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use std::hash::Hash;

fn letter_count(line: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::default();
    for c in line.chars() {
        *counts.entry(c).or_default() += 1;
    }
    counts
}

fn flip<K, V>(map: &HashMap<K, V>) -> HashMap<V, Vec<K>>
where
    K: Hash + Eq + Copy,
    V: Hash + Eq + Copy,
{
    let mut flipped: HashMap<V, Vec<K>> = HashMap::default();
    for (k, v) in map {
        flipped.entry(*v).or_default().push(*k);
    }
    flipped
}

fn checksum(words: &str) -> usize {
    let (has_two, has_three) =
        words
            .lines()
            .filter(|line| !line.is_empty())
            .fold((0, 0), |(two, three), line| {
                let counts = letter_count(line);
                let by_count = flip(&counts);
                (
                    two + usize::from(by_count.contains_key(&2)),
                    three + usize::from(by_count.contains_key(&3)),
                )
            });
    has_two * has_three
}

pub fn part_1(input: &str) -> Result<String> {
    Ok(checksum(input).to_string())
}

fn closest_match(words: &str) -> Option<(usize, &str, &str)> {
    words
        .lines()
        .cartesian_product(words.lines())
        .filter(|(left, right)| left != right && !(left.is_empty() || right.is_empty()))
        .map(|(left, right)| {
            (
                left.chars()
                    .zip(right.chars())
                    .filter(|(l, r)| l != r)
                    .count(),
                left,
                right,
            )
        })
        .min_by_key(|(count, _, _)| *count)
}

pub fn part_2(input: &str) -> Result<String> {
    let (differing_letters, left, right) =
        closest_match(input).ok_or_else(|| anyhow::anyhow!("No close matches found in input"))?;
    if differing_letters != 1 {
        return Err(anyhow::anyhow!(
            "Closest matches differ by {} letters",
            differing_letters
        ));
    }
    let common_letters: String = left
        .chars()
        .zip(right.chars())
        .filter(|(l, r)| l == r)
        .map(|(l, _)| l)
        .collect();
    Ok(common_letters)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fxhash::FxHashSet as HashSet;
    use quickcheck::quickcheck;

    #[test]
    fn test_letter_count() {
        let line = "bababc";
        let counts = letter_count(line);
        assert_eq!(counts.get(&'a'), Some(&2));
        assert_eq!(counts.get(&'b'), Some(&3));
        assert_eq!(counts.get(&'c'), Some(&1));
    }

    #[test]
    fn checksum_example() {
        let input = "abcdef
                     bababc
                     abbcde
                     abcccd
                     aabcdd
                     abcdee
                     ababab";
        assert_eq!(checksum(input), 12);
    }

    #[test]
    fn closest_match_example() {
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        let (differing_letters, left, right) = closest_match(input).unwrap();
        assert_eq!(differing_letters, 1);
        assert!(left == "fghij" || right == "fghij");
        assert!(left == "fguij" || right == "fguij");
    }

    quickcheck! {
        fn all_counted_letters_appear_in_word(word: String) -> bool {
            letter_count(word.as_str()).keys().all(|c| word.contains(*c))
        }
        fn sum_of_counts_equals_length_of_word(word: String) -> bool {
            let counts = letter_count(word.as_str());
            counts.values().sum::<usize>() == word.as_str().chars().count()
        }
        fn all_values_are_keys_in_flipped_hm(map: HashMap<char, u8>) -> bool {
            let flipped = flip(&map);
            map.values().all(|v| flipped.contains_key(v))
        }
        fn item_count_is_invariant(map: HashMap<char, u8>) -> bool {
            let flipped = flip(&map);
            map.len() == flipped.values().map(|v| v.len()).sum::<usize>()
        }
        fn closest_match_exists_iff_more_than_one_word(words: String) -> bool {
            let lines = words.lines().filter(|line| !line.is_empty()).count();
            closest_match(words.as_str()).is_some() == (lines > 1)
        }
        fn inequal_letters_is_at_least_1(words: String) -> bool {
            let unique_words: HashSet<_> = words.lines().filter(|w| !w.is_empty()).collect();
            unique_words.len() < 2 || closest_match(words.as_str()).unwrap().0 > 0
        }
    }
}
