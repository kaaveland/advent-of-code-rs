use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(i32, usize),
    Noop(),
}

fn parse_instruction(instr: &str) -> Instruction {
    let mut parts = instr.split(' ');
    let first = parts.next().expect("Empty instruction");
    match first {
        "noop" => Instruction::Noop(),
        "addx" => Instruction::Add(
            parts
                .next()
                .and_then(|arg| arg.parse().ok())
                .expect("Missing operand"),
            2,
        ),
        _ => panic!("Wrong instruction: {}", first),
    }
}

fn parse_instructions<'a, I: Iterator<Item = &'a str> + 'a>(it: I) -> Program<'a> {
    let out = it.filter(|l: &&str| !l.is_empty()).map(parse_instruction);
    Program {
        source: Box::new(out),
        register: 1,
        op: None,
        started: false,
    }
}

struct Program<'a> {
    source: Box<dyn Iterator<Item = Instruction> + 'a>,
    register: i32,
    op: Option<Instruction>,
    started: bool,
}

impl Iterator for Program<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started && self.op.is_none() {
            None
        } else {
            self.started = true;
            match self.op {
                None | Some(Instruction::Noop()) => {
                    self.op = self.source.next();
                    Some(self.register)
                }
                Some(Instruction::Add(count, cycles_remaining)) => {
                    if cycles_remaining == 1 {
                        self.op = self.source.next();
                        self.register += count;
                    } else {
                        self.op = Some(Instruction::Add(count, cycles_remaining - 1));
                    }
                    Some(self.register)
                }
            }
        }
    }
}

pub fn part_1(input: &str) -> Result<String> {
    let prog = parse_instructions(input.lines());
    let mut sum = 0;
    let cycles_read = [20, 60, 100, 140, 180, 220];

    for (index, register) in prog.enumerate() {
        let cycle: i32 = (index + 1) as i32;
        if cycles_read.contains(&cycle) {
            sum += cycle * register;
        }
    }

    Ok(format!("{sum}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let prog = parse_instructions(input.lines());
    let mut display: Vec<String> = Default::default();
    let mut row: String = Default::default();

    for register in prog {
        let visible = ((row.len() as i32) - register).abs() <= 1;
        row.push(if visible { '#' } else { ' ' });
        if row.len() == 40 {
            display.push(row);
            row = Default::default();
        }
    }
    Ok(display.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_EXAMPLE: &str = "noop
addx 3
addx -5
";
    const LARGE_EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[cfg(test)]
    #[test]
    fn test_small_example() {
        let mut prog = parse_instructions(SMALL_EXAMPLE.lines());
        assert_eq!(prog.next(), Some(1));
        assert_eq!(prog.next(), Some(1));
        assert_eq!(prog.next(), Some(1));
        assert_eq!(prog.next(), Some(4));
        assert_eq!(prog.next(), Some(4));
        assert_eq!(prog.next(), Some(-1));
        assert_eq!(prog.next(), None);
    }

    #[test]
    fn test_large_example() {
        let prog = parse_instructions(LARGE_EXAMPLE.lines());
        let cycles_read = [20, 60, 100, 140, 180, 220];
        let mut sum = 0;

        for (index, register) in prog.enumerate() {
            let cycle: i32 = (index + 1) as i32;
            if cycles_read.contains(&cycle) {
                println!(
                    "cycle: {} register: {} signal: {}",
                    cycle,
                    register,
                    cycle * register
                );
                sum += cycle * register;
            }
        }

        assert_eq!(sum, 13140);
    }

    #[test]
    fn test_parse_program() {
        let _lines: Vec<Instruction> = SMALL_EXAMPLE
            .lines()
            .filter(|l| !l.is_empty())
            .map(parse_instruction)
            .collect();
    }
}
