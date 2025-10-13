#![allow(non_snake_case)]

use macroquad::prelude::*;

use std::{env, fs::File, io::{BufReader, Read}};

const SCREEN_LENGTH: usize  = 16;
const PIXEL_SIZE: usize = 80;

struct RISCZ {
    registers: [u8; 16],
    memory: Vec<[u8; 256]>,
    inst_memory: [u16; 4096],
    pc: u16,
    stack: Vec<u16>,
    result_flag: bool,
    mem_page: u8,
}

impl RISCZ {
    fn new() -> Self {
        return RISCZ {
            registers: [0; 16],
            memory: vec![[0; 256]; 256],
            inst_memory: [0; 4096],
            pc: 0,
            stack: Vec::new(),
            result_flag: false,
            mem_page: 0,
        };
    }

    fn load_file(&mut self, filename: &str) {
        let f = File::open(filename).expect("File not found");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("Failed to read file");
        // Convert Vec<u8> to u16's in the inst memory
        let mut index = 0;
        for i in (0..buffer.len()).step_by(2) {
            self.inst_memory[index] = ((buffer[i] as u16) << 8) | buffer[i + 1] as u16;
            index += 1;
        }
    }

    fn tick(&mut self) {
        let opcode = (self.inst_memory[self.pc as usize] & 0xF000) >> 12;
        let r1 = ((self.inst_memory[self.pc as usize] & 0x0F00) >> 8) as u8;
        let r2 = ((self.inst_memory[self.pc as usize] & 0x00F0) >> 4) as u8;
        let r3 = ((self.inst_memory[self.pc as usize] & 0x000F) >> 0) as u8;
        let a1 = (self.inst_memory[self.pc as usize] & 0x0FFF) >> 0;
        let v1 = ((self.inst_memory[self.pc as usize] & 0x00FF) >> 0) as u8;

        self.pc += 1;

        match opcode {
            0x0 => self.OP_ADD(r1, r2, r3),
            0x1 => self.OP_SUB(r1, r2, r3),
            0x2 => self.OP_DIV(r1, r2, r3),
            0x3 => self.OP_AND(r1, r2, r3),
            0x4 => self.OP_ORR(r1, r2, r3),
            0x5 => self.OP_XOR(r1, r2, r3),
            0x6 => self.OP_NOT(r1, r2),
            0x7 => self.OP_LSH(r1, r2, r3),
            0x8 => self.OP_RSH(r1, r2, r3),
            0x9 => self.OP_RET(),
            0xA => self.OP_BIR(a1),
            0xB => self.OP_LDM(r1, r2),
            0xC => self.OP_STR(r1, r2),
            0xD => self.OP_LDI(r1, v1),
            0xE => self.OP_CMP(r1, r2, r3),
            0xF => self.OP_SPG(r1),
            _ => panic!("Invalid opcode {} on line {}", opcode, self.pc),
        }

        // If branch, decrease pc
        if opcode == 0xA {
            self.pc -= 1;
        }
    }

    fn read_reg(&self, reg_number: u8) -> u8{
        return match reg_number {
            0 => 0,
            1..=15 => self.registers[reg_number as usize],
            _ => panic!("Invalid register read"),
        }
    }

    fn OP_ADD(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2).wrapping_add(self.read_reg(r3));
    }

    fn OP_SUB(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2).wrapping_sub(self.read_reg(r3));
    }

    fn OP_DIV(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2) / self.read_reg(r3);
    }

    fn OP_AND(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2) & self.read_reg(r3);
    }

    fn OP_ORR(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2) | self.read_reg(r3);
    }

    fn OP_XOR(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2) ^ self.read_reg(r3);
    }

    fn OP_NOT(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = !self.read_reg(r2);
    }

    fn OP_LSH(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2) << self.read_reg(r3);
    }

    fn OP_RSH(&mut self, r1: u8, r2: u8, r3: u8) {
        self.registers[r1 as usize] = self.read_reg(r2) >> self.read_reg(r3);
    }

    fn OP_RET(&mut self) {
        // Do nothing on an empty stack
        if self.stack.len() > 0 {
            self.pc = self.stack.pop().unwrap();
        }
    }

    fn OP_BIR(&mut self, a1: u16) {
        if self.result_flag {
            self.stack.push(a1);
            self.pc = a1;
        }
    }

    fn OP_LDM(&mut self, r1: u8, r2: u8) {
        self.registers[r1 as usize] = self.memory[self.mem_page as usize][self.read_reg(r2) as usize];
    }

    fn OP_STR(&mut self, r1: u8, r2: u8) {
        let reg_value = self.read_reg(r1);
        self.memory[self.mem_page as usize][reg_value as usize] = self.read_reg(r2);
    }

    fn OP_LDI(&mut self, r1: u8, v1: u8) {
        self.registers[r1 as usize] = v1;
    }

    fn OP_CMP(&mut self, r1: u8, r2: u8, r3: u8) {
        self.result_flag = match self.read_reg(r1) {
            0 => self.read_reg(r2) == self.read_reg(r3),
            1 => self.read_reg(r2) > self.read_reg(r3),
            2 => self.read_reg(r2) < self.read_reg(r3),
            3 => self.read_reg(r2) >= self.read_reg(r3),
            4 => self.read_reg(r2) <= self.read_reg(r3),
            5 => self.read_reg(r2) != self.read_reg(r3),
            _ => panic!("Invalid comparison type"),
        }
    }

    fn OP_SPG(&mut self, r1: u8) {
        self.mem_page = self.read_reg(r1);
    }
}

async fn do_graphics(pixels: &[u8]) {
    clear_background(BLACK);
    for i in 0..SCREEN_LENGTH {
        for j in 0..SCREEN_LENGTH {
            let pixel = pixels[i * SCREEN_LENGTH + j];
            let r = ((pixel & 0b11100000) >> 5) as f32;
            let g = ((pixel & 0b11100) >> 2) as f32;
            let b = (pixel & 0b11) as f32;
            let color = Color::new(r / 7.0, g / 7.0, b / 3.0, 1.0);
            draw_rectangle(((j as usize) * PIXEL_SIZE) as f32, ((i as usize) * PIXEL_SIZE) as f32, PIXEL_SIZE as f32, PIXEL_SIZE as f32, color);
        }
    }
}

#[macroquad::main("RISC-Z")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut riscz = RISCZ::new();
    riscz.load_file(filename);
    while (riscz.pc as usize) < riscz.inst_memory.len() {
        riscz.tick();
        do_graphics(&riscz.memory[255]).await;
        next_frame().await;
    }
}
