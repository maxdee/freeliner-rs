use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start: Instant::now(),
        }
    }
}

impl Timer {
    pub fn get(&self) -> Duration {
        self.start.elapsed()
    }
    pub fn get_time(&self) -> u64 {
        // self.start.elapsed().as_secs() as f64
        // + (self.start.elapsed().subsec_nanos() as f64 * 1e-9)
        // (self.start.elapsed().subsec_nanos() as f64 * 1e-9)
        self.start.elapsed().as_secs() * 1000
            + u64::from(self.start.elapsed().subsec_nanos()) / 1_000_000
    }

    pub fn tap(&self) {}
}
