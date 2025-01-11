fn step(n: &[u8], step_into: &mut Vec<u8>) {
    let mut c = 1;
    for i in 0..n.len() {
        let cur = n[i];
        let next = n.get(i + 1).copied();
        if Some(cur) == next {
            c += 1;
        } else {
            step_into.push(c);
            step_into.push(cur);
            c = 1;
        }
    }
}

fn steps(s: &str, n: usize) -> usize {
    let mut v = Vec::new();
    for b in s.trim().as_bytes() {
        v.push(*b - b'0');
    }
    let mut buf = Vec::new();
    for _ in 0..n {
        step(&v, &mut buf);
        std::mem::swap(&mut v, &mut buf);
        buf.clear();
    }
    v.len()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(steps(s, 40).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    Ok(steps(s, 50).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let mut v = Vec::new();
        step(&[1], &mut v);
        assert_eq!(v, vec![1, 1]);
        v.clear();
        step(&[1, 1], &mut v);
        assert_eq!(v, vec![2, 1]);
        v.clear();
        step(&[2, 1], &mut v);
        assert_eq!(v, vec![1, 2, 1, 1]);
        v.clear();
        step(&[1, 2, 1, 1], &mut v);
        assert_eq!(v, vec![1, 1, 1, 2, 2, 1]);
        v.clear();
        step(&[1, 1, 1, 2, 2, 1], &mut v);
        assert_eq!(v, vec![3, 1, 2, 2, 1, 1]);
    }
}
