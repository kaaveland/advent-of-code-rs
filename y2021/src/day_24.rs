use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use regex::Regex;

// General notes:
// The assignment has 14 distinct subprograms, one for each digit. `z`
// carries over between each of them and is initially 0.
// Each program starts out by setting x to the value of z % 26, then
// it will divide z by 1 or 26.
// Then it will add a constant to the value of x (still z % 26); let's
// call that A, so that x = (z0 % 26) + A.
// Having done that, it compares `x` to the input `w`, which is the first
// time we actually use the digits, then compares that to 0 which is basically
// a bool inversion
// So at this point: we have calculated:
// z0 = 0 or carried over
// z1 = z / 1 or z / 26
// x0 = z0 % 26
// x1 = x0 + A
// x2 = (x1 == w) == 0 -> this is 1 when (z0 % 26) + A != input
// We start using y:
// y0 = 25 * x2 + 1 this is 0 or 26
// z2 = z1 * y
// y1 = (w + B) * x2
// z3 = z2 + y1
// So each program section is actually identified by only 3 numbers:
// the divisor of Z, A which is added to x and B which is added to y
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Program {
    div_z: i64,
    add_x: i64,
    add_y: i64,
}

fn parse(input: &str) -> Result<Vec<Program>> {
    let div_z_re = Regex::new(r"div z ([0-9]+)")?;
    let add_x_re = Regex::new(r"add x (-?[0-9]+)")?;
    let add_y_re = Regex::new(r"add y (-?[0-9]+)")?;

    input
        .split("inp w")
        .filter(|block| !block.is_empty())
        .map(|block| {
            let err = || anyhow!("Unable to match {block}");
            let div_z = div_z_re.captures(block).with_context(err)?;
            let div_z = div_z.get(1).with_context(err)?.as_str().parse()?;
            let add_x = add_x_re.captures(block).with_context(err)?;
            let add_x = add_x.get(1).with_context(err)?.as_str().parse()?;
            let add_y = add_y_re.captures_iter(block).last().with_context(err)?;
            let add_y = add_y.get(1).with_context(err)?.as_str().parse()?;
            Ok(Program {
                div_z,
                add_x,
                add_y,
            })
        })
        .collect()
}

fn solve(progs: &[Program], digits: &[i64]) -> Option<i64> {
    let mut work = vec![(0, 0, 0)];

    while let Some((index, z_in, current)) = work.pop() {
        if index == progs.len() && z_in == 0 {
            return Some(current);
        } else if index < progs.len() {
            let prog = progs[index];
            for w in digits.iter().copied() {
                if prog.div_z == 26 && z_in % 26 + prog.add_x != w {
                    continue;
                }
                let z_out = if z_in % 26 + prog.add_x == w {
                    z_in / prog.div_z
                } else {
                    z_in / prog.div_z * 26 + w + prog.add_y
                };

                work.push((index + 1, z_out, current * 10 + w));
            }
        }
    }

    None
}

pub fn part_1(input: &str) -> Result<String> {
    let progs = parse(input)?;
    let sol = solve(&progs, &(1..=9).collect_vec()).with_context(|| anyhow!("Unable to solve"))?;
    Ok(format!("{sol:?}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let progs = parse(input)?;
    let sol =
        solve(&progs, &(1..=9).rev().collect_vec()).with_context(|| anyhow!("Unable to solve"))?;
    Ok(format!("{sol:?}"))
}
