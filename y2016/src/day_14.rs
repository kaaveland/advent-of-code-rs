use itertools::Itertools;
use md5::Digest;
use nom::AsBytes;
use rayon::prelude::*;

#[inline]
fn hexchar(b: u8) -> u8 {
    if b < 10 {
        b + b'0'
    } else {
        (b - 10) + b'a'
    }
}

fn hexdigits(digest: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    let mut written = 0;
    for byte in digest {
        out[written] = hexchar(byte >> 4); // upper half
        out[written + 1] = hexchar(byte & 0x0f); // lower half
        written += 2;
    }
    out
}

fn hexdigest(s: &[u8], stretch: usize) -> [u8; 32] {
    let mut digest = hexdigits(md5::Md5::digest(s).as_bytes());
    for _ in 0..stretch {
        digest = hexdigits(md5::Md5::digest(digest).as_bytes());
    }
    digest
}

fn hexdigests(salt: &str, from: usize, to: usize, stretch: usize) -> Vec<[u8; 32]> {
    (from..to)
        .into_par_iter()
        .map(|index| {
            let seed = format!("{salt}{index}");
            hexdigest(seed.as_bytes(), stretch)
        })
        .collect()
}

fn first_triplet(digest: &[u8; 32]) -> Option<u8> {
    for i in 0..(digest.len() - 2) {
        if digest[i] == digest[i + 1] && digest[i] == digest[i + 2] {
            return Some(digest[i]);
        }
    }
    None
}

fn has_quintuplet(digest: &[u8; 32], ch: u8) -> bool {
    digest.windows(5).any(|w| w == [ch; 5])
}

fn solve(salt: &str, stretch: usize) -> usize {
    const BLOCK_SIZE: usize = 21_000;
    let salt = salt.trim();
    let mut found = vec![];
    for pos in 0.. {
        let digests = hexdigests(
            salt,
            pos * BLOCK_SIZE,
            (pos + 1) * BLOCK_SIZE + 1000,
            stretch,
        );
        let candidates = digests[..digests.len() - 1000]
            .iter()
            .enumerate()
            .collect_vec();
        let mut keys: Vec<_> = candidates
            .into_par_iter()
            .filter_map(|(relative_index, maybe_key)| {
                if let Some(look_for) = first_triplet(maybe_key) {
                    // our own hash is at relative_index. Now we need to look for a
                    // quintuplet of `look_for` anywhere in the next 1000 hashes.
                    if digests[relative_index + 1..relative_index + 1001]
                        .iter()
                        .any(|digest| has_quintuplet(digest, look_for))
                    {
                        Some(pos * BLOCK_SIZE + relative_index)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        keys.sort();
        found.extend(keys);
        if found.len() >= 64 {
            return found[63];
        }
    }
    unreachable!()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let ans = solve(s.trim(), 0);
    Ok(format!("{ans}"))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let ans = solve(s.trim(), 2016);
    Ok(format!("{ans}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_it() {
        assert_eq!(solve("abc", 0), 22728);
    }
}
