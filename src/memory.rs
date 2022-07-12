const MEMSIZE: usize = 0x1000;

pub struct Memory {
    mem: [u8; MEMSIZE]
}

impl Memory {
    pub fn new_array() -> Memory {
        Memory {
            mem: [0; MEMSIZE],
        }
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
        self.mem[start..].copy_from_slice(program);
    }

    fn load_program_default(&mut self, program: &[u8]) {
        self.load_program(program, 0x200);
    }
}

