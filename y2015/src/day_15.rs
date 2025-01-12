use anyhow::Context;
use regex::Regex;

type Ingredient = [i64; 5];

fn parse(s: &str) -> anyhow::Result<Vec<Ingredient>> {
    let digit = Regex::new(r"-?\d+").context("Invalid regex")?;
    s.lines()
        .map(|n| {
            let mut out = Ingredient::default();
            for (index, num) in digit.find_iter(n).enumerate() {
                out[index] = num.as_str().parse()?;
            }
            Ok(out)
        })
        .collect()
}

fn iterate<F>(target_sum: usize, digit_count: usize, eval: &F) -> i64
where
    F: Fn(&[i64]) -> i64,
{
    fn inner<F>(target_sum: usize, digit_count: usize, buf: &mut Vec<i64>, best: &mut i64, eval: &F)
    where
        F: Fn(&[i64]) -> i64,
    {
        if buf.len() == digit_count - 1 {
            buf.push(target_sum as i64);
            *best = *best.max(&mut eval(buf));
            buf.pop();
        } else {
            for value in 0..=target_sum {
                let remainder = target_sum - value;
                buf.push(value as i64);
                inner(remainder, digit_count, buf, best, eval);
                buf.pop();
            }
        }
    }
    let mut max = i64::MIN;
    inner(
        target_sum,
        digit_count,
        &mut Vec::with_capacity(10),
        &mut max,
        eval,
    );
    max
}

fn evaluator<'a>(ingredients: &'a [&[i64]]) -> impl Fn(&[i64]) -> i64 + 'a {
    |counts: &[i64]| {
        (0..4)
            .map(|prop_ix| {
                ingredients
                    .iter()
                    .zip(counts.iter())
                    .map(|(ingredient, count)| count * ingredient[prop_ix])
                    .sum::<i64>()
                    .max(0)
            })
            .product()
    }
}

fn best_possible(ingredients: &[&[i64]], target_sum: usize) -> i64 {
    let evaluate = evaluator(ingredients);
    iterate(target_sum, ingredients.len(), &evaluate)
}

pub fn part_1(s: &str) -> anyhow::Result<String> {
    let ingredients = parse(s)?;
    let sliced: Vec<_> = ingredients.iter().map(|v| v.as_slice()).collect();
    Ok(best_possible(&sliced, 100).to_string())
}

pub fn part_2(s: &str) -> anyhow::Result<String> {
    let ingredients = parse(s)?;
    let sliced: Vec<_> = ingredients.iter().map(|v| v.as_slice()).collect();
    let slice_ref = &sliced;
    let original_eval = evaluator(slice_ref);
    let eval = move |counts: &[i64]| {
        let calory_sum: i64 = slice_ref
            .iter()
            .zip(counts.iter())
            .map(|(ingredient, count)| count * ingredient[4])
            .sum();
        if calory_sum != 500 {
            i64::MIN
        } else {
            original_eval(counts)
        }
    };
    let best = iterate(100, ingredients.len(), &eval);
    Ok(best.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        let ing = [[-1, -2, 6, 3].as_slice(), [2, 3, -2, -1].as_slice()];
        let eval = evaluator(&ing);
        assert_eq!(eval(&[44, 56]), 62842880);
        assert_eq!(best_possible(&ing, 100), 62842880);
    }
}
