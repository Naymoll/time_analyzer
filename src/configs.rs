use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Value {
    Int { min: i64, max: i64 },
    Float { min: f64, max: f64 },
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Range {
    pub start: usize,
    pub end: usize,
    pub multiplier: usize,
}

impl Range {
    pub fn next(&mut self) {
        self.start = (self.start * self.multiplier).min(self.end);
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArrayConfig {
    value: Value,
    #[serde(flatten)]
    range: Range,
}

impl ArrayConfig {
    pub fn value(&self) -> Value {
        self.value
    }

    pub fn range(&self) -> Range {
        self.range
    }
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

#[cfg(test)]
mod tests {
    use crate::configs::{ArrayConfig, Range, Value};

    #[test]
    fn serde_test() {
        let config = ArrayConfig {
            value: Value::Int { min: 1, max: 2 },
            range: Range {
                start: 3,
                end: 4,
                multiplier: 5,
            },
        };

        let json = serde_json::to_string(&config).unwrap();
        println!("{}", json);
    }
}
