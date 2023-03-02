use rand::Rng;
use std::fs::{metadata, File};
use std::io::Read;

use super::display_driver::Display;
use super::keyboard_driver::Keyboard;
use super::speaker_driver::Speaker;

pub struct CPU<'a, 'b, 'c> {
    display: &'a mut Display,
    keyboard: &'b Keyboard,
    speaker: &'c Speaker,
    memory: Vec<u8>,
    v: Vec<i32>,
    i: i32,
    delay_timer: i32,
    sound_timer: i32,
    pc: i32,
    stack: Vec<i32>,
    paused: bool,
    speed: u32,
}

impl<'a, 'b, 'c> CPU<'a, 'b, 'c> {
    pub fn new(display: &'a mut Display, keyboard: &'b Keyboard, speaker: &'c Speaker) -> Self {
        CPU {
            display: display,
            keyboard: keyboard,
            speaker: speaker,
            memory: vec![0; 4096],
            v: vec![0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: vec![],
            paused: false,
            speed: 10,
        }
    }

    pub fn load_sprites_into_memory(&mut self) {
        let sprites: Vec<u8> = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for i in 0..sprites.len() {
            self.memory[i] = sprites[i];
        }
    }

    pub fn load_prog_into_memory(&mut self, prog: Vec<u8>) {
        for loc in 0..prog.len() {
            self.memory[0x200 + loc] = prog[loc];
        }
    }

    pub fn load_rom(&mut self, rom_name: &str) {
        let mut f = File::open(&rom_name).expect("no rom found");
        let metadata = metadata(&rom_name).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        self.load_prog_into_memory(buffer);
    }

    pub fn cycle(&mut self) {
        for i in 0..self.speed {
            if !self.paused {
                let opcode: i32 = ((self.memory[self.pc as usize] as i32) << 8
                    | self.memory[(self.pc + 1) as usize] as i32);
                self.execute_instruction(opcode);
            }
        }

        if !self.paused {
            self.update_timers();
        }

        self.play_sound();
        self.display.render();
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn play_sound(&mut self) {
        if self.sound_timer > 0 {
            // self.speaker.play(440);
        } else {
            // self.speaker.stop();
        }
    }

    pub fn execute_instruction(&mut self, opcode: i32) {
        self.pc += 2;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    self.display.clear();
                }
                0x00EE => {
                    self.pc = self.stack.pop().unwrap();
                }
                _ => {}
            },

            0x1000 => {
                self.pc = opcode & 0xFFF;
            }

            0x2000 => {
                self.stack.push(self.pc);
                self.pc = opcode & 0xFFF;
            }

            0x3000 => {
                if self.v[x] == (opcode & 0xFF) {
                    self.pc += 2;
                }
            }

            0x4000 => {
                if self.v[x] != (opcode & 0xFF) {
                    self.pc += 2;
                }
            }

            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }

            0x7000 => {
                self.v[x] += opcode & 0xFF;
            }

            0x8000 => match opcode & 0xF {
                0x0 => {
                    self.v[x] = self.v[y];
                }

                0x1 => {
                    self.v[x] |= self.v[y];
                }

                0x2 => {
                    self.v[x] &= self.v[y];
                }

                0x3 => {
                    self.v[x] ^= self.v[y];
                }

                0x4 => {
                    self.v[x] += self.v[y];
                    let sum = self.v[x];

                    self.v[0xF] = 0;

                    if sum > 0xFF {
                        self.v[0xF] = 1;
                    }

                    self.v[x] = sum;
                }

                0x5 => {
                    self.v[0xF] = 0;

                    if self.v[x] > self.v[y] {
                        self.v[0xF] = 1;
                    }

                    self.v[x] -= self.v[y];
                }

                0x6 => {
                    self.v[0xF] = self.v[x] & 0x1;

                    self.v[x] >>= 1;
                }

                0x7 => {
                    self.v[0xF] = 0;

                    if self.v[y] > self.v[x] {
                        self.v[0xF] = 1;
                    }

                    self.v[x] = self.v[y] - self.v[x];
                }

                0xE => {
                    self.v[0xF] = self.v[x] & 0x80;
                    self.v[x] <<= 1;
                }
                _ => {}
            },

            0x9000 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }

            0xA000 => {
                self.i = opcode & 0xFFF;
            }

            0xB000 => {
                self.pc = (opcode & 0xFFF) + self.v[0];
            }

            0xC000 => {
                let mut rng = rand::thread_rng();
                let rand = rng.gen_range(0..=0xFF);

                self.v[x] = rand & (opcode & 0xFF);
            }

            0xD000 => {
                let width = 8;
                let height = opcode & 0xF;

                self.v[0xF] = 0;

                for row in 0..height {
                    let mut sprite = self.memory[(self.i + row) as usize];

                    for col in 0..width {
                        if (sprite & 0x80) > 0 {
                            if self
                                .display
                                .set_pixel((self.v[x] + col) as i32, (self.v[y] + row) as i32)
                            {
                                self.v[0xF] = 1;
                            }
                        }

                        sprite <<= 1;
                    }
                }
            }

            0xE000 => match opcode & 0xFF {
                0x9E => {
                    if self.keyboard.is_key_pressed(self.v[x]) {
                        self.pc += 2;
                    }
                }

                0xA1 => {
                    if !self.keyboard.is_key_pressed(self.v[x]) {
                        self.pc += 2;
                    }
                }
                _ => {}
            },

            0xF000 => match opcode & 0xFF {
                0x07 => {
                    self.v[x] = self.delay_timer;
                }

                0x0A => {
                    self.paused = true;

                    // self.keyboard.on_next_key_press = Some(Box::new(move |key| {
                    //     self.v[x] = key;
                    //     self.paused = false;
                    // }));
                }

                0x15 => {
                    self.delay_timer = self.v[x];
                }

                0x18 => {
                    self.sound_timer = self.v[x];
                }

                0x1E => {
                    self.i += self.v[x];
                }

                0x29 => {
                    self.i = self.v[x] * 5;
                }

                0x33 => {
                    self.memory[self.i as usize] = (self.v[x] / 100) as u8;
                    self.memory[(self.i + 1) as usize] = ((self.v[x] % 100) / 10) as u8;
                    self.memory[(self.i + 2) as usize] = (self.v[x] % 10) as u8;
                }

                0x55 => {
                    for register_index in 0..=x {
                        self.memory[((self.i as usize) + register_index)] =
                            self.v[register_index] as u8;
                    }
                }

                0x65 => {
                    for register_index in 0..=x {
                        self.v[register_index] =
                            self.memory[(self.i as usize) + register_index] as i32;
                    }
                }
                _ => {
                    println!("Unknown opcode : {}", opcode);
                }
            },

            _ => {}
        }
    }
}
