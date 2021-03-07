use crate::configs;
use crate::configs::number_config;
use crate::generators::{ArgGen, GenError};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Uniform;
use rand::Rng;
use std::convert::TryFrom;
use std::fmt::Display;

pub struct NumberGen<T> {
    min: T,
    max: T,
}

impl TryFrom<number_config::NumberConf> for NumberGen<i32> {
    type Error = GenError;

    fn try_from(config: number_config::NumberConf) -> Result<Self, Self::Error> {
        match config.value {
            configs::Value::Int { min, max } => Ok(NumberGen { min, max }),
            _ => Err(GenError::WrongConfig(String::from(
                "Wrong configs value type",
            ))),
        }
    }
}

impl TryFrom<number_config::NumberConf> for NumberGen<f32> {
    type Error = GenError;

    fn try_from(config: number_config::NumberConf) -> Result<Self, Self::Error> {
        match config.value {
            configs::Value::Float { min, max } => Ok(NumberGen { min, max }),
            _ => Err(GenError::WrongConfig(String::from(
                "Wrong configs value type",
            ))),
        }
    }
}

impl<T> ArgGen for NumberGen<T>
where
    T: Copy + Display + SampleUniform,
{
    fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        rng.sample(Uniform::new_inclusive(self.min, self.max))
            .to_string()
    }

    fn generate_next(&mut self) -> String {
        self.generate()
    }
}
