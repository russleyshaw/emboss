use std::time::{Duration, SystemTime};

pub struct TimerBuilder {
    size: Option<usize>,
    target_duration: Option<Duration>,
}

impl TimerBuilder {
    pub fn new() -> Self {
        Self {
            size: None,
            target_duration: None,
        }
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn fps(mut self, fps: f64) -> Self {
        self.target_duration = Some(Duration::from_secs_f64(1.0 / fps));
        self
    }

    pub fn build(self) -> Timer {
        Timer {
            data: vec![Duration::default(); 0],
            avg: Duration::default(),

            total: Duration::default(),
            size: self.size.unwrap(),
            frame: 0,

            last_trigger: SystemTime::now(),
            target_duration: self.target_duration.unwrap(),
        }
    }
}

pub struct Timer {
    data: Vec<Duration>,
    avg: Duration,

    size: usize,
    total: Duration,
    frame: usize,

    last_trigger: SystemTime,
    target_duration: Duration,
}

impl Timer {
    pub fn trigger(&mut self) {
        let elapsed = self.last_trigger.elapsed().unwrap();
        if elapsed < self.target_duration {
            std::thread::sleep(self.target_duration - elapsed);
        }

        let elapsed = self.last_trigger.elapsed().unwrap();

        // Recalc average
        self.data.push(elapsed);
        self.total += elapsed;
        if self.data.len() >= self.size {
            let removed = self.data.remove(0);
            self.total -= removed;
        }
        self.avg = self.total.div_f64(self.data.len() as f64);

        // Reset last trigger
        self.last_trigger = SystemTime::now();
        self.frame += 1;
    }

    pub fn fps(&self) -> f64 {
        1.0 / self.avg.as_secs_f64()
    }

    pub fn frame(&self) -> usize {
        self.frame
    }
}
