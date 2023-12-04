use crate::display;
use crate::ram;

pub struct Bus {
    ram: ram::RAM,
    display: display::Display,
    keypad: [bool; 16],
}

impl Bus {
    pub fn init() -> Bus {
        Bus {
            ram: ram::RAM::init(),
            display: display::Display::init(),
            keypad: [false; 16],
        }
    }

    // Memory
    pub fn ram_write_byte(&mut self, addr: u16, value: u8) {
        self.ram.write_byte(addr, value)
    }

    pub fn ram_read_byte(&mut self, addr: u16) -> u8 {
        self.ram.read_byte(addr)
    }

    // Display
    pub fn display_clear(&mut self) {
        self.display.clear();
    }

    pub fn display_get_pixel(&mut self, index: usize) -> u32 {
        self.display.get_pixel(index)
    }

    pub fn display_write_pixel(&mut self, index: usize, value: u32) {
        self.display.write_pixel(index, value);
    }

    // Keyboard
    pub fn is_key_pressed(&self, index: usize) -> bool {
        self.keypad[index]
    }

    pub fn handle_key_press(&mut self, index: usize, state: bool) {
        self.keypad[index] = state;
    }
}

/// For printing values
impl Bus {
    pub fn ram_print(&self) {
        self.ram.print_memory()
    }
}
