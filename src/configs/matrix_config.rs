use crate::configs::{Step, Value};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MatrixConf {
    pub value: Value,

    pub min_rows: usize,
    pub min_columns: usize,
    pub step_rows: Step,

    pub max_rows: usize,
    pub max_columns: usize,
    pub step_columns: Step,
}

impl MatrixConf {
    pub fn new(
        value: Value,
        min_rows: usize,
        max_rows: usize,
        step_rows: Step,
        min_columns: usize,
        max_columns: usize,
        step_columns: Step,
    ) -> Self {
        MatrixConf {
            value,
            min_rows,
            max_rows,
            step_rows,
            min_columns,
            max_columns,
            step_columns,
        }
    }
}
