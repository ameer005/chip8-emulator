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

        // self.bus.ram_print()
    }

    pub fn exec_instructions(&mut self, opcode: u16) {
        let left_nibble = (opcode & 0xF000) >> 12;

        match left_nibble {
            0x0 => match opcode {
                /// Clearing display
                0x00E0 => self.bus.display_clear(),

                0x00EE => {
                    let sp = self.cpu.get_sp();
                    let stack_value = self.cpu.get_stack_value(sp);
                    self.cpu.update_pc(stack_value);
                    self.cpu.decrease_sp();
                }
                _ => println!("{}", "unknown op code"),
            },

            /// 1nnn
            /// set program couter to lowest 12 bit of opcode
            0x1 => {
                let addr = opcode & 0x0FFF;
                self.cpu.update_pc(addr);
            }

            /// 2nnn
            0x2 => {
                let addr = opcode & 0x0FFF;

                self.cpu.increase_sp();
                self.cpu
                    .set_stack_value(self.cpu.get_sp(), self.cpu.get_pc());

                self.cpu.update_pc(addr);
            }

            /// 3xkk
            0x3 => {
                let vx = opcode & 0x0F00;
                let kk = opcode & 0x00FF;

                if self.cpu.get_vreg_value(vx as u8) == kk as u8 {
                    self.cpu.update_pc(self.cpu.get_pc() + 2)
                }
            }

            _ => println!("Unknown opcode"),
        }
    }
}

// Test
#[cfg(test)]
mod chip8_tests {
    use crate::chip8::Chip8;

    #[test]
    fn test_return_instruction() {
        let mut chip8 = Chip8::init();

        // mocking
        chip8.cpu.increase_sp();
        let sp = chip8.cpu.get_sp();
        chip8.cpu.set_stack_value(sp, 302);

        //result
        chip8.exec_instructions(0x00EE);
        assert_eq!(chip8.cpu.get_sp(), 0);
        assert_eq!(chip8.cpu.get_pc(), 302);
    }

    #[test]
    fn test_1nnn() {
        let mut chip8 = Chip8::init();

        //mock
        let addr = 0x1300 & 0x0FFF;
        chip8.exec_instructions(0x1300);

        assert_eq!(chip8.cpu.get_pc(), addr);
    }

    #[test]
    fn test_2nnn() {
        let addr = 0x2300 & 0x0FFF;

        let mut chip8 = Chip8::init();

        //mock
        chip8.cpu.update_pc(23);
        chip8.exec_instructions(0x2300);

        assert_eq!(chip8.cpu.get_sp(), 1);
        assert_eq!(chip8.cpu.get_pc(), addr);
        assert_eq!(chip8.cpu.get_stack_value(chip8.cpu.get_sp()), 23);
    }

    #[test]
    fn test_3xkk() {
        let mut chip8 = Chip8::init();

        //mock
        let opcode: u16 = 0x3242;
        let vx = opcode & 0x0F00;
        let kk = opcode & 0x00FF;

        chip8.cpu.set_vreg_value(vx as u8, kk as u8);
        chip8.cpu.update_pc(0x2000);
        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_pc(), 0x2000 + 2);
    }
}
