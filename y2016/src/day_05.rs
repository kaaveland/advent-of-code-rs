use md5::Digest;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

const CHUNK: usize = 128_000;

const HEXDIGITS: &[u8] = b"0123456789abcdef";

fn md5check(k: &md5::Md5, n: usize) -> Option<(u8, u8)> {
    let mut ctx = k.clone();
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    let mut n = n;
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    ctx.update(&buf[i..]);
    let digest = ctx.finalize();
    if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0 == 0) {
        let dig = digest[2] & 0x0f;
        let nextdig = (digest[3] & 0xf0) >> 4;
        Some((dig, nextdig))
    } else {
        None
    }
}

type ChunkCache = Mutex<HashMap<(String, usize), Vec<(u8, u8)>>>;

/// Part 2 scans all the chunks part 1 does, so remember them across the parts
static CHUNK_CACHE: OnceLock<ChunkCache> = OnceLock::new();

fn check_chunk(k: &str, start: usize) -> Vec<(u8, u8)> {
    let cache = CHUNK_CACHE.get_or_init(Default::default);
    let key = (k.to_string(), start);
    if let Some(found) = cache.lock().unwrap().get(&key) {
        return found.clone();
    }
    let found = compute_chunk(k, start);
    cache.lock().unwrap().insert(key, found.clone());
    found
}

fn compute_chunk(k: &str, start: usize) -> Vec<(u8, u8)> {
    let k = md5::Md5::new_with_prefix(k);
    let mut r: Vec<_> = (start..(start + CHUNK))
        .into_par_iter()
        .filter_map(|n| md5check(&k, n).map(|hexdig| (n, hexdig)))
        .collect();
    r.sort();
    r.into_iter().map(|(_, hexdig)| hexdig).collect()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let mut ix = 0;
    let mut hex = Vec::with_capacity(10);
    while hex.len() < 8 {
        hex.extend(
            check_chunk(s.trim(), ix)
                .into_iter()
                .map(|(fst, _)| HEXDIGITS[fst as usize]),
        );
        ix += CHUNK;
    }
    Ok(String::from_utf8_lossy(&hex[..8]).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let mut pw = [0u8; 8];
    let mut written = 0u32;
    let mut ix = 0;
    while written.count_ones() < 8 {
        for (pos, ch) in check_chunk(s.trim(), ix)
            .into_iter()
            .filter(|(pos, _)| (0..8).contains(pos))
        {
            if written & (1 << pos) == 0 {
                pw[pos as usize] = HEXDIGITS[ch as usize];
                written |= 1 << pos;
            }
        }
        ix += CHUNK;
    }
    Ok(String::from_utf8_lossy(&pw).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(part_1("abc").unwrap().as_str(), "18f47a30");
        assert_eq!(part_2("abc").unwrap().as_str(), "05ace8e3");
    }
}
