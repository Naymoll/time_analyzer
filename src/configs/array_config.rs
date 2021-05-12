use crate::configs::{generate_array, ArgumentGenerator, Range, Value};
use rand::distributions::{Alphanumeric, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ArrayConfig {
    value: Value,
    #[serde(flatten)]
    range: Range,
}

impl Default for ArrayConfig {
    fn default() -> Self {
        ArrayConfig {
            value: Value::Int {
                min: i64::MIN,
                max: i64::MAX,
            },
            range: Range {
                start: 10,
                end: 2000,
                multiplier: 2,
            },
        }
    }
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
