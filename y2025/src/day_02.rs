struct IdRange {
    start: u64,
    end: u64,
}

fn parse(s: &str) -> Vec<IdRange> {
    s.trim()
        .split(',')
        .filter_map(|range| {
            let (start, end) = range.split_once('-')?;
            assert!(!start.starts_with('0'));
            assert!(!end.starts_with('0'));
            let start = start.parse().ok()?;
            let end = end.parse().ok()?;
            Some(IdRange { start, end })
        })
        .collect()
}

fn repeated_digits_twice(mut n: u64) -> bool {
    let mut digs = [None; 32];
    let mut pos = 0;
    while n > 0 {
        digs[pos] = Some((n % 10) as u8);
        n /= 10;
        pos += 1;
    }
    let mid = pos / 2;
    digs[..mid] == digs[mid..pos]
}

fn repeated_at_least_twice(mut n: u64) -> bool {
    // Assumes no u64 is longer than 32 decimal digits
    let mut digs = [None; 32];
    let mut pos = 0;
    while n > 0 {
        digs[pos] = Some((n % 10) as u8);
        n /= 10;
        pos += 1;
    }
    for len in 1..=(pos / 2) {
        if pos % len == 0 {
            let groups = pos / len;
            let mut repetition = true;
            for g in 1..groups {
                if digs[0..len] != digs[g * len..g * len + len] {
                    // Not a repetition
                    repetition = false;
                    break;
                }
            }
            if repetition {
                return true;
            }
        }
    }
    false
}

fn ids(ranges: &[IdRange]) -> impl Iterator<Item = u64> + use<'_> {
    ranges.iter().flat_map(|range| range.start..=range.end)
}

fn sum_invalid_ids(s: &str) -> u64 {
    let ranges = parse(s);
    ids(&ranges).filter(|id| repeated_digits_twice(*id)).sum()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", sum_invalid_ids(s)))
}

fn sum_invalid_p2_ids(s: &str) -> u64 {
    let ranges = parse(s);
    ids(&ranges).filter(|id| repeated_at_least_twice(*id)).sum()
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", sum_invalid_p2_ids(s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rep() {
        assert!(repeated_digits_twice(6464));
        assert!(repeated_digits_twice(55));
        assert!(repeated_digits_twice(123123));
        assert!(!repeated_digits_twice(123124));
    }

    #[test]
    fn test_rep_at_least_twice() {
        assert!(repeated_at_least_twice(6464));
        assert!(repeated_at_least_twice(55));
        assert!(repeated_at_least_twice(123123));
        assert!(!repeated_at_least_twice(123124));
        assert!(repeated_at_least_twice(999));
        assert!(repeated_at_least_twice(824824824));
        assert!(repeated_at_least_twice(2121212121));
        assert!(repeated_at_least_twice(1188511885));
    }

    #[test]
    fn test_ex() {
        assert_eq!(
            1227775554,
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            )
        );
    }

    #[test]
    fn test_ex2() {
        let ans = sum_invalid_p2_ids(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(4174379265, ans);
    }
}
