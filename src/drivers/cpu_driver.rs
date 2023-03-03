use rand::Rng;
use std::fs::{metadata, File};
use std::io::Read;
use std::vec;

use super::display_driver::Display;
use super::keyboard_driver::Keyboard;
use super::speaker_driver::Speaker;

pub struct CPU<'a, 'b, 'c> {
    display: &'a mut Display,
    _keyboard: &'b Keyboard,
    _speaker: &'c Speaker,
    memory: [u8; 4096],
    registers: [u8; 16],
    address: usize,
    delay_timer: u8,
    sound_timer: u8,
    pc: u32,
    stack: Vec<u32>,
    paused: bool,
    speed: u32,
}

impl<'a, 'b, 'c> CPU<'a, 'b, 'c> {
    pub fn new(display: &'a mut Display, keyboard: &'b Keyboard, speaker: &'c Speaker) -> Self {
        CPU {
            display: display,
            _keyboard: keyboard,
            _speaker: speaker,
            memory: [0; 4096],
            registers: [0; 16],
            address: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: vec![],
            paused: false,
            speed: 10,
        }
    }

    pub fn load_sprites_into_memory(&mut self) {
        // Array of hex values for each sprite. Each sprite is 5 bytes.
        // The technical reference provides us with each one of these values.
        let sprites = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2;
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
    
        // According to the technical reference, sprites are stored in the interpreter section of memory starting at hex 0x000
        for i in 0..sprites.len() {
            self.memory[i] = sprites[i];
        }
    }
    
    pub fn load_program_into_memory(&mut self, program: Vec<u8>) {
        for loc in 0..program.len() {
            self.memory[0x200 + loc] = program[loc];
        }
    }

    pub fn load_rom(&mut self, rom_name: &str) {
        let mut f = File::open(&rom_name).expect("no rom found");
        let metadata = metadata(&rom_name).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        self.load_program_into_memory(buffer);
    }

    pub fn cycle(&mut self) {
       for _ in 0..self.speed {
        if !self.paused {
            let opcode = (self.memory[self.pc as usize] as u32) << 8 | self.memory[self.pc as usize + 1] as u32;
            self.execute_instruction(opcode);
        }
       }

       if !self.paused {
        self.update_timers();
       }

    //    self.play_sound();
       self.display.render();
    }

    pub fn update_timers(&mut self) {
        if !self.delay_timer > 0 {
            self.delay_timer = u8::wrapping_sub(self.delay_timer, 1);
        }

        if !self.sound_timer > 0 {
            self.sound_timer = u8::wrapping_sub(self.sound_timer, 1);
        }
    }

    pub fn execute_instruction(&mut self, opcode: u32) {
        self.pc += 2;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self.display.clear(),
                0x00EE => self.pc = self.stack.pop().unwrap(),
                _ => {}
            }
            0x1000 => self.pc = opcode & 0xFFF,
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = opcode & 0xFFF;
            }
            0x3000 => {
                if self.registers[x] == (opcode & 0xFF) as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.registers[x] != (opcode & 0xFF) as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            0x6000 => self.registers[x] = (opcode & 0xFF) as u8,
            0x7000 => self.registers[x] = u8::wrapping_add(self.registers[x], (opcode & 0xFF) as u8),
            0x8000 => match opcode & 0xF {
                0x0 => self.registers[x] = self.registers[y],
                0x1 => self.registers[x] |= self.registers[y],
                0x2 => self.registers[x] &= self.registers[y],
                0x3 => self.registers[x] ^= self.registers[y],
                0x4 => {
                    let sum = (self.registers[x] as u32) + (self.registers[y] as u32);
                    self.registers[0xF] = 0;
                    if sum > 0xFF {
                        self.registers[0xF] = 1;
                    }
                    self.registers[x] = sum as u8;
                },
                0x5 => {
                    if self.registers[x] > self.registers[y] {
                        self.registers[0xF] = 1;
                    }

                    self.registers[x] = u8::wrapping_sub(self.registers[x], self.registers[y]);
                }
                0x6 => {
                    self.registers[0xF] = self.registers[x] & 0x1;
                    self.registers[x] >>= 1;
                }
                0x7 => {
                    self.registers[0xF] = 0;
                    if self.registers[y] > self.registers[x] {
                        self.registers[0xF] = 1;
                    }
                    self.registers[x] = self.registers[y] - self.registers[x];
                }
                0xE => {
                    self.registers[0xF] = self.registers[x] & 0x80;
                    self.registers[x] <<= 1;
                }
                _ => {}
            }
            0x9000 => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            0xA000 => self.address = (opcode & 0xFFF) as usize,
            0xB000 => self.pc = (opcode & 0xFFF) + self.registers[0] as u32,
            0xC000 => {
                let mut rng = rand::thread_rng();
                let rand: u8 = rng.gen_range(0..=0xFF);

                self.registers[x] = rand & (opcode & 0xFF) as u8;
            }
            0xD000 => {
                let width = 8;
                let height = opcode & 0xF;

                self.registers[0xF] = 0;

                for row in 0..height {
                    let mut sprite = self.memory[self.address + row as usize];

                    for col in 0..width {
                        if (sprite & 0x80) > 0 {
                            if self.display.set_pixel(
                                (self.registers[x] as i8).wrapping_add(col) as isize,
                                (self.registers[y] as i8).wrapping_add(row as i8) as isize,
                            ) {
                                self.registers[0xF] = 1;
                            }
                        }

                        sprite <<= 1;
                    }
                }
            }
            0xE000 => match opcode & 0xFF {
                0x9E => {
                    self.pc += 2;
                }
                0xA1 => {
                    self.pc += 2;
                }
                _ => {}
            }
            0xF000 => match opcode & 0xFF {
                0x07 => self.registers[x] = self.delay_timer,
                0x0A => {
                    self.paused = true;

                    // TODO : Keyboard
                }
                0x15 => self.delay_timer = self.registers[x],
                0x18 => self.sound_timer = self.registers[x],
                0x1E => self.address += self.registers[x] as usize,
                0x29 => self.address = self.registers[x] as usize * 5,
                0x33 => {
                    self.memory[self.address as usize] = self.registers[x] / 100;
                    self.memory[(self.address + 1) as usize] = (self.registers[x] % 100) / 10;
                    self.memory[(self.address + 2) as usize] = self.registers[x] % 10;
                }
                0x55 => {
                    for register_idx in 0..=x {
                        self.memory[self.address + register_idx] = self.registers[register_idx];
                    }
                }
                0x65 => {
                    for register_idx in 0..=x {
                        self.registers[register_idx] = self.memory[self.address + register_idx];
                    }
                }
                _ => {}
            }
            _ => println!("Unknow opcode : {}", opcode)
        }
    }
}