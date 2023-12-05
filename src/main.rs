#![allow(unused)]

use std::{
    env,
    fs::File,
    io::Read,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::chip8::EmulatorState;

mod bus;
mod chip8;
mod cpu;
mod display;
mod ram;
mod sdlh;

fn main() {
    // create separate function to take filename as argument
    // put some error checkers
    let data = open_file();

    let mut sdl_handler = sdlh::SDLHandler::init();
    let mut chip = chip8::Chip8::init();
    chip.load_rom(data);

    let target_fps = 60;
    let frame_duration = Duration::from_secs_f64(1.0 / f64::from(target_fps));

    // main game loop
    while chip.state != chip8::EmulatorState::Quit {
        sdl_handler.handle_events(&mut chip);

        if chip.state == EmulatorState::PAUSED {
            continue;
        }

        let mut last_frame_time = Instant::now();

        // Emulate CHIP8 Instructions
        chip.run();

        // Delay
        let elapsed_time = Instant::now().duration_since(last_frame_time);
        if elapsed_time < frame_duration {
            sleep(frame_duration - elapsed_time);
        }

        // Update window with changes
        sdl_handler.update_screen(&mut chip);
    }

    println!("fuck you")
}

fn open_file() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) {
        eprintln!("CHIP8 ROM path is required")
    }

    println!("{}", args[1]);
    let mut file = File::open(&args[1]).expect("failed to open file");
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    data
}
