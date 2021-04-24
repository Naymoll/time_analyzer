use crate::configs::{ArrayConfig, MatrixConfig, Range, Value};

use rand::distributions::uniform::SampleUniform;
use rand::distributions::Uniform;
use rand::Rng;
use std::convert::TryFrom;
use std::fmt::Debug;

pub trait ArgumentGenerator {
    fn len(&self) -> usize;
    fn next_len(&mut self) -> usize;
    fn generate(&self) -> String;
}

pub struct ArrayGenerator<T> {
    min: T,
    max: T,
    range: Range,
}

impl TryFrom<ArrayConfig> for ArrayGenerator<i64> {
    type Error = ();

    fn try_from(config: ArrayConfig) -> Result<Self, Self::Error> {
        match config.value() {
            Value::Int { min, max } => Ok(ArrayGenerator {
                min,
                max,
                range: config.range(),
            }),
            _ => Err(()),
        }
    }
}

impl TryFrom<ArrayConfig> for ArrayGenerator<f64> {
    type Error = ();

    fn try_from(config: ArrayConfig) -> Result<Self, Self::Error> {
        match config.value() {
            Value::Float { min, max } => Ok(ArrayGenerator {
                min,
                max,
                range: config.range(),
            }),
            _ => Err(()),
        }
    }
}

impl<T> ArgumentGenerator for ArrayGenerator<T>
where
    T: Copy + Debug + SampleUniform + ToString,
{
    fn len(&self) -> usize {
        self.range.start
    }

    fn next_len(&mut self) -> usize {
        self.range.next();
        self.len()
    }

    fn generate(&self) -> String {
        let rng = rand::thread_rng();
        let buf: Vec<String> = rng
            .sample_iter(Uniform::new_inclusive(self.min, self.max))
            .take(self.len())
            .map(|v| v.to_string())
            .collect();

        //format!("{:?}", buf)
        buf.join(" ")
    }
}

pub struct MatrixGenerator<T> {
    min: T,
    max: T,
    rows: Range,
    columns: Range,
}

impl TryFrom<MatrixConfig> for MatrixGenerator<i64> {
    type Error = ();

    fn try_from(config: MatrixConfig) -> Result<Self, Self::Error> {
        match config.value() {
            Value::Int { min, max } => Ok(MatrixGenerator {
                min,
                max,
                rows: config.rows(),
                columns: config.columns(),
            }),
            _ => Err(()),
        }
    }
}

impl TryFrom<MatrixConfig> for MatrixGenerator<f64> {
    type Error = ();

    fn try_from(config: MatrixConfig) -> Result<Self, Self::Error> {
        match config.value() {
            Value::Float { min, max } => Ok(MatrixGenerator {
                min,
                max,
                rows: config.rows(),
                columns: config.columns(),
            }),
            _ => Err(()),
        }
    }
}

impl<T> ArgumentGenerator for MatrixGenerator<T>
where
    T: Copy + Debug + SampleUniform + ToString,
{
    fn len(&self) -> usize {
        self.rows.start * self.columns.start
    }

    fn next_len(&mut self) -> usize {
        self.rows.next();
        self.columns.next();
        self.len()
    }

    fn generate(&self) -> String {
        let rng = rand::thread_rng();
        let buf: Vec<String> = rng
            .sample_iter(Uniform::new_inclusive(self.min, self.max))
            .take(self.len())
            .map(|v| v.to_string())
            .collect();

        //format!("{:?}", buf)
        buf.join(" ")
    }
}
