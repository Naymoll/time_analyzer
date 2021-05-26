use crate::configs::{generate_array, ArgumentGenerator, Range, Value};
use rand::distributions::{Alphanumeric, Uniform};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ArrayConfig {
    value: Value,
    #[serde(flatten)]
    #[validate]
    range: Range,
}

impl ArgumentGenerator for ArrayConfig {
    fn len(&self) -> usize {
        self.range.start
    }

    fn next_len(&mut self) -> usize {
        self.range.next()
    }

    fn generate(&self) -> String {
        match self.value {
            Value::Int { min, max } => generate_array(self.len(), Uniform::new_inclusive(min, max)),
            Value::Float { min, max } => {
                generate_array(self.len(), Uniform::new_inclusive(min, max))
            }
            Value::Char => generate_array(self.len(), Alphanumeric),
            //TODO: Возможно стоит заменить
            Value::Bool => generate_array(self.len(), Uniform::new_inclusive(0, 1)),
        }
    }
}
