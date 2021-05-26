//! Описание типов аргументов, возможных значений и их генерации.

pub mod array_config;
pub mod matrix_config;
pub mod range_config;

#[doc(inline)]
pub use crate::configs::array_config::ArrayConfig;
#[doc(inline)]
pub use crate::configs::matrix_config::MatrixConfig;
#[doc(inline)]
pub use crate::configs::range_config::RangeConfig;

use rand::distributions::Distribution;
use rand::Rng;
use serde::Deserialize;

use validator::{Validate, ValidationError, ValidationErrors};

///Типаж генератора аргументов.
pub trait ArgumentGenerator {
    ///Возвращает длину аргументов.
    fn len(&self) -> usize;
    ///Увеличивает длину аргументов, после чего возвращается её.
    fn next_len(&mut self) -> usize;
    ///Генерирует новые значения.
    fn generate(&self) -> String;
}

//Костыль. Нельзя просто так сделать десериализацию в Vec<dyn ArgumentGenerator>
//т.к это нарушение trait object safety.
//Возможно, есть другой способ, но я - ¯\_(ツ)_/¯
/// Варианты генерируемых аргументов.
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

/// Варианты содержимого [`ArrayConfig`] и [`MatrixConfig`].
#[derive(Deserialize, Copy, Clone)]
#[serde(tag = "type")]
pub enum Value {
    /// Целое число.
    Int {
        /// Минимальное значение.
        #[serde(default = "Value::int_min")]
        min: i64,
        /// Максимальное занчение. `max` >= `min`.
        #[serde(default = "Value::int_max")]
        max: i64,
    },
    /// Вещественое число.
    Float {
        /// Минимальное значение.
        #[serde(default = "Value::float_min")]
        min: f64,
        /// Максимальное занчение. `max` >= `min`.
        #[serde(default = "Value::float_max")]
        max: f64,
    },
    /// Символ: a-z, A-Z, 0-9.
    Char,
    /// Булево значение: 0, 1.
    Bool,
}

impl Validate for Value {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Value::Int { min, max } => {
                if min > max {
                    let mut errors = ValidationErrors::new();
                    let mut error = ValidationError::new("min > max");
                    error.add_param("min".into(), min);
                    errors.add("Value::Int", error);

                    Err(errors)
                } else {
                    Ok(())
                }
            }
            Value::Float { min, max } => {
                if min > max {
                    let mut errors = ValidationErrors::new();
                    let mut error = ValidationError::new("min > max");
                    error.add_param("min".into(), min);
                    errors.add("Value::Float", error);

                    Err(errors)
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

//Функции для констат. Serde не хочет принимать константы в качестве значений по умолчанию
// ¯\_(ツ)_/¯
impl Value {
    /// Возвращает значение `Int::min` по умолчанию - [`i64::MIN`]. Используется [`serde`].
    const fn int_min() -> i64 {
        i64::MIN
    }
    /// Возвращает значение `Int::max` по умолчанию - [`i64::MAX`]. Используется [`serde`].
    const fn int_max() -> i64 {
        i64::MAX
    }
    /// Возвращает значение `Float::min` по умолчанию - [`f64::MIN`]. Используется [`serde`].
    const fn float_min() -> f64 {
        f64::MIN
    }
    /// Возвращает значение `Float::max` по умолчанию - [`f64::MAX`]. Используется [`serde`].
    const fn float_max() -> f64 {
        f64::MAX
    }
}

/// Диапазон натуральных значений.
#[derive(Deserialize, Validate, Copy, Clone)]
pub struct Range {
    /// Начало диапазона. Минимальное значение 1.
    #[serde(default = "Range::start_default")]
    #[validate(range(min = 1))]
    pub start: usize,
    /// Конец диапазона.
    #[serde(default = "Range::end_default")]
    pub end: usize,
    /// Множитель `start`. Минимальное значение 2.
    #[serde(default = "Range::multiplier_default")]
    #[validate(range(min = 2))]
    pub multiplier: usize,
}

impl Range {
    /// Умножает `start` на `multiplier`.
    /// Если `start` >= `end`, то возвращается `end`, иначе новое значение.
    pub fn next(&mut self) -> usize {
        self.start = (self.start * self.multiplier).min(self.end);
        self.start
    }

    /// Возвращает значение `start` по умолчанию - 10. Используется [`serde`].
    const fn start_default() -> usize {
        10
    }

    /// Возвращает значение `end` по умолчанию - [`usize::MAX`]. Используется [`serde`].
    const fn end_default() -> usize {
        usize::MAX
    }

    /// Возвращает значение `multiplier` по умолчанию - 2. Используется [`serde`].
    const fn multiplier_default() -> usize {
        2
    }
}

/// Генерация массива длиной `len`, с распределением `distr`.
fn generate_array<T, D>(len: usize, distr: D) -> String
where
    T: ToString,
    D: Distribution<T>,
{
    let mut result = len.to_string();
    generate(&mut result, len, distr);

    result
}

/// Генерация матрицы `rows`*`columns`, с распределением `distr`.
fn generate_matrix<T, D>(rows: usize, columns: usize, distr: D) -> String
where
    T: ToString,
    D: Distribution<T>,
{
    let mut result = format!("{} {}", rows.to_string(), columns.to_string());
    generate(&mut result, rows * columns, distr);

    result
}

/// Генерация значений в строку `result`, длиной `len`, с распределением `distr`.
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
