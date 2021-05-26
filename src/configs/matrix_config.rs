//! Описание аргумента матрицы.

use crate::configs::{generate_matrix, ArgumentGenerator, Range, Value};
use rand::distributions::{Alphanumeric, Uniform};
use serde::Deserialize;
use validator::Validate;

/// Тип аргумента матрица.
#[derive(Deserialize, Validate)]
pub struct MatrixConfig {
    /// Тип содержимого матрицы.
    #[validate]
    value: Value,
    /// Размер строк.
    #[validate]
    rows: Range,
    /// Размер столбцов.
    #[validate]
    columns: Range,
}

impl ArgumentGenerator for MatrixConfig {
    fn len(&self) -> usize {
        self.rows.start * self.columns.start
    }

    fn next_len(&mut self) -> usize {
        self.rows.next() * self.columns.next()
    }

    fn generate(&self) -> String {
        match self.value {
            Value::Int { min, max } => generate_matrix(
                self.rows.start,
                self.columns.start,
                Uniform::new_inclusive(min, max),
            ),
            Value::Float { min, max } => generate_matrix(
                self.rows.start,
                self.columns.start,
                Uniform::new_inclusive(min, max),
            ),
            Value::Char => generate_matrix(self.rows.start, self.columns.start, Alphanumeric),
            //TODO: Возможно стоит заменить
            Value::Bool => generate_matrix(
                self.rows.start,
                self.columns.start,
                Uniform::new_inclusive(0, 1),
            ),
        }
    }
}
