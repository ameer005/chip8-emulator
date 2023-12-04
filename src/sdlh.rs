use crate::{
    chip8::{self, Chip8, EmulatorState},
    display,
};
use sdl2::{
    event::{self, Event},
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    video::Window,
    Sdl,
};

pub struct SDLHandler {
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
}

impl SDLHandler {
    pub fn init() -> SDLHandler {
        let mut sdl_context = sdl2::init().expect("Failed to initialize sdl");
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to create video_subsystem");

        let window = video_subsystem
            .window(
                "CHIP-8 Emulator",
                display::DISPLAY_WIDTH as u32 * display::SCALE_FACTOR,
                display::DISPLAY_HEIGHT as u32 * display::SCALE_FACTOR,
            )
            .position_centered()
            .build()
            .expect("Failed to build windows");

        let mut canvas: Canvas<Window> = window
            .into_canvas()
            .build()
            .expect("Failed to build canvas");

        let mut handler = SDLHandler {
            canvas,
            sdl: sdl_context,
        };

        // Initial screen to background colour
        handler.clear_screen();

        handler
    }

    pub fn handle_events(&self, emulator: &mut Chip8) {
        let mut event_pump = self
            .sdl
            .event_pump()
            .expect("failed to initialize event pump");

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => emulator.change_state(EmulatorState::Quit),

                Event::KeyUp { .. } => {}

                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => emulator.change_state(EmulatorState::Quit),

                    _ => println!("unknown key"),
                },

                _ => {}
            }
        }
    }

    pub fn clear_screen(&mut self) {
        let r: u8 = ((display::BG_COLOR >> 24) & 0xFF) as u8;
        let g: u8 = ((display::BG_COLOR >> 16) & 0xFF) as u8;
        let b: u8 = ((display::BG_COLOR >> 8) & 0xFF) as u8;
        let a: u8 = ((display::BG_COLOR >> 24) & 0xFF) as u8;

        let color = Color::RGBA(r, g, b, a);
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn update_screen(&self) {}
}
