use anyhow::Result;
use fxhash::FxHashSet as HashSet;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Instr {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

type Program = Vec<Instr>;

fn parse(input: &str) -> Program {
    use Instr::*;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (instr, offset) = line.split_once(' ')?;
            let offset = offset.strip_prefix('+').unwrap_or(offset);
            let offset: i32 = offset.parse().ok()?;
            match instr {
                "nop" => Some(Nop(offset)),
                "jmp" => Some(Jmp(offset)),
                "acc" => Some(Acc(offset)),
                _ => None,
            }
        })
        .collect()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Registers {
    instr: i32,
    acc: i32,
}

pub fn part_1(input: &str) -> Result<String> {
    let prog = parse(input);
    let reg = run_program(&prog);
    Ok(format!("{}", reg.acc))
}

fn run_program(prog: &Program) -> Registers {
    use Instr::*;
    let mut seen = HashSet::default();
    let mut reg = Registers { instr: 0, acc: 0 };
    while !seen.contains(&reg.instr) && reg.instr < prog.len() as i32 {
        seen.insert(reg.instr);
        let instr = &prog[reg.instr as usize];

        match instr {
            Nop(_) => {
                reg.instr += 1;
            }
            Acc(n) => {
                reg.acc += *n;
                reg.instr += 1;
            }
            Jmp(n) => {
                reg.instr += *n;
            }
        }
    }
    reg
}

pub fn part_2(input: &str) -> Result<String> {
    use Instr::*;
    let mut prog = parse(input);
    let mut res = run_program(&prog);
    let max_bad_instr = res.instr as usize;

    for i in (0..max_bad_instr).rev() {
        match prog[i] {
            old @ Nop(n) => {
                let new = Jmp(n);
                prog[i] = new;
                res = run_program(&prog);
                if res.instr < prog.len() as i32 {
                    prog[i] = old
                } else {
                    break;
                }
            }
            old @ Jmp(n) => {
                let new = Nop(n);
                prog[i] = new;
                res = run_program(&prog);
                if res.instr < prog.len() as i32 {
                    prog[i] = old
                } else {
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(format!("{}", res.acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let r = part_1(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
",
        )
        .unwrap();
        assert_eq!(r.as_str(), "5");
    }
}
