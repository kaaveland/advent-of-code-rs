const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let n = s
        .lines()
        .filter(|word| {
            let mut vowels = i32::from(
                word.chars()
                    .map(|ch| VOWELS.contains(&ch))
                    .next()
                    .unwrap_or(false),
            );
            let mut twice = false;
            for (ch, nch) in word.chars().zip(word.chars().skip(1)) {
                twice = twice || ch == nch;
                vowels += i32::from(VOWELS.contains(&nch));
                if [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')].contains(&(ch, nch)) {
                    return false;
                }
            }
            (vowels >= 3) && twice
        })
        .count();
    Ok(n.to_string())
}

fn pair_twice(s: &str) -> bool {
    for i in 0..(s.len() - 2) {
        let pair = &s[i..i + 2];
        if s[i + 2..].contains(pair) {
            return true;
        }
    }
    false
}

fn repeats(s: &str) -> bool {
    for i in 0..(s.len() - 2) {
        if s[i..i + 1] == s[i + 2..i + 3] {
            return true;
        }
    }
    false
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let n = s
        .lines()
        .filter(|word| pair_twice(word) && repeats(word))
        .count();
    Ok(n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        assert!(pair_twice("qaqlyoyouotsmamm"));
        assert!(repeats("qjhvhtzxzqqjkmpb"));
        assert!(pair_twice("xxyxx"));
        assert!(repeats("xxyxx"));
        assert!(!pair_twice("ieodomkazucvgmuy"));
        assert!(repeats("ieodomkazucvgmuy"));
    }
}
