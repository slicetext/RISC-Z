#![allow(non_snake_case)]

use std::{env, fs::File, io::{self, BufReader, Read}};

struct RISCZ {
    registers: [u8; 16],
    memory: [u8; 256],
    inst_memory: [u16; 4096],
    pc: u16,
    stack: Vec<u16>,
    result_flag: bool,
}

impl RISCZ {
    fn new() -> Self {
        return RISCZ {
            registers: [0; 16],
            memory: [0; 256],
            inst_memory: [0; 4096],
            pc: 0,
            stack: Vec::new(),
            result_flag: false,
        };
    }

    fn OP_NOP() {
    }

    fn OP_ADD(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2.wrapping_add(r3);
    }

    fn OP_SUB(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2.wrapping_sub(r3);
    }

    fn OP_DIV(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2.wrapping_div(r3);
    }

    fn OP_AND(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2 & r3;
    }

    fn OP_ORR(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2 | r3;
    }

    fn OP_XOR(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2 ^ r3;
    }

    fn OP_NOT(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = !r2;
    }

    fn OP_LSH(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2 << r3;
    }

    fn OP_RSH(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = r2 >> r3;
    }

    fn OP_RET(&mut self) {
        self.pc = self.stack.pop().expect("Called RET (return) on an empty stack");
    }

    fn OP_BIR(&mut self, a1: u16) {
        if self.result_flag {
            self.stack.push(a1);
            self.pc = a1;
        }
    }

    fn OP_LDM(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = self.memory[self.registers[r2 as usize] as usize];
    }

    fn OP_STR(&mut self, r1: u8, r2: u8) {
        self.memory[self.registers[r1 as usize] as usize] = self.registers[r2 as usize];
    }

    fn OP_LDI(&mut self, r1: u8, v1: u8) {
        self.registers[r1 as usize] = v1;
    }

    fn OP_CMP(&mut self, r1: u8, r2: u8, r3: u8) {
        self.result_flag = match r1 {
            0 => self.registers[r2 as usize] == self.registers[r3 as usize],
            1 => self.registers[r2 as usize] > self.registers[r3 as usize],
            2 => self.registers[r2 as usize] < self.registers[r3 as usize],
            3 => self.registers[r2 as usize] >= self.registers[r3 as usize],
            4 => self.registers[r2 as usize] <= self.registers[r3 as usize],
            _ => panic!("Invalid comparison type"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).expect("File not found");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Failed to read file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut cpu = RISCZ::new();
    }
}
