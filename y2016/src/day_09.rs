use regex::Regex;
use std::sync::LazyLock;

static RLE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\((\d+)x(\d+)\)").unwrap());

fn decompressed_segments(s: &str) -> Option<(&str, &str, usize)> {
    if let Some(m) = RLE_RE.captures(s) {
        let match_chars = RLE_RE.find(s).unwrap().as_str();
        let next = &s[match_chars.len()..];
        let nchars = m.get(1)?.as_str().parse().ok()?;
        let nrepeats = m.get(2)?.as_str().parse().ok()?;
        Some((&next[nchars..], &next[..nchars], nrepeats))
    } else {
        None
    }
}

pub fn recursive_rle_len(mut s: &str) -> usize {
    let mut len = 0;
    while !s.is_empty() {
        if let Some((next, rec, repeats)) = decompressed_segments(s) {
            let nch = recursive_rle_len(rec);
            len += repeats * nch;
            s = next;
        } else {
            len += 1;
            s = &s[1..];
        }
    }
    len
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let s: String = s.chars().filter(|ch| !ch.is_whitespace()).collect();
    let mut as_str = s.as_str();
    let mut len = 0;
    while !as_str.is_empty() {
        if let Some((next, rec, repeats)) = decompressed_segments(as_str) {
            as_str = next;
            len += rec.len() * repeats;
        } else {
            as_str = &as_str[1..];
        }
    }
    Ok(len.to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let s: String = s.chars().filter(|ch| !ch.is_whitespace()).collect();
    let len = recursive_rle_len(s.as_str());
    Ok(len.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_examples() {
        assert_eq!(part_1("ADVENT").unwrap().as_str(), "6");
        assert_eq!(part_1("A(1x5)BC").unwrap().as_str(), "7");
        assert_eq!(part_1("X(8x2)(3x3)ABCY").unwrap().as_str(), "18");
    }

    #[test]
    fn p2_examples() {
        assert_eq!(part_2("(3x3)XYZ").unwrap().as_str(), "9");

        assert_eq!(
            part_2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
                .unwrap()
                .as_str(),
            "445"
        );
    }
}
