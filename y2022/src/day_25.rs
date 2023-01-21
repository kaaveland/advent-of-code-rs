use anyhow::Result;

const SNAFU_STR: [char; 5] = ['0', '1', '2', '=', '-'];

fn snafu_to_dec(snafu: &str) -> i64 {
    let mut accum = 0;
    for ch in snafu.chars() {
        accum *= 5;
        accum += match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown snafu digit: {ch}"),
        }
    }
    accum
}

fn dec_to_snafu(mut dec: i64) -> String {
    let mut out = String::new();

    while dec != 0 {
        let rem = dec.rem_euclid(5) as usize;
        out.push(SNAFU_STR[rem]);
        dec = (2 + dec) / 5;
    }

    out.chars().rev().collect()
}

pub fn part_1(input: &str) -> Result<String> {
    let sum: i64 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(snafu_to_dec)
        .sum();
    let snafu = dec_to_snafu(sum);
    Ok(snafu)
}

pub fn part_2(_input: &str) -> Result<String> {
    Ok("Enter the answers and collect the last star!".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: [(&str, i64); 13] = [
        ("1=-0-2", 1747),
        ("12111", 906),
        ("2=0=", 198),
        ("21", 11),
        ("2=01", 201),
        ("111", 31),
        ("20012", 1257),
        ("112", 32),
        ("1=-1=", 353),
        ("1-12", 107),
        ("12", 7),
        ("1=", 3),
        ("122", 37),
    ];

    #[test]
    fn test_from_snafu() {
        for (snafu, dec) in EXAMPLES {
            assert_eq!(snafu_to_dec(snafu), dec);
        }
    }

    #[test]
    fn test_to_snafu() {
        for (snafu, dec) in EXAMPLES {
            assert_eq!(dec_to_snafu(dec), snafu);
        }
    }
}
