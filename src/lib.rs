use std::time::Duration;

use cpu::CPUState;
use disassembler::disassemble_opcode;
use display::DisplayData;
use memory::{Memory, PROG_START_ADDR};

pub mod cpu;
pub mod display;
pub mod memory;
pub mod keyboard;
pub mod instructions;
mod disassembler;

pub struct Machine {
    cpu_state: CPUState,
    cpu_clock_freq: f64,
    cpu_t: Duration,
    timer_regs_t: Duration,
    
}

impl Machine {
    pub fn new(program: &[u8], clock_speed_hz: f64) -> Machine {
        Machine {
            cpu_state: CPUState::new(Memory::with_prog(program), DisplayData::new_64x32()),
            cpu_clock_freq: clock_speed_hz,
            cpu_t: Duration::ZERO, 
            timer_regs_t: Duration::ZERO }
    }

    pub fn run(&mut self, dur: Duration) {
        let cpu_instr_dur = Duration::from_secs_f64(1f64 / self.cpu_clock_freq);
        let timer_regs_dur = Duration::from_secs_f64(1f64 / 60f64);

        self.cpu_t += dur;
        self.timer_regs_t += dur;
        if self.cpu_t > cpu_instr_dur {
            self.cpu_state.run_instr();
            self.cpu_t -= cpu_instr_dur;
        }
        if self.timer_regs_t > timer_regs_dur {
            if self.cpu_state.dt > 0 { self.cpu_state.dt -= 1; }
            if self.cpu_state.st > 0 { self.cpu_state.st -= 1; }
            self.timer_regs_t -= timer_regs_dur;
        }
    }

    pub fn press_key(&mut self, key: u8) {
        self.cpu_state.kbstate.key[key as usize] = true;
        self.cpu_state.kbstate.just_pressed = Some(key);
    }

    pub fn release_key(&mut self, key: u8) {
        self.cpu_state.kbstate.key[key as usize] = false;
    }

    pub fn display_data(&self) -> &DisplayData {
        &self.cpu_state.disp
    }

    pub fn current_opcode(&self) -> u16 {
        self.cpu_state.get_opcode()
    }
}

//TODO this should return string
pub fn disassemble_program(program: &[u8]) {
    disassemble_program_at(program, 0)
}

pub fn disassemble_program_at(program: &[u8], start: usize) {
    let mut i = start;
    while i < program.len() {
        if i == program.len()-1 {
            println!("{:X} | Standalone byte {:X}", i+PROG_START_ADDR, program[i]);
            break;
        }
        let opcode = ((program[i] as u16) << 8) | (program[i+1] as u16);
        match disassemble_opcode(opcode) {
            Ok(s) => println!("{:X} | {:X} | {}", i+PROG_START_ADDR, opcode, s),
            Err(s) => { println!("{:X} | {}", i+PROG_START_ADDR, s); break; },
        }
        i += 2;
    }
}