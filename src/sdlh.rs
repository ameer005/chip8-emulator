use crate::{
    chip8::{self, Chip8, EmulatorState},
    display,
};
use sdl2::{
    event::{self, Event},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
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

                Event::KeyUp {
                    keycode: Some(key), ..
                } => {}

                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => emulator.change_state(EmulatorState::Quit),

                    Keycode::Space => {
                        if emulator.state == EmulatorState::Running {
                            emulator.change_state(EmulatorState::PAUSED);
                            println!("=== PAUSED ====")
                        } else {
                            emulator.change_state(EmulatorState::Running);
                            println!("=== RUNNING ====")
                        }
                    }

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

    pub fn update_screen(&mut self, emulator: &mut Chip8) {
        let bg_r: u8 = ((display::BG_COLOR >> 24) & 0xFF) as u8;
        let bg_g: u8 = ((display::BG_COLOR >> 16) & 0xFF) as u8;
        let bg_b: u8 = ((display::BG_COLOR >> 8) & 0xFF) as u8;
        let bg_a: u8 = (display::BG_COLOR & 0xFF) as u8;

        let fg_r: u8 = ((display::FG_COLOR >> 24) & 0xFF) as u8;
        let fg_g: u8 = ((display::FG_COLOR >> 16) & 0xFF) as u8;
        let fg_b: u8 = ((display::FG_COLOR >> 8) & 0xFF) as u8;
        let fg_a: u8 = (display::FG_COLOR & 0xFF) as u8;

        let fg_color = Color::RGBA(fg_r, fg_g, fg_b, fg_a);
        let bg_color = Color::RGBA(bg_r, bg_g, bg_b, bg_a);

        let video_buffer = emulator.get_video_buffer();

        for i in 0..video_buffer.len() {
            // extracting x and y coords with correct scale factor
            let x = (i % display::DISPLAY_WIDTH) as i32 * display::SCALE_FACTOR as i32;
            let y = (i / display::DISPLAY_WIDTH) as i32 * display::SCALE_FACTOR as i32;

            let rect = Rect::new(x, y, display::SCALE_FACTOR, display::SCALE_FACTOR);

            if video_buffer[i] == 1 {
                self.canvas.set_draw_color(fg_color);
            } else {
                self.canvas.set_draw_color(bg_color);
            }

            self.canvas
                .fill_rect(rect)
                .expect("failed to fill rectangle")
        }

        self.canvas.present();
    }
}
