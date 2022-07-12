use crate::keyboard::KeyboardState;

use super::{instructions::OUTER_FUNC_TABLE, memory::Memory, display::DisplayData};

pub struct CPUState {
    pub pc: u16,
    pub i: u16,
    pub v: [u8; 0x10],
    pub sp: u8,
    pub dt: u8,
    pub st: u8,

    pub mem: Memory,
    pub disp: DisplayData,
    pub kbstate: KeyboardState,
}

impl CPUState {
    pub fn new(mem: Memory, disp: DisplayData) -> CPUState {
        CPUState { pc: 0x200, i: 0, v: [0; 0x10], sp: 0, dt: 0, st: 0, mem, disp, kbstate: KeyboardState::default() }
    }

    pub fn get_opcode(&self) -> u16 {
        (self.mem.read(self.pc) as u16) << 8 & (self.mem.read(self.pc+1) as u16)
    }

    pub fn d_addr(&self) -> u16 {
        self.get_opcode() & 0x0FFF
    }

    pub fn d_n(&self) -> u8 {
        (self.get_opcode() & 0x000F) as u8
    }

    pub fn d_x(&self) -> usize {
        ((self.get_opcode() & 0x0F00) >> 8) as usize
    }

    pub fn d_y(&self) -> usize {
        ((self.get_opcode() & 0x00F0) >> 4) as usize
    }

    pub fn d_kk(&self) -> u8 {
        (self.get_opcode() & 0x00FF) as u8
    }

    pub fn run_instr(&mut self) {
        let highest_nibble = (self.get_opcode() & 0xF000) >> 12;
        OUTER_FUNC_TABLE[highest_nibble as usize](self);
    }
    
}
