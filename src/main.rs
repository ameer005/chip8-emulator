#![allow(unused)]

use std::{fs::File, io::Read};

mod bus;
mod chip8;
mod cpu;
mod display;
mod ram;

fn main() {
    let mut file = File::open("data/PONG").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    let mut chip = chip8::Chip8::init();
    chip.load_rom(data);

    chip.exec_instructions(0x2345);
}
