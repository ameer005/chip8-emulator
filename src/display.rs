const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    video: [u32; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Display {
    pub fn init() -> Display {
        Display {
            video: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.video = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT]
    }
}
