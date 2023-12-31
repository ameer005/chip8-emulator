use crate::{bus, cpu, display};
use rand::prelude::*;

pub const INSTRUCTIONS_PER_SECOND: u32 = 500;

#[derive(Eq, PartialEq)]
pub enum EmulatorState {
    Quit,
    Running,
    PAUSED,
}

pub struct Chip8 {
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
    pub state: EmulatorState,
}

impl Chip8 {
    pub fn init() -> Chip8 {
        Chip8 {
            cpu: cpu::CPU::init(),
            bus: bus::Bus::init(),
            state: EmulatorState::Running,
        }
    }

    pub fn run(&mut self) {
        let first_byte = self.bus.ram_read_byte(self.cpu.get_pc()) as u16;
        let second_byte = self.bus.ram_read_byte(self.cpu.get_pc() + 1) as u16;

        let opcode = (first_byte << 8) | second_byte;

        self.cpu.increment_pc();
        self.exec_instructions(opcode);
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        let offset = cpu::EXECUTION_INDEX;

        for (i, val) in data.into_iter().enumerate() {
            self.bus.ram_write_byte(offset as u16 + i as u16, val)
        }

        // self.bus.ram_print()
    }

    pub fn update_timer(&mut self) {
        if (self.cpu.get_delay_timer() > 0) {
            self.cpu.decrease_delay_timer();
        }

        if (self.cpu.get_sound_timer() > 0) {
            self.cpu.decrease_sound_timer();
        }
    }

    pub fn change_state(&mut self, state: EmulatorState) {
        self.state = state
    }

    pub fn get_video_buffer(&self) -> &[u32] {
        self.bus.display_get_buffer()
    }

