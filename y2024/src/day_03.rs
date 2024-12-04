use regex::Regex;

fn sum_muls(input: &str) -> anyhow::Result<(i32, i32)> {
    let mul_re = Regex::new(r#"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))"#)?;
    let r: anyhow::Result<_> =
        mul_re
            .captures_iter(input)
            .try_fold((0, 0, true), |(p1, p2, enabled), m| {
                if let (Some(l), Some(r)) = (m.get(2), m.get(3)) {
                    let mul = l.as_str().parse::<i32>()? * r.as_str().parse::<i32>()?;
                    Ok((p1 + mul, if enabled { p2 + mul } else { p2 }, enabled))
                } else {
                    Ok((p1, p2, m.get(4).is_some()))
                }
            });
    let (p1, p2, _) = r?;
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

    const EXAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_example() {
        assert_eq!(sum_muls(EXAMPLE).unwrap(), (161, 48));
    }
}
