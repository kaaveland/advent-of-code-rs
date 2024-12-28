use crate::elflang;
use crate::elflang::AsmInstruction::Equal;
use crate::elflang::Operand::Reg;
use crate::elflang::{exec_with_ipreg, Command, DisElf, Registers};
use anyhow::{Context, Result};
use fxhash::FxHashSet;

fn find_ins_reg_to_watch(program: &[Command], regs: [&str; 6]) -> Option<(usize, usize)> {
    program
        .iter()
        .enumerate()
        .filter_map(|(instruction, cmd)| {
            let dis = DisElf::dis(cmd, regs)?;
            match dis.asm_instruction {
                Equal(Reg("a"), Reg(o)) | Equal(Reg(o), Reg("a")) => Some(
                    regs.into_iter()
                        .enumerate()
                        .find(|(_, n)| *n == o)
                        .map(|(i, _)| (instruction, i))?,
                ),
                _ => None,
            }
        })
        .next()
}

fn tick_once(
    registers: &mut Registers<6>,
    watch: (usize, usize),
    ip: &mut usize,
    ip_reg: usize,
    program: &[Command],
) -> Option<usize> {
    let (ins, reg) = watch;
    while *ip < program.len() {
        let peek = *ip == ins;
        let cmd = program[*ip];
        *registers = exec_with_ipreg(&cmd, ip, ip_reg, *registers)?;
        *ip += 1;
        if peek {
            return Some(registers[reg]);
        }
    }
    None
}

fn setup(s: &str) -> Result<(usize, Vec<Command>, (usize, usize))> {
    let (ip_reg, program) = elflang::parse_elflang_asm(s)?;
    let mut reg: [&str; 6] = ["a", "b", "c", "d", "e", "f"]; // Useful for reading the disassembly
    reg[ip_reg] = "ip";
    let watch =
        find_ins_reg_to_watch(&program, reg).context("Unable to discover register to watch")?;
    Ok((ip_reg, program, watch))
}

pub fn part_1(s: &str) -> Result<String> {
    let (ip_reg, program, watch) = setup(s)?;
    let mut ip: usize = 0;
    let mut registers: Registers<6> = [0usize; 6];

    let n = tick_once(&mut registers, watch, &mut ip, ip_reg, &program)
        .context("Unable to evaluate")?;

    Ok(n.to_string())
}

// Disassembly
// Useless section:
// 00 DisElf { result_reg: "e", asm_instruction: Set(Lit(123)) } | e = 123
// 01 DisElf { result_reg: "e", asm_instruction: And(Reg("e"), Lit(456)) } | e = e & 456
// 02 DisElf { result_reg: "e", asm_instruction: Equal(Reg("e"), Lit(72)) } | e = e == 72
// 03 DisElf { result_reg: "ip", asm_instruction: Add(Reg("e"), Reg("ip")) } | ip = ip + e
// 04 DisElf { result_reg: "ip", asm_instruction: Set(Lit(0)) } | ip = 0 -- goto 0
// End useless section
// 05 DisElf { result_reg: "e", asm_instruction: Set(Lit(0)) } | e = 0
// 06 DisElf { result_reg: "f", asm_instruction: Or(Reg("e"), Lit(65536)) } | f = e | 0x10000
// 07 DisElf { result_reg: "e", asm_instruction: Set(Lit(10704114)) } | e = 10704114
// 08 DisElf { result_reg: "c", asm_instruction: And(Reg("f"), Lit(255)) } | c = f & 0xff
// 09 DisElf { result_reg: "e", asm_instruction: Add(Reg("e"), Reg("c")) } | e = e + c
// 10 DisElf { result_reg: "e", asm_instruction: And(Reg("e"), Lit(16777215)) } | e = e & 0xffffff
// 11 DisElf { result_reg: "e", asm_instruction: Mul(Reg("e"), Lit(65899)) } | e = e * 65899
// 12 DisElf { result_reg: "e", asm_instruction: And(Reg("e"), Lit(16777215)) } | e = e & 0xffffff
// 13 DisElf { result_reg: "c", asm_instruction: Greater(Lit(256), Reg("f")) } | c = 256 > f
// 14 DisElf { result_reg: "ip", asm_instruction: Add(Reg("c"), Reg("ip")) } | ip = c + ip -- skip next goto 16
// 15 DisElf { result_reg: "ip", asm_instruction: Add(Reg("ip"), Lit(1)) } | ip = ip + 1 -- skip next
// 16 DisElf { result_reg: "ip", asm_instruction: Set(Lit(27)) } | ip = 27 -- goto 28
// 17 DisElf { result_reg: "c", asm_instruction: Set(Lit(0)) } | c = 0
// 18 DisElf { result_reg: "d", asm_instruction: Add(Reg("c"), Lit(1)) } | d = c + 1
// 19 DisElf { result_reg: "d", asm_instruction: Mul(Reg("d"), Lit(256)) } | d = d * 256
// 20 DisElf { result_reg: "d", asm_instruction: Greater(Reg("d"), Reg("f")) } | d = d > f
// 21 DisElf { result_reg: "ip", asm_instruction: Add(Reg("d"), Reg("ip")) } | ip = d + ip -- skip next, goto 23
// 22 DisElf { result_reg: "ip", asm_instruction: Add(Reg("ip"), Lit(1)) } | ip = ip + 1 -- skip next, goto 24
// 23 DisElf { result_reg: "ip", asm_instruction: Set(Lit(25)) } | ip = 25 -- goto 26
// 24 DisElf { result_reg: "c", asm_instruction: Add(Reg("c"), Lit(1)) } | c = c + 1
// 25 DisElf { result_reg: "ip", asm_instruction: Set(Lit(17)) } | ip = 17 -- goto 18
// 26 DisElf { result_reg: "f", asm_instruction: Set(Reg("c")) } | f = c
// 27 DisElf { result_reg: "ip", asm_instruction: Set(Lit(7)) } | ip = 7 -- goto 8
// 28 DisElf { result_reg: "c", asm_instruction: Equal(Reg("e"), Reg("a")) } | c = e == a
// 29 DisElf { result_reg: "ip", asm_instruction: Add(Reg("c"), Reg("ip")) } | ip = ip + c -- skip next, terminate
// 30 DisElf { result_reg: "ip", asm_instruction: Set(Lit(5)) } | ip = 5 -- goto 6
// Appears to be some sort of hashing function that is working on the lower 24 bits (& 0xffffff) 8 bits at a time (& 0xff)
// Reg("a") is only used in 28, which is why watching c works. Part 1 is solved by providing
// the very first value of c. The phrasing indicates that we should run the program until we find
// a cycle in the values of c. We can provide 0 for a and just keep recording our observations of c.
// Note: This is fantastically slow, but works. Should probably figure out how to write
// this hash function in rust.

pub fn part_2(s: &str) -> Result<String> {
    let (ip_reg, program, watch) = setup(s)?;
    let mut ip: usize = 0;
    let mut registers: Registers<6> = [0usize; 6];
    let mut last = 0;
    let mut seen = FxHashSet::default();

    loop {
        let n = tick_once(&mut registers, watch, &mut ip, ip_reg, &program)
            .context("Unable to evaluate")?;
        if !seen.insert(n) {
            return Ok(last.to_string());
        }
        last = n;
    }
}
