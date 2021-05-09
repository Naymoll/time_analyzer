use crate::configs::{generate_matrix_with_distr, ArgumentGenerator, Range, Value};
use rand::distributions::{Alphanumeric, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MatrixConfig {
    value: Value,
    rows: Range,
    columns: Range,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        MatrixConfig {
            value: Value::Int {
                min: i64::MIN,
                max: i64::MAX,
            },
            rows: Range {
                start: 10,
                end: 2000,
                multiplier: 2,
            },
            columns: Range {
                start: 10,
                end: 2000,
                multiplier: 2,
            },
        }
    }
}

impl ArgumentGenerator for MatrixConfig {
    fn len(&self) -> usize {
        self.rows.start * self.columns.start
    }

    fn next_len(&mut self) -> usize {
        self.rows.next();
        self.columns.next();

        self.rows.start * self.columns.start
    }

    fn generate(&self) -> String {
        match self.value {
            Value::Int { min, max } => generate_matrix_with_distr(
                self.rows.start,
                self.columns.start,
                Uniform::new_inclusive(min, max),
            ),
            Value::Float { min, max } => generate_matrix_with_distr(
                self.rows.start,
                self.columns.start,
                Uniform::new_inclusive(min, max),
            ),
            Value::Char => {
                generate_matrix_with_distr(self.rows.start, self.columns.start, Alphanumeric)
            }
            //TODO: Возможно стоит заменить
            Value::Bool => generate_matrix_with_distr(
                self.rows.start,
                self.columns.start,
                Uniform::new_inclusive(0, 1),
            ),
        }
    }
}
