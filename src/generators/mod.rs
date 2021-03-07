use crate::configs::Step;
use std::fmt;
use std::fmt::{Display, Formatter};

pub mod array_generator;
pub mod matrix_generator;
pub mod number_generator;

pub trait ArgGen {
    fn generate(&self) -> String;
    fn generate_next(&mut self) -> String;
}

pub enum GenError {
    WrongConfig(String),
}

impl Display for GenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GenError::WrongConfig(s) => write!(f, "{}", s),
        }
    }
}

pub(in crate::generators) struct Row {
    min_len: usize,
    max_len: usize,
    cur_len: usize,
    step: Step,
}

impl Row {
    pub fn new(min_len: usize, max_len: usize, step: Step) -> Self {
        Row {
            min_len,
            max_len,
            cur_len: min_len,
            step,
        }
    }

    pub fn next_step(&mut self) {
        let next = match self.step {
            Step::None => self.cur_len,
            Step::Fixed(v) => self.cur_len + v,
            Step::Multiply(m) => self.cur_len * m as usize,
        };

        self.cur_len = next.clamp(self.min_len, self.max_len);
    }
}
