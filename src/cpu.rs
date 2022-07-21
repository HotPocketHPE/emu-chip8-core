use crate::{
    keyboard::KeyboardState,
    memory::{PROG_START_ADDR, STACK_START_ADDR}, instructions::{DRW, Fx0AHandler},
};

use super::{display::DisplayData, instructions::OUTER_FUNC_TABLE, memory::Memory};

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

    pub halt_status: HaltStatus
}

pub enum HaltStatus {
    NotHalted,
    WaitingVblank,
    WaitingFx0A,
    ExecutingDRW
}

impl CPUState {
    pub fn new(mem: Memory, disp: DisplayData) -> CPUState {
        CPUState {
            pc: PROG_START_ADDR as u16,
            i: 0,
            v: [0; 0x10],
            sp: STACK_START_ADDR as u8,
            dt: 0,
            st: 0,
            mem,
            disp,
            kbstate: KeyboardState::new(),
            halt_status: HaltStatus::NotHalted
        }
    }

    pub fn debug_state(&self) -> String {
        let mut s = String::new();
        for i in 0..self.v.len() {
            s += &format!("V{:X}: {:X} ", i, self.v[i]);
        }
        s += &format!(
            "\nPC: {:X} I: {:X} SP: {:X} DT: {:X} ST: {:X}",
            self.pc, self.i, self.sp, self.dt, self.st
        );
        return s;
    }

    pub fn get_opcode(&self) -> u16 {
        u16::from_be_bytes([self.mem.read(self.pc), self.mem.read(self.pc + 1)])
    }

    pub fn d_addr(&self) -> u16 {
        nnn(self.get_opcode())
    }

    pub fn d_n(&self) -> u8 {
        n(self.get_opcode())
    }

    pub fn d_x(&self) -> usize {
        x(self.get_opcode()) as usize
    }

    pub fn d_y(&self) -> usize {
        y(self.get_opcode()) as usize
    }

    pub fn d_kk(&self) -> u8 {
        kk(self.get_opcode())
    }

    pub fn enter_vblank(&mut self) {
        if let HaltStatus::WaitingVblank = self.halt_status {
            self.halt_status = HaltStatus::ExecutingDRW;
        }
    }

    pub fn run_cycle(&mut self) -> bool {
        match self.halt_status {
            HaltStatus::WaitingVblank => {
                return false;
            },
            HaltStatus::ExecutingDRW => {
                DRW(self);
                self.halt_status = HaltStatus::NotHalted;
                return true;
            },
            HaltStatus::WaitingFx0A => {
                let finished = Fx0AHandler(self);
                if finished {
                    self.halt_status = HaltStatus::NotHalted;
                }
                return finished;
            },
            HaltStatus::NotHalted => {
                let highest_nibble = (self.get_opcode() & 0xF000) >> 12;
                OUTER_FUNC_TABLE[highest_nibble as usize](self);
                return matches!(self.halt_status, HaltStatus::NotHalted);
            },
        }
    }
}

pub fn nnn(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

pub fn x(opcode: u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

pub fn y(opcode: u16) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

pub fn n(opcode: u16) -> u8 {
    (opcode & 0x000F) as u8
}

pub fn kk(opcode: u16) -> u8 {
    (opcode & 0x00FF) as u8
}
