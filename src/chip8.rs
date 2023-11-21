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
                let vx = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;

                if self.cpu.get_vreg_value(vx as u8) == kk as u8 {
                    self.cpu.update_pc(self.cpu.get_pc() + 2)
                }
            }

            /// Skip next instruction if Vx != kk.
            0x4 => {
                let vx = (opcode & 0x0F00) >> 8;
                let kk = opcode & 0x00FF;

                if self.cpu.get_vreg_value(vx as u8) != kk as u8 {
                    self.cpu.update_pc(self.cpu.get_pc() + 2)
                }
            }

            /// Skip next instruction if Vx = Vy
            0x5 => {
                let vx = (opcode & 0x0F00) >> 8;
                let vy = (opcode & 0x00F0) >> 4;

                if self.cpu.get_vreg_value(vx as u8) == self.cpu.get_vreg_value(vy as u8) {
                    self.cpu.update_pc(self.cpu.get_pc() + 2);
                }
            }

            ///Set Vx = kk
            0x6 => {
                let vx = (opcode & 0x0F00) >> 8;
                let value = opcode & 0x00ff;

                self.cpu.set_vreg_value(vx as u8, value as u8)
            }

            ///Set Vx = Vx + kk.
            0x7 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let value = opcode & 0x00ff;

                self.cpu
                    .set_vreg_value(vx, self.cpu.get_vreg_value(vx) + value as u8)
            }

            /// Nested
            0x8 => {
                let op8 = opcode & 0x000F;

                match op8 {
                    /// Set Vx = Vy.
                    0x0 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        self.cpu.set_vreg_value(vx, self.cpu.get_vreg_value(vy));
                    }

                    // Set Vx = Vx OR Vy
                    0x1 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        let value = self.cpu.get_vreg_value(vx) | self.cpu.get_vreg_value(vy);

                        self.cpu.set_vreg_value(vx, value);
                    }

                    /// Set Vx = Vx AND Vy.
                    0x2 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        let value = self.cpu.get_vreg_value(vx) & self.cpu.get_vreg_value(vy);

                        self.cpu.set_vreg_value(vx, value);
                    }

                    /// Set Vx = Vx XOR Vy.
                    0x3 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        let value = self.cpu.get_vreg_value(vx) ^ self.cpu.get_vreg_value(vy);

                        self.cpu.set_vreg_value(vx, value);
                    }

                    ///Set Vx = Vx + Vy, set VF = carry.
                    0x4 => {
                        let vx = ((opcode & 0x0F00) >> 8) as u8;
                        let vy = ((opcode & 0x00F0) >> 4) as u8;

                        let result =
                            self.cpu.get_vreg_value(vx) as u16 + self.cpu.get_vreg_value(vy) as u16;

                        let vf = if result > 255 { 1 } else { 0 };

                        self.cpu.set_vreg_value(0xF, vf);
                        self.cpu.set_vreg_value(vx, (result & 0x00ff) as u8);
                    }

                    _ => println!("unknown opcode"),
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
        let vx = (opcode & 0x0F00) >> 8;
        let kk = opcode & 0x00FF;
        chip8.cpu.update_pc(0x2000);

        // kk is not equal to the value at Vx register
        chip8.cpu.set_vreg_value(vx as u8, (kk + 1) as u8);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), 0x2000);

        // if kk is equal to the value at Vx register
        chip8.cpu.set_vreg_value(vx as u8, kk as u8);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), 0x2000 + 2);
    }

    #[test]
    fn test_4xkk() {
        let mut chip8 = Chip8::init();

        //testing if kk is equal to value at reg Vx
        //mock
        let opcode: u16 = 0x4000;
        let vx = (opcode & 0x0F00) >> 8;
        let kk = opcode & 0x00FF;

        chip8.cpu.set_vreg_value(vx as u8, kk as u8);
        chip8.cpu.update_pc(0x2000);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), 0x2000);

        //if kk is not equal to register at vx
        chip8.cpu.set_vreg_value(vx as u8, (kk + 2) as u8);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), 0x2000 + 2);
    }

    #[test]
    fn test_5xy0() {
        let mut chip8 = Chip8::init();
        let opcode = 0x5370;
        let vx = (opcode & 0x0F00) >> 8;
        let vy = (opcode & 0x00F0) >> 4;
        chip8.cpu.update_pc(0x2000);

        //if Vx is not equal to Vy
        chip8.cpu.set_vreg_value(vx as u8, 5);
        chip8.cpu.set_vreg_value(vy as u8, 8);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), 0x2000, "should not be equal");

        //if Vx is equal to Vy
        chip8.cpu.set_vreg_value(vx as u8, 5);
        chip8.cpu.set_vreg_value(vy as u8, 5);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), 0x2000 + 2, "should be equal");
    }

    #[test]
    fn test_6xkk() {
        let mut chip8 = Chip8::init();
        let opcode = 0x65CD;

        let vx = (opcode & 0x0F00) >> 8;
        let value = opcode & 0x00ff;

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(vx as u8), value as u8);
    }

    #[test]
    fn test_7xkk() {
        let mut chip8 = Chip8::init();
        let opcode = 0x7242;
        let vx = (opcode & 0x0F00) >> 8;
        let value = opcode & 0x00ff;

        chip8.cpu.set_vreg_value(vx as u8, 4);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(vx as u8), value as u8 + 4);
    }

    #[test]
    fn test_8xy0() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8370;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vy, 8);
        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_vreg_value(vx), 8);
    }

    #[test]
    fn test_8xy1() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8371;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 30);
        chip8.cpu.set_vreg_value(vy, 100);

        let value = chip8.cpu.get_vreg_value(vx) | chip8.cpu.get_vreg_value(vy);

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(vx), value);
    }

    #[test]
    fn test_8xy2() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8372;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 30);
        chip8.cpu.set_vreg_value(vy, 100);

        let value = chip8.cpu.get_vreg_value(vx) & chip8.cpu.get_vreg_value(vy);

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(vx), value);
    }

    #[test]
    fn test_8xy3() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8373;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 30);
        chip8.cpu.set_vreg_value(vy, 100);

        let value = chip8.cpu.get_vreg_value(vx) ^ chip8.cpu.get_vreg_value(vy);

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(vx), value);
    }

    #[test]
    fn test_8xy4_carry() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8374;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 200);
        chip8.cpu.set_vreg_value(vy, 100);

        let result = chip8.cpu.get_vreg_value(vx) as u16 + chip8.cpu.get_vreg_value(vy) as u16;

        let vf = if result > 255 { 1 } else { 0 };

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(vx), (result & 0x00ff) as u8);
        assert_eq!(chip8.cpu.get_vreg_value(0xF), 1, "should be 1")
    }

    #[test]
    fn test_8xy4_nocarry() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8374;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 20);
        chip8.cpu.set_vreg_value(vy, 100);

        let result = chip8.cpu.get_vreg_value(vx) as u16 + chip8.cpu.get_vreg_value(vy) as u16;

        let vf = if result > 255 { 1 } else { 0 };

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(vx), (result & 0x00ff) as u8);
        assert_eq!(chip8.cpu.get_vreg_value(0xF), 0, "should be 0")
    }
}
