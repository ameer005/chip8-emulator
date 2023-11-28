use crate::display;
use crate::ram;

pub struct Bus {
    ram: ram::RAM,
    display: display::Display,
}

impl Bus {
    pub fn init() -> Bus {
        Bus {
            ram: ram::RAM::init(),
            display: display::Display::init(),
        }
    }

    pub fn ram_write_byte(&mut self, addr: usize, value: u8) {
        self.ram.write_byte(addr, value)
    }

    pub fn ram_read_byte(&mut self, addr: u16) -> u8 {
        self.ram.read_byte(addr)
    }

    pub fn display_clear(&mut self) {
        self.display.clear();
    }

    pub fn display_get_pixel(&mut self, index: usize) -> u32 {
        self.display.get_pixel(index)
    }

    pub fn display_write_pixel(&mut self, index: usize, value: u32) {
        self.display.write_pixel(index, value);
    }
}

/// For printing values
impl Bus {
    pub fn ram_print(&self) {
        self.ram.print_memory()
    }
}
