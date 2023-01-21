use anyhow::Result;
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use std::ops::RangeInclusive;

const REQUIRED: [&str; 7] = ["byr:", "iyr:", "eyr:", "hgt:", "ecl:", "hcl:", "pid:"];
pub fn part_1(input: &str) -> Result<String> {
    let n = input
        .split("\n\n")
        .filter(|rec| REQUIRED.iter().all(|field| rec.contains(field)))
        .count();
    Ok(format!("{n}"))
}

fn validate_dig(record: &HashMap<&str, &str>, key: &str, range: RangeInclusive<i32>) -> bool {
    record
        .get(key)
        .and_then(|val| {
            let val = val.parse::<i32>().ok()?;
            Some(range.contains(&val))
        })
        .unwrap_or(false)
}

fn validate_digits(record: &HashMap<&str, &str>) -> bool {
    validate_dig(record, "byr", 1920..=2002)
        && validate_dig(record, "eyr", 2020..=2030)
        && validate_dig(record, "iyr", 2010..=2020)
}

fn validate_hgt(record: &HashMap<&str, &str>) -> bool {
    record
        .get("hgt")
        .and_then(|hgt| {
            let unit = &hgt[hgt.len() - 2..];
            let amount = &hgt[..hgt.len() - 2].parse::<i32>().ok()?;
            Some(
                (unit == "cm" && (150..=193).contains(amount))
                    || (unit == "in" && (59..=76).contains(amount)),
            )
        })
        .unwrap_or(false)
}

fn validate_hcl(record: &HashMap<&str, &str>) -> bool {
    record
        .get("hcl")
        .map(|hcl| {
            hcl.len() == 7
                && hcl.starts_with('#')
                && hcl
                    .chars()
                    .skip(1)
                    .all(|ch| "0123456789abcdef".contains(ch))
        })
        .unwrap_or(false)
}

const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn validate_ecl(record: &HashMap<&str, &str>) -> bool {
    record
        .get("ecl")
        .map(|ecl| ECL.iter().contains(ecl))
        .unwrap_or(false)
}

fn validate_pid(record: &HashMap<&str, &str>) -> bool {
    record
        .get("pid")
        .map(|pid| pid.len() == 9 && pid.chars().all(|ch| ch.is_ascii_digit()))
        .unwrap_or(false)
}

type Rule = fn(&HashMap<&str, &str>) -> bool;
const RULES: [Rule; 5] = [
    validate_digits,
    validate_ecl,
    validate_pid,
    validate_hgt,
    validate_hcl,
];

fn valid(block: &&str) -> bool {
    let rec: HashMap<&str, &str> = block
        .split_ascii_whitespace()
        .filter_map(|kv| kv.split_once(':'))
        .collect();
    RULES.iter().all(|validate| validate(&rec))
}
pub fn part_2(input: &str) -> Result<String> {
    let n = input.split("\n\n").filter(valid).count();
    Ok(format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let records: Vec<HashMap<_, _>> = EXAMPLE
            .split("\n\n")
            .map(|rec| {
                rec.split_ascii_whitespace()
                    .filter_map(|kv| kv.split_once(':'))
                    .collect()
            })
            .collect();

        assert!(records.iter().all(validate_digits));
        assert!(records.iter().all(validate_ecl));
        assert!(records.iter().all(validate_hcl));
        assert!(records.iter().all(validate_pid));
        assert!(records.iter().all(validate_hgt));
    }
    const EXAMPLE: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
}
