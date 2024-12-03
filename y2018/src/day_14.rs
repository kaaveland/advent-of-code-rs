use anyhow::Result;

#[derive(Debug, Clone, Eq, PartialEq)]
struct RecipeIterator {
    recipes: Vec<u8>,
    elf_1: usize,
    elf_2: usize,
    ix: usize,
}

impl RecipeIterator {
    fn new() -> Self {
        Self {
            recipes: vec![3, 7],
            elf_1: 0,
            elf_2: 1,
            ix: 0,
        }
    }
}

impl Iterator for RecipeIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.ix >= self.recipes.len() {
            let score_1 = self.recipes[self.elf_1];
            let score_2 = self.recipes[self.elf_2];
            let sum = score_1 + score_2;
            if sum >= 10 {
                self.recipes.push(sum / 10);
            }
            self.recipes.push(sum % 10);
            self.elf_1 = (self.elf_1 + (score_1 as usize) + 1) % self.recipes.len();
            self.elf_2 = (self.elf_2 + (score_2 as usize) + 1) % self.recipes.len();
        }
        let res = self.recipes[self.ix];
        self.ix += 1;
        Some(res)
    }
}

pub fn part_1(s: &str) -> Result<String> {
    let n = s.trim().parse::<usize>()?;
    let it = RecipeIterator::new();
    let recipes = it.skip(n).take(10);
    Ok(recipes.map(|n| n.to_string()).collect::<Vec<_>>().join(""))
}

pub fn part_2(s: &str) -> Result<String> {
    let target = s
        .trim()
        .as_bytes()
        .iter()
        .map(|b| *b - b'0')
        .collect::<Vec<_>>();
    let mut it = RecipeIterator::new();
    let mut ix = 0;
    for _ in 0..target.len() {
        let _ = it.next();
    }
    loop {
        if it.recipes[ix..ix + target.len()] == target {
            break;
        }
        ix += 1;
        let _ = it.next();
    }
    Ok(ix.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("9").unwrap(), "5158916779".to_string());
        assert_eq!(part_1("5").unwrap(), "0124515891".to_string());
        assert_eq!(part_1("18").unwrap(), "9251071085".to_string());
        assert_eq!(part_1("2018").unwrap(), "5941429882".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("51589").unwrap(), "9".to_string());
        assert_eq!(part_2("01245").unwrap(), "5".to_string());
        assert_eq!(part_2("92510").unwrap(), "18".to_string());
        assert_eq!(part_2("59414").unwrap(), "2018".to_string());
    }
}
