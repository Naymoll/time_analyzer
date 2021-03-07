use crate::configs::{Step, Value};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ArrayConf {
    pub value: Value,
    pub min_len: usize,
    pub max_len: usize,
    pub step: Step,
}

impl ArrayConf {
    pub fn new(value: Value, min_len: usize, max_len: usize, step: Step) -> Self {
        ArrayConf {
            value,
            min_len,
            max_len,
            step,
        }
    }
}
