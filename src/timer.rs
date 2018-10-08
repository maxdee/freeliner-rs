use std::time::{Duration, Instant};

pub struct Timer {
    start : Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start : Instant::now(),
        }
    }
    pub fn get(&self) -> Duration {
        self.start.elapsed()
    }
    pub fn get_time(&self) -> u64 {
        // self.start.elapsed().as_secs() as f64
        // + (self.start.elapsed().subsec_nanos() as f64 * 1e-9)
        // (self.start.elapsed().subsec_nanos() as f64 * 1e-9)
        self.start.elapsed().as_secs() * 1000 +
            self.start.elapsed().subsec_nanos() as u64 / 1_000_000
    }

    pub fn tap(&self){

    }
}
