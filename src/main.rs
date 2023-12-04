#![allow(unused)]

use std::{
    fs::File,
    io::Read,
    thread::sleep,
    time::{Duration, Instant},
};

mod bus;
mod chip8;
mod cpu;
mod display;
mod ram;
mod sdlh;

fn main() {
    let mut file = File::open("data/PONG").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    let mut sdl_handler = sdlh::SDLHandler::init();
    let mut chip = chip8::Chip8::init();
    chip.load_rom(data);

    let target_fps = 60;
    let frame_duration = Duration::from_secs_f64(1.0 / f64::from(target_fps));

    // main game loop
    while chip.state != chip8::EmulatorState::Quit {
        let mut last_frame_time = Instant::now();
        // Emulate CHIP8 Instructions

        // Delay
        let elapsed_time = Instant::now().duration_since(last_frame_time);
        if elapsed_time < frame_duration {
            sleep(frame_duration - elapsed_time);
        }

        // Update window with changes

        sdl_handler.handle_events(&mut chip);
    }

    println!("fuck you")
}
