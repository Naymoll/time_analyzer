use crate::configs::{ArgumentGenerator, Range};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RangeConfig {
    #[serde(flatten)]
    range: Range,
}

impl ArgumentGenerator for RangeConfig {
    fn len(&self) -> usize {
        self.range.start
    }

    fn next_len(&mut self) -> usize {
        self.range.next();
        self.range.start
    }

    fn generate(&self) -> String {
        self.range.start.to_string()
    }
}
