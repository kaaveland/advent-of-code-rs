fn parse(s: &str) -> anyhow::Result<Vec<i32>> {
    s.split_whitespace()
        .filter(|n| !n.is_empty())
        .map(|n| Ok(n.parse()?))
        .collect()
}

fn count_possible_triangles(numbers: &[i32]) -> usize {
    assert_eq!(numbers.len() % 3, 0);
    let mut count = 0;
    for i in 0..(numbers.len() / 3) {
        let j = i * 3;
        let a = numbers[j];
        let b = numbers[j + 1];
        let c = numbers[j + 2];
        if b + c > a && c + a > b && a + b > c {
            count += 1;
        }
    }
    count
}

fn reshape(n: &[i32]) -> Vec<i32> {
    assert_eq!(n.len() % 3, 0);
    let mut out = Vec::with_capacity(n.len());
    for col in 0..3usize {
        for ix in (0..n.len()).filter(|ix| *ix % 3 == col) {
            out.push(n[ix]);
        }
    }
    assert_eq!(n.len(), out.len());
    out
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    Ok(count_possible_triangles(&parse(s)?).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    Ok(count_possible_triangles(&reshape(&parse(s)?)).to_string())
}
