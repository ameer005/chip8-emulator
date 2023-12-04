pub const EXECUTION_INDEX: u16 = 0x200;

pub struct CPU {
    v_regs: [u8; 16],
    i_reg: u16,
    delay_reg: u8,
    sound_reg: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
}

impl CPU {
    pub fn init() -> CPU {
        CPU {
            v_regs: [0; 16],
            i_reg: 0,
            delay_reg: 0,
            sound_reg: 0,
            program_counter: EXECUTION_INDEX,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    ///stacks
    pub fn get_stack_value(&self, stack_pointer: u8) -> u16 {
        self.stack[stack_pointer as usize]
    }

    pub fn set_stack_value(&mut self, stack_pointer: u8, value: u16) {
        self.stack[stack_pointer as usize] = value;
    }

    pub fn decrease_sp(&mut self) {
        if self.stack_pointer > 0 {
            self.stack_pointer -= 1;
        }
    }

    pub fn increase_sp(&mut self) {
        if self.stack_pointer < 255 {
            self.stack_pointer += 1;
        }
    }

    /// program counter
    pub fn update_pc(&mut self, val: u16) {
        self.program_counter = val;
    }

    // increment program counter by two
    pub fn increment_pc(&mut self) {
        self.program_counter += 2;
    }

    /// V registers
    pub fn set_vreg_value(&mut self, index: u8, val: u8) {
        self.v_regs[index as usize] = val;
    }

    pub fn get_vreg_value(&self, index: u8) -> u8 {
        self.v_regs[index as usize]
    }

    /// I register
    pub fn set_i_reg_value(&mut self, value: u16) {
        self.i_reg = value;
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_reg = value;
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_reg = value;
    }
}

/// Access to read only fields
impl CPU {
    pub fn get_sp(&self) -> u8 {
        self.stack_pointer
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn get_i_reg_value(&self) -> u16 {
        self.i_reg
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_reg
    }
}
