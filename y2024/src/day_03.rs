use regex::Regex;

fn sum_muls(input: &str) -> anyhow::Result<(i32, i32)> {
    let mut mul = true;
    let mut p1 = 0;
    let mut p2 = 0;
    let mul_re = Regex::new(r#"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))"#)?;
    for m in mul_re.captures_iter(input) {
        if let Some((l, r)) = m.get(2).and_then(|l| m.get(3).map(|r| (l, r))) {
            let g = l.as_str().parse::<i32>()? * r.as_str().parse::<i32>()?;
            p1 += g;
            if mul {
                p2 += g;
            }
        } else if m.get(4).is_some() {
            mul = true;
        } else if m.get(5).is_some() {
            mul = false;
        }
    }
    Ok((p1, p2))
}

pub fn part_1(input: &str) -> anyhow::Result<String> {
    let (p1, _) = sum_muls(input)?;
    Ok(format!("{p1}"))
}

pub fn part_2(input: &str) -> anyhow::Result<String> {
    let (_, p2) = sum_muls(input)?;
    Ok(format!("{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn test_example() {
        assert_eq!(sum_muls(EXAMPLE).unwrap(), (161, 48));
    }
}
