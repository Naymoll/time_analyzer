use crate::configs;
use crate::configs::matrix_config;
use crate::generators::{ArgGen, GenError, Row};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Uniform;
use rand::Rng;
use std::convert::TryFrom;
use std::fmt::Debug;

pub struct MatrixGen<T> {
    min: T,
    max: T,
    rows: Row,
    columns: Row,
}

impl TryFrom<matrix_config::MatrixConf> for MatrixGen<i32> {
    type Error = GenError;

    fn try_from(config: matrix_config::MatrixConf) -> Result<Self, Self::Error> {
        match config.value {
            configs::Value::Int { min, max } => Ok(MatrixGen {
                min,
                max,
                rows: Row::new(config.min_rows, config.max_rows, config.step_rows),
                columns: Row::new(config.min_columns, config.max_columns, config.step_columns),
            }),
            _ => Err(GenError::WrongConfig(String::from(
                "Wrong configs value type",
            ))),
        }
    }
}

impl TryFrom<matrix_config::MatrixConf> for MatrixGen<f32> {
    type Error = GenError;

    fn try_from(config: matrix_config::MatrixConf) -> Result<Self, Self::Error> {
        match config.value {
            configs::Value::Float { min, max } => Ok(MatrixGen {
                min,
                max,
                rows: Row::new(config.min_rows, config.max_rows, config.step_rows),
                columns: Row::new(config.min_columns, config.max_columns, config.step_columns),
            }),
            _ => Err(GenError::WrongConfig(String::from(
                "Wrong configs value type",
            ))),
        }
    }
}

impl<T> MatrixGen<T> {
    fn next_step(&mut self) {
        self.rows.next_step();
        self.columns.next_step();
    }
}

impl<T> ArgGen for MatrixGen<T>
where
    T: Copy + Debug + SampleUniform,
{
    fn generate(&self) -> String {
        let rng = rand::thread_rng();
        let buf: Vec<T> = rng
            .sample_iter(Uniform::new_inclusive(self.min, self.max))
            .take(self.rows.cur_len * self.columns.cur_len)
            .collect();

        format!("{:?}", buf)
    }

    fn generate_next(&mut self) -> String {
        self.next_step();
        self.generate()
    }
}
