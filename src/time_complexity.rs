use std::iter::Sum;
use std::ops::{Add, Div, Sub};

pub enum Complexity {
    Const,
    Log,
    LogN,
    N,
    NLogN,
    Quad,
    Cubic,
    Exp,
    Factorial,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stats {
    pub min: u128,
    pub max: u128,
    pub avg: u128,
}

impl Stats {
    pub fn new(min: u128, max: u128, avg: u128) -> Self {
        Stats { min, max, avg }
    }
}

impl<'a> Sum<&'a Stats> for Stats {
    fn sum<I: Iterator<Item = &'a Stats>>(iter: I) -> Self {
        iter.fold(Self::new(0, 0, 0), |acc, s| acc + s)
    }
}

impl<'a> Add<&'a Stats> for Stats {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {
        Stats {
            max: self.max + rhs.max,
            min: self.min + rhs.min,
            avg: self.avg + rhs.avg,
        }
    }
}

impl<'a> Sub<&'a Stats> for &'a Stats {
    type Output = Stats;

    fn sub(self, rhs: Self) -> Self::Output {
        Stats {
            max: self.max - rhs.max,
            min: self.min - rhs.min,
            avg: self.avg - rhs.avg,
        }
    }
}

impl Div<u128> for Stats {
    type Output = Self;

    fn div(self, rhs: u128) -> Self::Output {
        Stats {
            max: self.max / rhs,
            min: self.min / rhs,
            avg: self.avg / rhs,
        }
    }
}
