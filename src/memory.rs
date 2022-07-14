const MEMSIZE: usize = 0x1000;
pub const PROG_START_ADDR: usize = 0x200;
pub const STACK_START_ADDR: usize = 0x000;
const STACK_SIZE: usize = 0x10 * 2;
const FONT_START_ADDR: usize = STACK_START_ADDR + STACK_SIZE;

const FONT_LETTER_SIZE: usize = 5;
const FONT_DATA: [u8; FONT_LETTER_SIZE*16] = [
    //0
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    //1
    0x20, 0x60, 0x20, 0x20, 0x70,
    //2
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    //3
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    //4
    0x90, 0x90, 0xF0, 0x10, 0x10,
    //5
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    //6
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    //7
    0xF0, 0x10, 0x20, 0x40, 0x40,
    //8
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    //9
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    //A
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    //B
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    //C
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    //D
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    //E
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    //F
    0xF0, 0x80, 0xF0, 0x80, 0x80
];

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

    pub fn slice(&self) -> &[u8] {
        &self.mem
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
        self.mem[FONT_START_ADDR..FONT_START_ADDR+FONT_DATA.len()].copy_from_slice(&FONT_DATA);
    }

    pub fn get_font_addr(num: u8) -> u16 {
        (FONT_START_ADDR + (num as usize * FONT_LETTER_SIZE)).try_into().unwrap()
    }
}

