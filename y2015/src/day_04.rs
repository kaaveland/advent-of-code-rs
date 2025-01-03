use rayon::prelude::*;

const CHUNK: usize = 16_000;

fn md5test(k: &str, n: usize, mask: u8) -> bool {
    let mut ctx = md5::Context::new();
    ctx.consume(k);
    ctx.consume(n.to_string());
    let digest = ctx.compute();
    digest[0] == 0 && digest[1] == 0 && (digest[2] & mask) == 0
}

fn md5pariter(k: &str, mask: u8) -> usize {
    for i in 0.. {
        if let Some(found) = (i * CHUNK..(i + 1) * CHUNK)
            .into_par_iter()
            .filter(|i| md5test(k, *i, mask))
            .min()
        {
            return found;
        }
    }
    unreachable!()
}

pub fn part_1(k: &str) -> anyhow::Result<String> {
    Ok(md5pariter(k.trim(), 0xf0).to_string())
}

pub fn part_2(k: &str) -> anyhow::Result<String> {
    Ok(md5pariter(k.trim(), 0xff).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        assert!(md5test("abcdef", 609043, 0xf0));
    }
}
