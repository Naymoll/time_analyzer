use crate::configs;
use crate::configs::array_config;
use crate::generators::{ArgGen, GenError, Row};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Uniform;
use rand::Rng;
use std::convert::TryFrom;
use std::fmt::Debug;

pub struct ArrayGen<T> {
    min: T,
    max: T,
    row: Row,
}

impl TryFrom<array_config::ArrayConf> for ArrayGen<i32> {
    type Error = GenError;

    fn try_from(config: array_config::ArrayConf) -> Result<Self, Self::Error> {
        match config.value {
            configs::Value::Int { min, max } => Ok(ArrayGen {
                min,
                max,
                row: Row::new(config.min_len, config.max_len, config.step),
            }),
            _ => Err(GenError::WrongConfig(String::from(
                "Wrong configs value type",
            ))),
        }
    }
}

impl TryFrom<array_config::ArrayConf> for ArrayGen<f32> {
    type Error = GenError;

    fn try_from(config: array_config::ArrayConf) -> Result<Self, Self::Error> {
        match config.value {
            configs::Value::Float { min, max } => Ok(ArrayGen {
                min,
                max,
                row: Row::new(config.min_len, config.max_len, config.step),
            }),
            _ => Err(GenError::WrongConfig(String::from(
                "Wrong configs value type",
            ))),
        }
    }
}

impl<T> ArgGen for ArrayGen<T>
where
    T: Copy + Debug + SampleUniform,
{
    fn generate(&self) -> String {
        let rng = rand::thread_rng();
        let buf: Vec<T> = rng
            .sample_iter(Uniform::new_inclusive(self.min, self.max))
            .take(self.row.cur_len)
            .collect();

        format!("{:?}", buf)
    }

    fn generate_next(&mut self) -> String {
        self.row.next_step();
        self.generate()
    }
}