    pub fn exec_instructions(&mut self, opcode: u16) {
        let left_nibble = (opcode & 0xF000) >> 12;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;
        let n = (opcode & 0x000F) as u8;

        match left_nibble {
            0x0 => match opcode {
                /// Clearing display
                0x00E0 => {
                    debug_opcodes(&opcode, "00E0", "Clear the display.");

                    self.bus.display_clear()
                }

                0x00EE => {
                    debug_opcodes(&opcode, "00EE", "Return from a subroutine.");

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
                debug_opcodes(
                    &opcode,
                    "1nnn",
                    "set program couter to lowest 12 bit of opcode",
                );

                self.cpu.update_pc(nnn);
            }

            /// 2nnn
            0x2 => {
                debug_opcodes(&opcode, "2nnn", "Call subroutine at nnn.");

                self.cpu.increase_sp();
                self.cpu
                    .set_stack_value(self.cpu.get_sp(), self.cpu.get_pc());

                self.cpu.update_pc(nnn);
            }

            /// 3xkk
            0x3 => {
                debug_opcodes(&opcode, "3xkk", "Skip next instruction if Vx = kk.");

                if self.cpu.get_vreg_value(vx) == kk {
                    self.cpu.update_pc(self.cpu.get_pc() + 2)
                }
            }

            /// Skip next instruction if Vx != kk.
            0x4 => {
                debug_opcodes(&opcode, "4xkk", "Skip next instruction if Vx != kk");

                if self.cpu.get_vreg_value(vx) != kk {
                    self.cpu.update_pc(self.cpu.get_pc() + 2)
                }
            }

            /// Skip next instruction if Vx = Vy
            0x5 => {
                debug_opcodes(&opcode, "5xy0", "Skip next instruction if Vx = Vy.");

                if self.cpu.get_vreg_value(vx) == self.cpu.get_vreg_value(vy) {
                    self.cpu.update_pc(self.cpu.get_pc() + 2);
                }
            }

            ///Set Vx = kk
            0x6 => {
                debug_opcodes(&opcode, "6xkk", "Set Vx = kk.");

                self.cpu.set_vreg_value(vx, kk)
            }

            ///Set Vx = Vx + kk.
            0x7 => {
                debug_opcodes(&opcode, "7xkk", "Set Vx = Vx + kk.");

                self.cpu
                    .set_vreg_value(vx, self.cpu.get_vreg_value(vx).wrapping_add(kk))
            }

            /// Nested
            0x8 => {
                debug_opcodes(&opcode, "8xy0", "Set Vx = Vy.");

                let op8 = opcode & 0x000F;

                match op8 {
                    /// Set Vx = Vy.
                    0x0 => {
                        debug_opcodes(&opcode, "8xy0", "Set Vx = Vy.");

                        self.cpu.set_vreg_value(vx, self.cpu.get_vreg_value(vy));
                    }

                    // Set Vx = Vx OR Vy
                    0x1 => {
                        debug_opcodes(&opcode, "8xy1", "Set Vx = Vx OR Vy");

                        let value = self.cpu.get_vreg_value(vx) | self.cpu.get_vreg_value(vy);

                        self.cpu.set_vreg_value(vx, value);
                    }

                    /// Set Vx = Vx AND Vy.
                    0x2 => {
                        debug_opcodes(&opcode, "8xy2", "Set Vx = Vx AND Vy");

                        let value = self.cpu.get_vreg_value(vx) & self.cpu.get_vreg_value(vy);

                        self.cpu.set_vreg_value(vx, value);
                    }

                    /// Set Vx = Vx XOR Vy.
                    0x3 => {
                        debug_opcodes(&opcode, "8xy3", "Set Vx = Vx XOR Vy.");

                        let value = self.cpu.get_vreg_value(vx) ^ self.cpu.get_vreg_value(vy);

                        self.cpu.set_vreg_value(vx, value);
                    }

                    ///Set Vx = Vx + Vy, set VF = carry.
                    0x4 => {
                        debug_opcodes(&opcode, "8xy4", "Set Vx = Vx + Vy, set VF = carry.");

                        let result =
                            self.cpu.get_vreg_value(vx) as u16 + self.cpu.get_vreg_value(vy) as u16;

                        let vf = if result > 0xFF { 1 } else { 0 };

                        self.cpu.set_vreg_value(vx, result as u8);
                        self.cpu.set_vreg_value(0xF, vf);
                    }

                    // Set Vx = Vx - Vy, set VF = NOT borrow.
                    0x5 => {
                        debug_opcodes(&opcode, "8xy5", "Set Vx = Vx - Vy, set VF = NOT borrow");

                        let vf = if self.cpu.get_vreg_value(vx) >= self.cpu.get_vreg_value(vy) {
                            1
                        } else {
                            0
                        };

                        let value = self
                            .cpu
                            .get_vreg_value(vx)
                            .wrapping_sub(self.cpu.get_vreg_value(vy));

                        self.cpu.set_vreg_value(vx, value);
                        self.cpu.set_vreg_value(0xF, vf);
                    }

                    //Set Vx = Vx SHR 1.
                    //wrong
                    0x6 => {
                        debug_opcodes(&opcode, "8xy6", "Set Vx = Vx SHR 1.");

                        // Store the least significant bit of Vx in VF
                        self.cpu
                            .set_vreg_value(0xF, self.cpu.get_vreg_value(vx) & 0x1);

                        // Shift Vx 1 bit to the right
                        self.cpu
                            .set_vreg_value(vx, self.cpu.get_vreg_value(vx) >> 1);
                    }

                    /// Set Vx = Vy - Vx, set VF = NOT borrow.
                    // TODO not correct
                    0x7 => {
                        debug_opcodes(&opcode, "8xy7", "Set Vx = Vy - Vx, set VF = NOT borrow.");
                        // THINK :- might want to clamp it to zero if it goes below then 0
                        let value = self
                            .cpu
                            .get_vreg_value(vy)
                            .wrapping_sub(self.cpu.get_vreg_value(vx));

                        let vf = if self.cpu.get_vreg_value(vy) >= self.cpu.get_vreg_value(vx) {
                            1
                        } else {
                            0
                        };

                        self.cpu.set_vreg_value(vx, value);
                        self.cpu.set_vreg_value(0xF, vf);
                    }

                    // Set Vx = Vx SHL 1.
                    // wrong
                    0xE => {
                        debug_opcodes(&opcode, "8xyE", "Set Vx = Vx SHL 1.");

                        self.cpu
                            .set_vreg_value(0xF, self.cpu.get_vreg_value(vx) >> 7);

                        self.cpu
                            .set_vreg_value(vx, self.cpu.get_vreg_value(vx) << 1);
                    }

                    _ => println!("unknown opcode"),
                }
            }

            /// Skip next instruction if Vx != Vy.
            0x9 => {
                if self.cpu.get_vreg_value(vx) != self.cpu.get_vreg_value(vy) {
                    self.cpu.update_pc(self.cpu.get_pc() + 2);
                }
            }

            /// Set I = nnn.
            0xA => {
                debug_opcodes(&opcode, "Annn", "Set I = nnn.");
                self.cpu.set_i_reg_value(opcode & 0x0FFF)
            }

            /// Jump to location nnn + V0.
            0xB => {
                debug_opcodes(&opcode, "Bnnn", "Jump to location nnn + V0.");

                let addr = self.cpu.get_vreg_value(0) as u16 + nnn;
                self.cpu.update_pc(addr);
            }

            /// Set Vx = random byte AND kk.
            0xC => {
                debug_opcodes(&opcode, "Cxkk", "Set Vx = random byte AND kk");

                let value = gen_random_byte() & kk as u8;

                self.cpu.set_vreg_value(vx, value);
            }

            /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            0xD => {
                debug_opcodes(&opcode, "Dxyn", "Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.");

                let x_reg = self.cpu.get_vreg_value(vx);
                let y_reg = self.cpu.get_vreg_value(vy);
                let i_addr = self.cpu.get_i_reg_value();

                // reading n bytes from memory starting at i_addr
                for row in 0..n {
                    // extracting bytes one by one
                    let sprite_byte = self.bus.ram_read_byte(i_addr + row as u16);

                    // each byte is made up of 8 bits. it loop over each bytes
                    // XOR each pixel. it means if the current pixel is 1 it will set it to zero
                    for col in 0..8 {
                        // modulo operator dealing with oveflowing. it used to wrap the value.
                        let pixel_x = (x_reg + col) % display::DISPLAY_WIDTH as u8;
                        let pixel_y = (y_reg + row) % display::DISPLAY_HEIGHT as u8;

                        // basic way to get correct coordinates from 1d array
                        let index = pixel_x as usize + (pixel_y as usize * display::DISPLAY_WIDTH);
                        let current_pixel = self.bus.display_get_pixel(index);

                        // XOR each bit with current pixel and updating the display
                        let value = current_pixel ^ (sprite_byte as u32 >> (7 - col)) & 0x1;
                        self.bus.display_write_pixel(index, value);

                        if value == 1 {
                            self.cpu.set_vreg_value(0xF, 1);
                        }
                    }
                }

                // println!("{:?}", self.bus.display_get_buffer())
            }

            /// Skip next instruction if key with the value of Vx is pressed.
            0xE => {
                let op_e = opcode & 0x000F;
                match op_e {
                    0xE => {
                        debug_opcodes(
                            &opcode,
                            "Ex9E",
                            "Skip next instruction if key with the value of Vx is pressed.",
                        );

                        let x_reg = self.cpu.get_vreg_value(vx);

                        if self.bus.is_key_pressed(x_reg as usize) {
                            self.cpu.increment_pc();
                        }
                    }

                    0x1 => {
                        debug_opcodes(
                            &opcode,
                            "ExA1",
                            "Skip next instruction if key with the value of Vx is pressed.",
                        );

                        let x_reg = self.cpu.get_vreg_value(vx);

                        if !self.bus.is_key_pressed(x_reg as usize) {
                            self.cpu.increment_pc();
                        }
                    }

                    _ => println!("unknown opcode"),
                }
            }

            0xF => {
                let op_e = opcode & 0x00FF;
                match op_e {
                    /// Set Vx = delay timer value.
                    0x07 => {
                        debug_opcodes(&opcode, "Fx07", "Set Vx = delay timer value.");

                        self.cpu.set_vreg_value(vx, self.cpu.get_delay_timer());
                    }

                    /// Wait for a key press, store the value of the key in Vx.
                    0x0A => {
                        debug_opcodes(&opcode, "Fx0A", "unimplemented");
                        let mut is_key_pressed = false;

                        for i in 0..self.bus.get_keypad().len() {
                            if self.bus.is_key_pressed(i) {
                                self.cpu.set_vreg_value(vx, i as u8);
                                is_key_pressed = true;
                                break;
                            }
                        }

                        // keeping looping until the key is pressed
                        if !is_key_pressed {
                            self.cpu.decrease_pc();
                        }
                    }

                    /// Set delay timer = Vx.
                    0x15 => {
                        debug_opcodes(&opcode, "Fx15", "Set delay timer = Vx.");

                        self.cpu.set_delay_timer(self.cpu.get_vreg_value(vx));
                    }

                    ///Set sound timer = Vx.
                    0x18 => {
                        debug_opcodes(&opcode, "Fx18", "Set sound timer = Vx.");

                        self.cpu.set_sound_timer(self.cpu.get_vreg_value(vx));
                    }

                    /// Set I = I + Vx.
                    0x1E => {
                        debug_opcodes(&opcode, "Fx1E", "Set I = I + Vx.");

                        self.cpu.set_i_reg_value(
                            self.cpu
                                .get_i_reg_value()
                                .wrapping_add(self.cpu.get_vreg_value(vx) as u16),
                        )
                    }

                    /// Set I = location of sprite for digit Vx.
                    0x29 => {
                        debug_opcodes(&opcode, "Fx29", "Set I = location of sprite for digit Vx");

                        let digit = self.cpu.get_vreg_value(vx);

                        self.cpu
                            .set_i_reg_value(crate::ram::FONTSET_START_ADDRESS + (5 * digit) as u16)
                    }

                    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    0x33 => {
                        debug_opcodes(
                            &opcode,
                            "Fx33",
                            "Store BCD representation of Vx in memory locations I, I+1, and I+2",
                        );

                        let value = self.cpu.get_vreg_value(vx);

                        // Extract hundreds, tens, and units digits
                        let hundreds = value / 100;
                        let tens = (value / 10) % 10;
                        let units = value % 10;

                        let i_reg = self.cpu.get_i_reg_value();

                        self.bus.ram_write_byte(i_reg, hundreds);
                        self.bus.ram_write_byte(i_reg + 1, tens);
                        self.bus.ram_write_byte(i_reg + 2, units);
                    }

                    /// Store registers V0 through Vx in memory starting at location I.
                    0x55 => {
                        debug_opcodes(
                            &opcode,
                            "Fx55",
                            "Store registers V0 through Vx in memory starting at location I.",
                        );

                        let i_reg = self.cpu.get_i_reg_value();

                        for i in 0..=vx {
                            self.bus
                                .ram_write_byte(i_reg + i as u16, self.cpu.get_vreg_value(i));
                        }
                    }

                    /// Read registers V0 through Vx from memory starting at location I.
                    0x65 => {
                        debug_opcodes(
                            &opcode,
                            "Fx65",
                            "Read registers V0 through Vx from memory starting at location I.",
                        );

                        let i_reg = self.cpu.get_i_reg_value();

                        for i in 0..=vx {
                            self.cpu
                                .set_vreg_value(i, self.bus.ram_read_byte(i_reg + i as u16))
                        }
                    }

                    _ => println!("unknown opcode"),
                }
            }

            _ => println!("Unknown opcode"),
        }
    }
}

fn gen_random_byte() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=255)
}

