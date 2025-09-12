use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
    last_frame: Instant,
    delta: Duration,
    accumulated: Duration,
    frame_count: u64,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start: now,
            last_frame: now,
            delta: Duration::ZERO,
            accumulated: Duration::ZERO,
            frame_count: 0,
        }
    }
    
    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_frame;
        self.last_frame = now;
        self.accumulated += self.delta;
        self.frame_count += 1;
    }
    
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
    
    pub fn elapsed_seconds(&self) -> f32 {
        self.accumulated.as_secs_f32()
    }
    
    pub fn fps(&self) -> f32 {
        if self.accumulated.as_secs_f32() > 0.0 {
            self.frame_count as f32 / self.accumulated.as_secs_f32()
        } else {
            0.0
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}