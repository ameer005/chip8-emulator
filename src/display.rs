pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const SCALE_FACTOR: u32 = 14;
pub const FG_COLOR: u32 = 0xFFFFFFFF;
pub const BG_COLOR: u32 = 0x00000000;

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

    pub fn get_pixel(&self, index: usize) -> u32 {
        self.video[index]
    }

    pub fn write_pixel(&mut self, index: usize, value: u32) {
        self.video[index] = value;
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.video
    }
}
