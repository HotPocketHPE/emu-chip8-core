
use std::time::Duration;

use crate::cli_debug::debug_state;
use crate::config::CHIP8_CONFIG;
use crate::cpu::CPUState;
use crate::display::DisplayData;
use crate::memory::Memory;
use crate::timer::Timer;

const NUM_SAVESTATES: usize = 8;
const UNINIT_SAVESTATE: Option<CPUState> = None;

pub struct Machine {
    cpu_state: CPUState,
    cpu_instr_timer: Timer,
    cpu_timer_regs_timer: Timer,
    vblank_timer: Timer,
    saved_states: [Option<CPUState>; NUM_SAVESTATES],
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
            saved_states: [UNINIT_SAVESTATE; NUM_SAVESTATES],
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
        self.vblank_timer.run(|| { 
            self.cpu_state.enter_vblank();
        });
    }

    fn run_until_instr(&mut self) {
        self.resume_timers();
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
            self.vblank_timer.run(|| { self.cpu_state.enter_vblank(); });
            if ran_instr {
                self.pause_timers();
                break;
            }
        }
    }

    pub fn resume_timers(&mut self) {
        self.cpu_instr_timer.resume();
        self.cpu_timer_regs_timer.resume();
        self.vblank_timer.resume();
    }

    pub fn pause_timers(&mut self) {
        self.cpu_instr_timer.pause();
        self.cpu_timer_regs_timer.pause();
        self.vblank_timer.pause();
    }

    pub fn run_step_debug(&mut self) -> String {
        self.run_until_instr();
        return debug_state(&self.cpu_state);
    }

    pub fn save_current_state(&mut self, index: usize) -> Result<(), String> {
        if index >= NUM_SAVESTATES {
            return Err(format!("Savestate index is too high! {}, max {}", index, NUM_SAVESTATES-1));
        }
        self.saved_states[index] = Some(self.cpu_state.clone());
        return Ok(());
    }

    pub fn load_state(&mut self, index: usize) -> Result<(), String> {
        let state_slot = self.saved_states.get(index)
            .ok_or(format!("Savestate index is too high! {}, max {}", index, NUM_SAVESTATES-1))?;
        let state = state_slot.as_ref().ok_or("No savestate in this slot")?;
        self.cpu_state = state.clone();
        return Ok(());
    }

    pub fn debug_cond(&self) -> bool {
        self.cpu_state.dt > 0
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
