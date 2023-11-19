use crate::{bus, cpu};

pub struct Chip8 {
    cpu: cpu::CPU,
    bus: bus::Bus,
}

impl Chip8 {
    pub fn init() -> Chip8 {
        Chip8 {
            cpu: cpu::CPU::init(),
            bus: bus::Bus::init(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        let offset = cpu::EXECUTION_INDEX;

        for (i, val) in data.into_iter().enumerate() {
            self.bus.ram_write_byte(offset as usize + i, val)
        }

        self.bus.ram_print()
    }
}
