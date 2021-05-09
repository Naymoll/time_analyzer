use std::fmt::{Display, Formatter};
use std::fmt;
use crate::program::TimeStat;

#[derive(Copy, Clone)]
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
    pub fn curve(&self)  -> impl Fn(usize) -> f64 {
        match self {
            Complexity::OLogN => |s| (s as f64).log2(),
            Complexity::ON => |s| s as f64,
            Complexity::ONLogN => |s|  (s as f64) * (s as f64).log2(),
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
            Complexity::Unknown => "Unknown"
        };

        write!(f, "{}", complexity_str)
    }
}

pub struct LeastSq {
    pub coef: f64,
    pub complexity: Complexity,
    pub rms: f64,
}

pub fn minimal_least_sq<F>(times: &Vec<TimeStat>, fitting_curve: F) -> LeastSq
where F: Fn(usize) -> f64 {
    let mut _sigma_gn = 0.0;
    let mut sigma_gn_squared = 0.0;
    let mut sigma_time = 0.0;
    let mut sigma_time_gn = 0.0;

    for time in times {
        let gn_i = fitting_curve(time.args_len); //TODO: Curve
        _sigma_gn += gn_i;
        sigma_gn_squared += gn_i * gn_i;
        sigma_time += time.min.as_secs_f64();
        sigma_time_gn += time.min.as_secs_f64() * gn_i;
    }

    let coef = sigma_time_gn / sigma_gn_squared;
    let mut rms = 0.0;
    for time in times {
        let fit = coef * fitting_curve(time.args_len);
        rms += (time.min.as_secs_f64() - fit).powi(2);
    }

    let len = times.len() as f64;
    let mean = sigma_time / len;
    let result = LeastSq {
        coef,
        complexity: Complexity::Unknown,
        rms: (rms / len).sqrt() / mean,
    };

    result
}

pub fn computate_big_o(times: &Vec<TimeStat>) -> LeastSq {
    const FIT_CURVES: [Complexity; 5] = [
        Complexity::OLogN,
        Complexity::ON,
        Complexity::ONLogN,
        Complexity::ONSquared,
        Complexity::ONCubed,
    ];

    let mut best_fit = minimal_least_sq(times, Complexity::O1.curve());
    best_fit.complexity = Complexity::O1;

    for fit_curve in &FIT_CURVES {
        let current_fit = minimal_least_sq(times, fit_curve.curve());

        if current_fit.rms < best_fit.rms {
            best_fit = current_fit;
            best_fit.complexity = fit_curve.clone();
        }
    }

    best_fit
}
