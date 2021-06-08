//! Вывод асимптотической временной сложности в BigO нотации.

use crate::run::Run;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Варианты временных сложностей.
#[derive(Copy, Clone, Debug)]
pub enum Complexity {
    O1,
    OLogN,
    ON,
    ONLogN,
    ONSquared,
    ONCubed,
    Unknown,
}

impl Complexity {
    /// Возвращает функции соответсвующей ей сложности. O(N) -> N, O(N^2) -> N^2...
    pub fn curve(&self) -> impl Fn(usize) -> f64 {
        match self {
            Complexity::OLogN => |s| (s as f64).log2(),
            Complexity::ON => |s| s as f64,
            Complexity::ONLogN => |s| (s as f64) * (s as f64).log2(),
            Complexity::ONSquared => |s| (s as f64).powi(2),
            Complexity::ONCubed => |s| (s as f64).powi(3),
            _ => |_| 1.0,
        }
    }
}

impl Display for Complexity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let complexity_str = match self {
            Complexity::O1 => "O(1)",
            Complexity::OLogN => "O(logN)",
            Complexity::ON => "O(N)",
            Complexity::ONLogN => "O(NlogN)",
            Complexity::ONSquared => "O(N^2)",
            Complexity::ONCubed => "O(N^3)",
            Complexity::Unknown => "Unknown",
        };

        write!(f, "{}", complexity_str)
    }
}

pub struct LeastSquares {
    /// Коэффициент
    pub coef: f64,
    /// Сложность программы.
    pub complexity: Complexity,
    /// Ошибка.
    pub rms: f64,
}

impl LeastSquares {
    #[doc(hidden)]
    fn minimal_least_squares<F>(runs: &[Run], fitting_curve: F) -> Self
    where
        F: Fn(usize) -> f64,
    {
        let (sq_points, times, time_points) = runs.iter().fold((0.0, 0.0, 0.0), |acc, run| {
            let point = fitting_curve(run.len);

            let sq_points = acc.0 + point.powi(2);
            let times = acc.1 + run.min;
            let time_points = acc.2 + run.min * point;

            (sq_points, times, time_points)
        });

        let coef = time_points / sq_points;
        let rms = runs.iter().fold(0.0, |acc, run| {
            let fit = coef * fitting_curve(run.len);
            acc + (run.min - fit).powi(2)
        });

        let len = runs.len() as f64;
        LeastSquares {
            coef,
            complexity: Complexity::Unknown,
            rms: (rms / len).sqrt() / (times / len),
        }
    }

    /// Вычисляет временную сложность на основе времени выполнения программы методов наименьших квадратов.
    pub fn computate_big_o(times: &[Run]) -> Self {
        const COMPLEXITIES: [Complexity; 5] = [
            Complexity::OLogN,
            Complexity::ON,
            Complexity::ONLogN,
            Complexity::ONSquared,
            Complexity::ONCubed,
        ];

        let mut best_fit = Self::minimal_least_squares(times, Complexity::O1.curve());
        best_fit.complexity = Complexity::O1;

        for complexity in &COMPLEXITIES {
            let current_fit = Self::minimal_least_squares(times, complexity.curve());

            if current_fit.rms < best_fit.rms {
                best_fit = current_fit;
                best_fit.complexity = *complexity;
            }
        }

        best_fit
    }
}
