use anyhow::anyhow;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

fn parse(s: &str) -> IResult<&str, [u32; 3]> {
    let (s, h) = map_res(terminated(digit1, char('x')), |s: &str| s.parse())(s)?;
    let (s, w) = map_res(terminated(digit1, char('x')), |s: &str| s.parse())(s)?;
    let (s, l) = map_res(digit1, |s: &str| s.parse())(s)?;
    let mut dims = [h, w, l];
    dims.sort();
    Ok((s, dims))
}

fn presents(s: &str) -> anyhow::Result<Vec<[u32; 3]>> {
    separated_list1(char('\n'), parse)(s)
        .map_err(|err| anyhow!("{err}"))
        .map(|(_, p)| p)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(presents(s)?
        .into_iter()
        .map(|p| {
            let [h, w, l] = p;
            h * w + 2 * l * w + 2 * w * h + 2 * l * h
        })
        .sum::<u32>()
        .to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    Ok(presents(s)?
        .into_iter()
        .map(|p| {
            let [h, w, l] = p;
            h * w * l + 2 * h + 2 * w
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(part_1("2x3x4\n1x1x10\n").unwrap().as_str(), "101");
    }
    #[test]
    fn test_p2() {
        assert_eq!(part_2("2x3x4\n1x1x10\n").unwrap().as_str(), "48");
    }
}