fn debug_opcodes(address: &u16, opcode: &str, desc: &str) {
    // println!(
    //     "Address: {}, Opcode: {}, Desc: {}",
    //     format!("{:X}", address),
    //     opcode,
    //     desc
    // )
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

    #[test]
    fn test_8xy5_1() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8375;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 10);
        chip8.cpu.set_vreg_value(vy, 8);

        let value = chip8
            .cpu
            .get_vreg_value(vx)
            .wrapping_sub(chip8.cpu.get_vreg_value(vy));

        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_vreg_value(vx), value);
        assert_eq!(chip8.cpu.get_vreg_value(0xF), 1, "0xF should be 1");
    }

    #[test]
    fn test_8xy5_0() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8375;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 8);
        chip8.cpu.set_vreg_value(vy, 10);
        let value = chip8
            .cpu
            .get_vreg_value(vx)
            .wrapping_sub(chip8.cpu.get_vreg_value(vy));

        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_vreg_value(vx), value);
        assert_eq!(chip8.cpu.get_vreg_value(0xF), 0, "0xF should be 0");
    }

    #[test]
    fn test_8xy6_1() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8126;
        let vx = ((opcode & 0x0F00) >> 8) as u8;

        chip8.cpu.set_vreg_value(vx, 1);

        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_vreg_value(0xF), 1, "Vf should be 1");
        assert_eq!(chip8.cpu.get_vreg_value(vx), 1 >> 1);
    }

    #[test]
    fn test_8xy6_0() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8126;
        let vx = ((opcode & 0x0F00) >> 8) as u8;

        chip8.cpu.set_vreg_value(vx, 4);
        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_vreg_value(vx), 4 >> 1);
        assert_eq!(chip8.cpu.get_vreg_value(0xF), 0, "Vf should be 0")
    }

    #[test]
    fn test_8xy7_1() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8827;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 4);
        chip8.cpu.set_vreg_value(vy, 5);

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(0xF), 1, "0xF should be 1");
        assert_eq!(chip8.cpu.get_vreg_value(vx), 5 - 4);
    }
    #[test]
    fn test_8xy7_0() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x8827;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 5);
        chip8.cpu.set_vreg_value(vy, 4);

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_vreg_value(0xF), 0, "0xF should be 0");
        assert_eq!(chip8.cpu.get_vreg_value(vx), 4_u8.wrapping_sub(5));
    }

    #[test]
    fn test_8xye_1() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x882E;
        let vx = ((opcode & 0x0F00) >> 8) as u8;

        chip8.cpu.set_vreg_value(vx, 128);
        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_vreg_value(0xF), 1, "0xF should be 1");
        assert_eq!(chip8.cpu.get_vreg_value(vx), 128 << 1);
    }

    #[test]
    fn test_8xye_0() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x872E;
        let vx = ((opcode & 0x0F00) >> 8) as u8;

        chip8.cpu.set_vreg_value(vx, 80);
        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_vreg_value(0xF), 0, "0xF should be 0");
        assert_eq!(chip8.cpu.get_vreg_value(vx), 80 << 1)
    }

    #[test]
    fn test_9xy0() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0x9B20;
        let vx = ((opcode & 0x0F00) >> 8) as u8;
        let vy = ((opcode & 0x00F0) >> 4) as u8;

        chip8.cpu.set_vreg_value(vx, 10);
        chip8.cpu.set_vreg_value(vy, 10);

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), 0x200, "Program counter should be zero");

        chip8.cpu.set_vreg_value(vy, 12);
        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_pc(), 0x200 + 2, "Program counter should be 2");
    }

    #[test]
    fn test_annn() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0xAB20;

        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_i_reg_value(), opcode & 0x0FFF);
    }

    #[test]
    fn test_bnnn() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0xB300;

        chip8.cpu.set_vreg_value(0, 10);
        let val = 10 + (opcode & 0x0FFF);
        chip8.exec_instructions(opcode);

        assert_eq!(chip8.cpu.get_pc(), val);
    }

    #[test]
    fn test_ex9e() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0xE22E;

        let vx = ((opcode & 0x0F00) >> 8) as u8;
        chip8.cpu.set_vreg_value(vx, 2);
        chip8.bus.handle_key_press(2, true);

        let pc = chip8.cpu.get_pc();
        chip8.exec_instructions(opcode);

        assert_eq!(
            chip8.bus.is_key_pressed(2),
            true,
            "key pressed must be true"
        );

        assert_eq!(
            chip8.cpu.get_pc(),
            pc + 2,
            "program counter must increase by 2"
        );

        // testing for false condition
        chip8.cpu.set_vreg_value(vx, 3);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), pc + 2, "PC must stay the same");
    }

    #[test]
    fn test_ex9a() {
        let mut chip8 = Chip8::init();
        let opcode: u16 = 0xE221;

        let vx = ((opcode & 0x0F00) >> 8) as u8;
        chip8.cpu.set_vreg_value(vx, 2);
        let pc = chip8.cpu.get_pc();
        chip8.exec_instructions(opcode);

        assert_eq!(
            chip8.bus.is_key_pressed(2),
            false,
            "key pressed must be true"
        );

        assert_eq!(
            chip8.cpu.get_pc(),
            pc + 2,
            "program counter must increase by 2"
        );

        chip8.bus.handle_key_press(2, true);
        chip8.exec_instructions(opcode);
        assert_eq!(chip8.cpu.get_pc(), pc + 2, "PC must stay the same");
    }
}
