pub struct Display {
    video: [u32; 64 * 32],
}

impl Display {
    pub fn init() -> Display {
        Display {
            video: [0; 64 * 32],
        }
    }
}
