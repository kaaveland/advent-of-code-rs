use anyhow::{anyhow, Context, Result};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum MaskBit {
    Passthrough,
    Off,
    On,
}

type Mask = Vec<MaskBit>;

fn parse_mask(line: &str) -> Result<Instruction> {
    let (to, mask) = line
        .split_once(" = ")
        .with_context(|| anyhow!("No = in mask line"))?;
    if to != "mask" {
        return Err(anyhow!("Not a mask: {line}"));
    }

    let bits: Result<Vec<_>> = mask
        .chars()
        .rev()
        .map(|ch| match ch {
            'X' => Ok(MaskBit::Passthrough),
            '0' => Ok(MaskBit::Off),
            '1' => Ok(MaskBit::On),
            _ => Err(anyhow!("Illegal mask bit: {ch}")),
        })
        .collect();
    let bits = bits?;
    Ok(Instruction::Mask { mask: bits })
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Instruction {
    Write { mem: usize, set: u64 },
    Mask { mask: Mask },
}

fn get_re() -> &'static Regex {
    static MEM_RE: OnceCell<Regex> = OnceCell::new();
    MEM_RE.get_or_init(|| Regex::new(r"mem\[([0-9]+)]").unwrap())
}

fn parse_write(line: &str) -> Result<Instruction> {
    let regex = get_re();

    let (addr_part, value_part) = line
        .split_once(" = ")
        .with_context(|| anyhow!("Missing = in instruction"))?;
    let caps = regex
        .captures(addr_part)
        .with_context(|| anyhow!("Illegal addr_part: {addr_part}"))?;
    let addr: usize = caps.get(1).unwrap().as_str().parse()?;
    let set = value_part.parse()?;
    Ok(Instruction::Write { mem: addr, set })
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mask = parse_mask(line).ok();
            mask.or_else(|| parse_write(line).ok())
        })
        .collect()
}

fn solve_1(input: &str) -> Result<u64> {
    use Instruction::*;
    let instructions = parse(input);
    let mut active_mask = vec![MaskBit::Passthrough; 36];
    let max_instr = instructions
        .iter()
        .filter_map(|ins| match ins {
            Write { mem, .. } => Some(*mem),
            Mask { .. } => None,
        })
        .max()
        .with_context(|| anyhow!("Missing instructions"))?;
    let mut memory = vec![0; max_instr + 1];
    for ins in instructions {
        match ins {
            Mask { mask } => {
                active_mask = mask;
            }
            Write { mem, mut set } => {
                for (bit, val) in active_mask.iter().copied().enumerate() {
                    match val {
                        MaskBit::Passthrough => {}
                        MaskBit::Off => {
                            set &= !(1 << bit);
                        }
                        MaskBit::On => {
                            set |= 1 << bit;
                        }
                    }
                }
                memory[mem] = set;
            }
        }
    }
    Ok(memory.into_iter().sum())
}

pub fn part_1(input: &str) -> Result<String> {
    solve_1(input).map(|n| format!("{n}"))
}

fn memory_decoder(memory: &mut HashMap<u64, u64>, mask: &Mask, mut addr: u64, val: u64) {
    // First, set all of the definite on-bits from the mask
    let on_bits = mask
        .iter()
        .enumerate()
        .filter(|(_, bit)| matches!(bit, MaskBit::On))
        .map(|(bitno, _)| bitno);

    for bit in on_bits {
        addr |= 1 << bit;
    }
    // Next, find the bit positions we are floating
    let floating_bits = mask
        .iter()
        .enumerate()
        .filter(|(_, bit)| matches!(bit, MaskBit::Passthrough))
        .map(|(bit_pos, _)| bit_pos)
        .collect_vec();
    // We will need to touch this many memory addresses
    let addresses_touched: u64 = 1 << floating_bits.len();
    for bits_val in 0..addresses_touched {
        let mut this_addr = addr;
        // Now take each bit position from bits_val and set it using the
        // map of floating bits on this_addr
        for (bits_val_pos, addr_pos) in (0..floating_bits.len()).zip(floating_bits.iter()) {
            let set_it = (1 << bits_val_pos) & bits_val > 0;
            // Clear that bit
            this_addr &= !(1 << *addr_pos);
            if set_it {
                this_addr |= 1 << *addr_pos;
            }
        }
        memory.insert(this_addr, val);
    }
}

fn solve_2(input: &str) -> Result<u64> {
    use Instruction::*;
    let instructions = parse(input);
    let mut active_mask = vec![MaskBit::Off; 36];
    let mut memory = HashMap::default();

    for ins in instructions {
        match ins {
            Mask { mask } => {
                active_mask = mask;
            }
            Write { mem, set } => {
                memory_decoder(&mut memory, &active_mask, mem as u64, set);
            }
        }
    }
    Ok(memory.values().sum())
}

pub fn part_2(input: &str) -> Result<String> {
    solve_2(input).map(|n| format!("{n}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let ans = solve_2(
            "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        )
        .unwrap();
        assert_eq!(ans, 208);
    }

    #[test]
    fn test_memory_decoder() {
        let mut memory = HashMap::default();
        let mask = vec![
            MaskBit::Passthrough,
            MaskBit::Passthrough,
            MaskBit::On,
            MaskBit::Off,
        ];
        memory_decoder(&mut memory, &mask, 0, 5);
        assert_eq!(memory.len(), 4);
    }

    #[test]
    fn test_parse_mask() {
        assert!(parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").is_ok());
    }

    #[test]
    fn test_parse_write() {
        assert!(parse_write("mem[7] = 101").is_ok());
    }
    #[test]
    fn test_1() {
        let n = solve_1(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
        )
        .unwrap();
        assert_eq!(n, 165);
    }
}
