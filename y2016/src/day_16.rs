use std::ops::Rem;

fn modified_dragon_curve_step(a: &[u8]) -> Vec<u8> {
    let b_it = a
        .iter()
        .rev()
        .map(|&ch| if ch == b'0' { b'1' } else { b'0' });
    let mut o = Vec::from(a);
    o.push(b'0');
    o.extend(b_it);
    o
}

fn checksum_step(a: &[u8]) -> Vec<u8> {
    let mut chk = Vec::with_capacity(a.len() / 2);
    for i in 0..(a.len() / 2) {
        let l = a[i * 2];
        let r = a[i * 2 + 1];
        chk.push(if l == r { b'1' } else { b'0' })
    }
    chk
}

fn checksum(a: &[u8]) -> Vec<u8> {
    let mut chk = checksum_step(a);
    while chk.len().rem(2) == 0 {
        chk = checksum_step(&chk);
    }
    chk
}

fn modified_dragon_curve(a: &[u8], req_len: usize) -> Vec<u8> {
    let mut state = Vec::from(a);
    while state.len() < req_len {
        state = modified_dragon_curve_step(&state);
    }
    state
}

fn modified_dragon_curve_checksum(a: &[u8], req_len: usize) -> Vec<u8> {
    let dragon_curve = modified_dragon_curve(a, req_len);
    checksum(&dragon_curve[..req_len])
}

pub fn part_1(a: &str) -> anyhow::Result<String> {
    let a = a.trim().as_bytes();
    let h = modified_dragon_curve_checksum(a, 272);
    Ok(String::from_utf8(h)?)
}

pub fn part_2(a: &str) -> anyhow::Result<String> {
    let a = a.trim().as_bytes();
    let h = modified_dragon_curve_checksum(a, 35651584);
    Ok(String::from_utf8(h)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modified_dragon_curve_step() {
        assert_eq!(b"100".to_vec(), modified_dragon_curve_step(b"1"));
        assert_eq!(b"001".to_vec(), modified_dragon_curve_step(b"0"));
        assert_eq!(
            b"11111000000".to_vec(),
            modified_dragon_curve_step(b"11111")
        );
        assert_eq!(
            b"1111000010100101011110000".to_vec(),
            modified_dragon_curve_step(b"111100001010")
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(b"100".to_vec(), checksum(b"110010110100"));
    }

    #[test]
    fn test_whole_p1() {
        assert_eq!(
            modified_dragon_curve_checksum(b"10000", 20),
            b"01100".to_vec()
        );
    }
}
