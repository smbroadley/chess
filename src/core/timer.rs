use std::{fmt::Display, time::Instant};

pub struct CountdownTimer {
    remaining: usize,
    start: Option<Instant>,
}

impl CountdownTimer {
    pub fn new(time: std::time::Duration) -> Self {
        Self {
            remaining: time.as_millis() as usize,
            start: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        let elapsed = self.start.unwrap().elapsed();
        self.remaining = self.remaining.saturating_sub(elapsed.as_millis() as usize);
        self.start = None;
    }

    pub fn remaining(&self) -> usize {
        if let Some(timer) = self.start {
            self.remaining - (timer.elapsed().as_millis() as usize)
        } else {
            self.remaining
        }
    }
}

impl Display for CountdownTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.remaining() / 1000; // in seconds

        let m = r / 60;
        let s = r % 60;

        write!(f, "{:2}:{:02}", m, s)
    }
}
