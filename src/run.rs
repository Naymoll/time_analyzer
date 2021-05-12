use core::default::Default;
use core::fmt::Debug;

#[derive(PartialOrd, PartialEq, Debug)]
pub struct Run {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub len: usize,
}

impl Run {
    pub fn update(&mut self, duration: f64) {
        self.min = self.min.min(duration);
        self.max = self.max.max(duration);
        self.avg += duration;
    }
}

impl Default for Run {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            avg: 0.0,
            len: 0,
        }
    }
}
