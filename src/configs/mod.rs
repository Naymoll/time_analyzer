pub mod array_config;
pub mod matrix_config;
pub mod range_config;

use crate::configs::array_config::ArrayConfig;
use crate::configs::matrix_config::MatrixConfig;
use crate::configs::range_config::RangeConfig;

use rand::distributions::Distribution;
use rand::Rng;
use serde::Deserialize;

use validator::{Validate, ValidationErrors};

pub trait ArgumentGenerator {
    fn len(&self) -> usize;
    fn next_len(&mut self) -> usize;
    fn generate(&self) -> String;
}

//Костыль. Нельзя просто так сделать десериализацию в Vec<dyn ArgumentGenerator>
//т.к это нарушение trait object safety.
//Возможно, есть другой способ, но я - ¯\_(ツ)_/¯
#[derive(Deserialize)]
pub enum Config {
    Array(ArrayConfig),
    Matrix(MatrixConfig),
    Range(RangeConfig),
}

impl Validate for Config {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Config::Array(array) => array.validate(),
            Config::Matrix(matrix) => matrix.validate(),
            Config::Range(range) => range.validate(),
        }
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(tag = "type")]
pub enum Value {
    Int {
        #[serde(default = "Value::int_min")]
        min: i64,
        #[serde(default = "Value::int_max")]
        max: i64,
    },
    Float {
        #[serde(default = "Value::float_min")]
        min: f64,
        #[serde(default = "Value::float_max")]
        max: f64,
    },
    Char, //Генерируемые значения: A-Z | a-z | 0-9
    Bool,
}

//Функции для констат. Serde не хочет принимать константы в качестве значений по умолчанию
// ¯\_(ツ)_/¯
impl Value {
    const fn int_min() -> i64 {
        i64::MIN
    }

    const fn int_max() -> i64 {
        i64::MAX
    }

    const fn float_min() -> f64 {
        f64::MIN
    }

    const fn float_max() -> f64 {
        f64::MAX
    }
}

#[derive(Deserialize, Validate, Copy, Clone)]
pub struct Range {
    #[serde(default = "Range::start_default")]
    #[validate(range(min = 1))]
    pub start: usize,
    #[serde(default = "Range::end_default")]
    pub end: usize,
    #[serde(default = "Range::multiplier_default")]
    #[validate(range(min = 2))]
    pub multiplier: usize,
}

impl Range {
    pub fn next(&mut self) -> usize {
        self.start = (self.start * self.multiplier).min(self.end);
        self.start
    }

    const fn start_default() -> usize {
        10
    }

    const fn end_default() -> usize {
        usize::MAX
    }

    const fn multiplier_default() -> usize {
        2
    }
}

fn generate_array<T, D>(len: usize, distr: D) -> String
where
    T: ToString,
    D: Distribution<T>,
{
    let mut result = len.to_string();
    generate(&mut result, len, distr);

    result
}

fn generate_matrix<T, D>(rows: usize, columns: usize, distr: D) -> String
where
    T: ToString,
    D: Distribution<T>,
{
    let mut result = format!("{} {}", rows.to_string(), columns.to_string());
    generate(&mut result, rows * columns, distr);

    result
}

fn generate<T, D>(result: &mut String, len: usize, distr: D)
where
    T: ToString,
    D: Distribution<T>,
{
    let rng = rand::thread_rng();
    let rng_iter = rng.sample_iter(distr).take(len).map(|v| v.to_string());

    for val in rng_iter {
        result.push(' ');
        result.push_str(&val);
    }
}

#[cfg(test)]
mod tests {
    use crate::configs::Config;

    #[test]
    fn deserialization_test() {
        let json = r#"[{"Range":{"start":10,"end":1000,"multiplier":2}},{"Array":{"value":{"type":"Int","min":0,"max":100},"start":10,"end":1000,"multiplier":2}}]"#;
        let _from_json: Vec<Config> = serde_json::from_str(&json).unwrap();
    }
}
