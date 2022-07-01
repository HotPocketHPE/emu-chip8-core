

pub struct CPUState {
    ///Program Counter
    pub pc: u16,
    ///Index Register
    pub i: u16,
    ///Registers
    pub v: [u8; 0x10],
    ///Stack Pointer
    pub sp: u8,
    ///Delay Timer
    pub dt: u8,
    ///Sound Timer
    pub st: u8
}

impl CPUState {
    pub fn write_mem(&mut self, addr: u16, val: u8) {
        todo!();
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        todo!();
    }

    pub fn get_opcode(&self) -> u16 {
        (self.read_mem(self.pc) as u16) << 8 & (self.read_mem(self.pc+1) as u16)
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
}

