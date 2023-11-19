pub const EXECUTION_INDEX: u16 = 0x200;

pub struct CPU {
    v_regs: [u8; 16],
    i_reg: u16,
    delay_reg: u8,
    sound_reg: u8,
    pc: u16,
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
            pc: EXECUTION_INDEX,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
}
