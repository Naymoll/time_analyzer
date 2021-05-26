//! Статистика.

use core::default::Default;
use core::fmt::Debug;

/// Статистика времени выполнения программы в поколении.
#[derive(PartialOrd, PartialEq, Debug)]
pub struct Run {
    /// Минимльное время.
    pub min: f64,
    /// Максимальное время.
    pub max: f64,
    /// Среднее время.
    pub avg: f64,
    /// Длина аргументов.
    pub len: usize,
}

impl Run {
    /// Обновляет `self`, в зависимости от `duration`.
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

#[cfg(test)]
mod tests {
    use crate::run::Run;

    #[test]
    fn update_test() {
        let mut run = Run::default();
        run.update(10.0);

        assert_eq!(
            Run {
                min: 10.0,
                max: 10.0,
                avg: 10.0,
                len: 0,
            },
            run
        )
    }
}
