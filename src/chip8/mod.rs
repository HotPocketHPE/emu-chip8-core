use std::time::{Instant, Duration};

use self::{cpu::CPUState, display::DisplayData, memory::Memory};

mod cpu;
mod keyboard;
mod display;
mod instructions;
mod memory;

fn create_machine() {
    let state = CPUState::new(Memory::new_array(), DisplayData::new_64x32());
}

fn run_machine(state: &mut CPUState, clock_speed: f64) {
    let cpu_instr_dur = Duration::from_secs_f64(1f64 / clock_speed);
    let timer_regs_dur = Duration::from_secs_f64(1f64 / 60f64);
    let mut cpu_instr_t = Duration::ZERO;
    let mut timer_regs_t = Duration::ZERO;

    let mut start = Instant::now();
    let mut dur = Duration::ZERO;
    loop {
        cpu_instr_t += dur;
        timer_regs_t += dur;
        if cpu_instr_t > cpu_instr_dur {
            state.run_instr();
            cpu_instr_t -= cpu_instr_dur;
        }
        if timer_regs_t > timer_regs_dur {
            if state.dt > 1 { state.dt -= 1; }
            if state.st > 1 { state.st -= 1; }
            timer_regs_t -= timer_regs_dur;
        }
        dur = start.elapsed();
        start = Instant::now();
    }
}