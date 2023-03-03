mod drivers {
    pub mod cpu_driver;
    pub mod display_driver;
    pub mod keyboard_driver;
    pub mod speaker_driver;
}

use drivers::cpu_driver::CPU;
use drivers::display_driver::Display;
use drivers::keyboard_driver::Keyboard;
use drivers::speaker_driver::Speaker;

fn main() {
    let mut display = Display::new("Chip-8 Emulator - Alpha", 20);
    let speaker = Speaker::new();
    let keyboard = Keyboard::new();
    let mut cpu = CPU::new(&mut display, &keyboard, &speaker);

    cpu.load_sprites_into_memory();
    cpu.load_rom("D:/DOCUMENTS/Desktop/Rust Projects/emulateur_chip-8/src/Airplane.ch8");

    loop {
        cpu.cycle();
    }
}