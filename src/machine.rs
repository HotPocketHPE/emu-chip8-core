
use std::time::Duration;

use crate::cli_debug::debug_state;
use crate::config::CHIP8_CONFIG;
use crate::cpu::CPUState;
use crate::display::DisplayData;
use crate::memory::Memory;
use crate::timer::Timer;

pub struct Machine {
    cpu_state: CPUState,
    cpu_instr_timer: Timer,
    cpu_timer_regs_timer: Timer,
    vblank_timer: Timer,
}

impl Machine {
    pub fn new(program: &[u8]) -> Machine {
        let cpu_state = CPUState::new(Memory::with_prog(program), DisplayData::new_64x32());
        let cpu_clock_freq = CHIP8_CONFIG.clock_speed_hz;
        Machine {
            cpu_state,
            cpu_instr_timer: Timer::new(Duration::from_secs_f64(1.0 / cpu_clock_freq as f64)),
            cpu_timer_regs_timer: Timer::new(Duration::from_secs_f64(1.0 / 60.0)),
            vblank_timer: Timer::new(Duration::from_secs_f64(1.0 / 60.0)),
        }
    }

    pub fn run(&mut self) {
        self.cpu_instr_timer.run(|| { self.cpu_state.run_cycle(); });
        self.cpu_timer_regs_timer.run(|| {
            if self.cpu_state.dt > 0 {
                self.cpu_state.dt -= 1;
            }
            if self.cpu_state.st > 0 {
                self.cpu_state.st -= 1;
            }
        });
        self.vblank_timer.run(|| { self.cpu_state.disp.enter_vblank(); });
    }

    fn run_until_instr(&mut self) {
        self.cpu_instr_timer.resume();
        self.cpu_timer_regs_timer.resume();
        self.vblank_timer.resume();
        loop {
            let ran_instr = self.cpu_instr_timer.run_once_check_cond(|| { self.cpu_state.run_cycle() });
            self.cpu_timer_regs_timer.run(|| {
                if self.cpu_state.dt > 0 {
                    self.cpu_state.dt -= 1;
                }
                if self.cpu_state.st > 0 {
                    self.cpu_state.st -= 1;
                }
            });
            self.vblank_timer.run(|| { self.cpu_state.disp.enter_vblank(); });
            if ran_instr {
                self.cpu_instr_timer.pause();
                self.cpu_timer_regs_timer.pause();
                self.vblank_timer.pause();
                break;
            }
        }
    }

    pub fn run_step_debug(&mut self) -> String {
        self.run_until_instr();
        return debug_state(&self.cpu_state);
    }

    pub fn press_key(&mut self, key: u8) {
        self.cpu_state.kbstate.press_key(key);
    }

    pub fn release_key(&mut self, key: u8) {
        self.cpu_state.kbstate.release_key(key);
    }

    pub fn display_data(&self) -> &DisplayData {
        &self.cpu_state.disp
    }

    pub fn should_make_sound(&self) -> bool {
        self.cpu_state.st > 1
    }

    pub fn current_opcode(&self) -> u16 {
        self.cpu_state.get_opcode()
    }
}
