use crate::display::{FONT_DATA, FONT_LETTER_SIZE};

const MEMSIZE: usize = 0x1000;
pub const PROG_START_ADDR: usize = 0x200;
//const STACK_START_ADDR: usize = 0x000;
const FONT_START_ADDR: usize = 0x020;

#[derive(Debug)]
pub struct Memory {
    mem: [u8; MEMSIZE]
}

impl Memory {
    pub fn with_prog(program: &[u8]) -> Memory {
        let mut mem = Memory {
            mem: [0; MEMSIZE],
        };
        mem.load_fonts();
        mem.load_program_default(program);
        return mem;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val
    }

    fn load_program(&mut self, program: &[u8], start: usize) {
        if program.len() > MEMSIZE - start {
            panic!("Program is too big to fit in memory! Size: {} Space: {} ({} - {})",
                program.len(), MEMSIZE-start, MEMSIZE, start);
        }
        self.mem[start..start+program.len()].copy_from_slice(program);
    }

    fn load_program_default(&mut self, program: &[u8]) {
        self.load_program(program, PROG_START_ADDR);
    }

    fn load_fonts(&mut self) {
        self.mem[FONT_START_ADDR..FONT_DATA.len()+FONT_START_ADDR].copy_from_slice(&FONT_DATA);
    }

    pub fn get_font_addr(num: u8) -> u16 {
        (FONT_START_ADDR + (num as usize * FONT_LETTER_SIZE)).try_into().unwrap()
    }
}

