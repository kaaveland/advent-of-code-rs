use rayon::prelude::*;

const CHUNK: usize = 32_000;

const HEXDIGITS: &[u8] = b"0123456789abcdef";

fn md5check(k: &str, n: usize) -> Option<(u8, u8)> {
    let mut ctx = md5::Context::new();
    ctx.consume(k);
    ctx.consume(n.to_string());
    let digest = ctx.compute();
    if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0 == 0) {
        let dig = digest[2] & 0x0f;
        let nextdig = (digest[3] & 0xf0) >> 4;
        Some((dig, nextdig))
    } else {
        None
    }
}

fn check_chunk(k: &str, start: usize) -> Vec<(u8, u8)> {
    let mut r: Vec<_> = (start..(start + CHUNK))
        .into_par_iter()
        .filter_map(|n| md5check(k, n).map(|hexdig| (n, hexdig)))
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
