use std::time::Instant;

pub struct TimeKeeper {
    start: Instant,
    th: f64,
}

impl TimeKeeper {
    pub fn new(th: f64) -> Self {
        Self {
            start: Instant::now(),
            th: th - 0.05,
        }
    }

    pub fn is_over(&self) -> bool {
        self.start.elapsed().as_secs_f64() >= self.th
    }

    pub fn progress(&self) -> f64 {
        self.start.elapsed().as_secs_f64() / self.th
    }
}
