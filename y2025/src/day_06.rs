use itertools::Itertools;

// No points for thinking ahead and parsing the problem today
struct Input {
    numbers: Vec<Vec<i64>>,
    operators: Vec<char>,
}

fn parse(s: &str) -> Input {
    let mut numbers: Vec<Vec<_>> = Vec::new();
    let mut operators = Vec::new();
    for line in s.lines().filter(|l| !l.is_empty()) {
        let parts = line.split_ascii_whitespace();
        let first = parts.clone().next().unwrap();
        let numeric = first.parse::<i64>();
        if numeric.is_ok() {
            numbers.push(parts.map(|n| n.parse().unwrap()).collect());
        } else {
            for op in parts {
                operators.push(op.chars().next().unwrap())
            }
        }
    }

    let len = operators.len();
    for n in numbers.iter() {
        assert_eq!(n.len(), len);
    }
    Input { numbers, operators }
}

fn calc(input: &Input) -> Vec<i64> {
    let len = input.operators.len();
    let mut results = Vec::with_capacity(len);
    for i in 0..len {
        let op = input.operators[i];
        let mut total = match op {
            '+' => 0,
            '*' => 1,
            ch => panic!("Unsupported operand {ch}"),
        };
        for j in 0..input.numbers.len() {
            match op {
                '+' => total += input.numbers[j][i],
                '*' => total *= input.numbers[j][i],
                _ => unreachable!(),
            }
        }
        results.push(total);
    }
    results
}

fn grand_total(s: &str) -> i64 {
    let input = parse(s);
    calc(&input).into_iter().sum()
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(format!("{}", grand_total(s)))
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let m: Vec<_> = s
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.bytes().collect_vec())
        .collect();
    let len = m[0].len();
    let lines = m.len();
    for (i, v) in m.iter().enumerate() {
        assert_eq!(v.len(), len, "{i}");
    }

    let mut cursor = 0;
    let mut total = 0;

    while cursor < len {
        let op = m[lines - 1][cursor];
        assert!(op == b'*' || op == b'+');
        let mut n = match op {
            b'*' => 1,
            b'+' => 0,
            _ => unreachable!(),
        };
        loop {
            let mut digit = 0;
            let mut found = false;
            for line in m[..lines - 1].iter() {
                let ch = line[cursor];
                if ch == b' ' {
                    continue;
                } else {
                    found = true;
                    digit = digit * 10 + (ch - b'0') as i64;
                }
            }

            if !found {
                break;
            }

            match op {
                b'*' => n *= digit,
                b'+' => n += digit,
                _ => unreachable!(),
            }
            cursor += 1;
            if cursor >= len {
                break;
            }
        }
        cursor += 1;
        total += n;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_ex_p1() {
        assert_eq!(grand_total(EX), 4277556);
    }
    #[test]
    fn test_ex_p2() {
        assert_eq!(part_2(EX).unwrap().as_str(), "3263827");
    }
}
