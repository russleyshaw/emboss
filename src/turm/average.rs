use std::time::Duration;

pub struct Average {
    pub data: Vec<Duration>,
    pub avg: Duration,

    size: usize,
    total: Duration,
}

impl Average {
    pub fn new(size: usize) -> Average {
        return Average {
            data: vec![Duration::default(); 0],
            avg: Duration::default(),

            total: Duration::default(),
            size,
        };
    }

    pub fn push(&mut self, val: Duration) {
        self.data.push(val);
        self.total += val;
        if self.data.len() >= self.size {
            let removed = self.data.remove(0);
            self.total -= removed;
        }

        self.avg = self.total.div_f64(self.data.len() as f64);
    }
}
