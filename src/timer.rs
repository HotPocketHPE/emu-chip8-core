use std::time::{Duration, Instant};


pub struct Timer {
    curr_t: Duration,
    last_run: Instant,
    trigger_dur: Duration
}

impl Timer {
    pub fn new(dur: Duration) -> Self {
        Self { curr_t: Duration::ZERO, last_run: Instant::now(), trigger_dur: dur }
    }

    pub fn run<F>(&mut self, mut func_on_trigger: F) -> u8 where F: FnMut() {
        let dur = Instant::now().duration_since(self.last_run);
        self.last_run = Instant::now();
        let mut call_count = 0;
        self.curr_t += dur;
        while self.curr_t > self.trigger_dur {
            func_on_trigger();
            self.curr_t -= self.trigger_dur;
            call_count += 1;
        }
        return call_count;
    }

    pub fn run_once<F>(&mut self, mut func_on_trigger: F) -> bool where F: FnMut() {
        let dur = Instant::now().duration_since(self.last_run);
        self.last_run = Instant::now();
        self.curr_t += dur;
        if self.curr_t > self.trigger_dur {
            func_on_trigger();
            self.curr_t -= self.trigger_dur;
            return true;
        }
        return false;
    }

    pub fn run_once_check_cond<F>(&mut self, mut func_on_trigger: F) -> bool where F: FnMut() -> bool {
        let dur = Instant::now().duration_since(self.last_run);
        self.last_run = Instant::now();
        self.curr_t += dur;
        if self.curr_t > self.trigger_dur {
            let cond = func_on_trigger();
            self.curr_t -= self.trigger_dur;
            return cond;
        }
        return false;
    }

    pub fn pause(&mut self) {
        self.curr_t += Instant::now().duration_since(self.last_run);
    }

    pub fn resume(&mut self) {
        self.last_run = Instant::now();
    }
}