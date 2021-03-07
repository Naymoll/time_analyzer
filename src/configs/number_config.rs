use crate::configs::Value;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct NumberConf {
    pub value: Value,
}

impl NumberConf {
    pub fn new(value: Value) -> Self {
        NumberConf { value }
    }
}
