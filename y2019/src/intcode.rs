use anyhow::{anyhow, Context};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use std::num::ParseIntError;

type Instructions = Vec<i64>;
type Memory = HashMap<i64, i64>;

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Program {
    instructions: Instructions,
    memory: Memory,
    instruction_pointer: i64,
    inputs: Vec<i64>,
    input_pointer: usize,
    outputs: Vec<i64>,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operation {
    Halt,
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    Less,
    Equal,
}

impl Operation {
    fn operands(self) -> usize {
        use Operation::*;
        match self {
            Halt => 0,
            Add | Multiply | Less | Equal => 3,
            Input | Output => 1,
            JumpFalse | JumpTrue => 2,
        }
    }
}

impl TryFrom<i64> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use Operation::*;
        let opcode = value % 100;
        match opcode {
            99 => Ok(Halt),
            1 => Ok(Add),
            2 => Ok(Multiply),
            3 => Ok(Input),
            4 => Ok(Output),
            5 => Ok(JumpTrue),
            6 => Ok(JumpFalse),
            7 => Ok(Less),
            8 => Ok(Equal),
            _ => Err(anyhow!("Unknown opcode: {opcode}")),
        }
    }
}

impl Program {
    pub fn new<T>(instructions: &[T]) -> Self
    where
        T: Into<i64> + Copy,
    {
        Program {
            instructions: instructions.iter().map(|i| (*i).into()).collect_vec(),
            ..Program::default()
        }
    }

    pub fn parse(i: &str) -> Result<Program, ParseIntError> {
        let v: Result<Vec<i64>, ParseIntError> = i.split(',').map(|dig| dig.parse()).collect();
        Ok(Self::new(&v?))
    }

    pub fn input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn output(&self) -> &[i64] {
        &self.outputs
    }

    pub fn read_addr(&self, addr: i64, mode: ParameterMode) -> i64 {
        use ParameterMode::*;

        let val = self
            .memory
            .get(&addr)
            .copied()
            .or_else(|| self.instructions.get(addr as usize).copied())
            .unwrap_or(0);

        match mode {
            Position => self.read_addr(val, Immediate),
            Immediate => val,
        }
    }

    fn write_addr(&mut self, addr: i64, value: i64, mode: ParameterMode) {
        use ParameterMode::*;

        match mode {
            Immediate => {
                self.memory.insert(addr, value);
            }
            Position => {
                let addr = self.read_addr(addr, Immediate);
                self.memory.insert(addr, value);
            }
        }
    }

    fn exec_step(&mut self) -> Result<bool, anyhow::Error> {
        use Operation::*;
        use ParameterMode::*;

        let mut instr = self.read_addr(self.instruction_pointer, Immediate);
        let op: Operation = instr.try_into()?;
        instr /= 100;

        self.instruction_pointer += 1;
        let mut parameter_modes = vec![];
        while parameter_modes.len() < op.operands() {
            let mode = if instr % 10 == 0 { Position } else { Immediate };
            parameter_modes.push((self.instruction_pointer, mode));
            self.instruction_pointer += 1;
            instr /= 10;
        }

        let decode = |(addr, mode)| self.read_addr(addr, mode);
        match op {
            binop @ (Add | Multiply | Equal | Less) => {
                let lhs = decode(parameter_modes[0]);
                let rhs = decode(parameter_modes[1]);
                let result = match binop {
                    Add => lhs + rhs,
                    Multiply => lhs * rhs,
                    Equal => i64::from(lhs == rhs),
                    Less => i64::from(lhs < rhs),
                    _ => unreachable!(),
                };
                self.write_addr(parameter_modes[2].0, result, Position);
            }
            Halt => return Ok(true),
            Input => {
                let input = self.inputs[self.input_pointer];
                self.input_pointer += 1;
                self.write_addr(parameter_modes[0].0, input, parameter_modes[0].1);
            }
            Output => {
                let output = decode(parameter_modes[0]);
                self.outputs.push(output);
            }
            JumpTrue => {
                let param = decode(parameter_modes[0]);
                if param != 0 {
                    self.instruction_pointer = decode(parameter_modes[1]);
                }
            }
            JumpFalse => {
                let param = decode(parameter_modes[0]);
                if param == 0 {
                    self.instruction_pointer = decode(parameter_modes[1]);
                }
            }
        }

        Ok(false)
    }

    pub fn exec(&mut self) -> Result<(), anyhow::Error> {
        while !self.exec_step()? {}
        Ok(())
    }

    pub fn produce_output(&mut self) -> Result<Output, anyhow::Error> {
        let available_outputs = self.outputs.len();
        while self.outputs.len() == available_outputs {
            if self.exec_step()? {
                return Ok(Output::Exhausted);
            }
        }
        self.outputs
            .last()
            .context("Missing outputs")
            .copied()
            .map(Output::Value)
    }
}

pub enum Output {
    Value(i64),
    Exhausted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_tests() {
        let mut prog = Program {
            instructions: vec![1, 0, 0, 0, 99],
            ..Program::default()
        };
        prog.exec().unwrap();
        assert_eq!(prog.memory.get(&0).copied(), Some(2));
        let mut prog = Program {
            instructions: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            ..Program::default()
        };
        prog.exec().unwrap();
        assert_eq!(prog.memory.get(&0).copied(), Some(30));
    }
}
