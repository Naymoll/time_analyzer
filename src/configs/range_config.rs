use crate::configs::{ArgumentGenerator, Range};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RangeConfig {
    #[serde(flatten)]
    range: Range,
}

impl ArgumentGenerator for RangeConfig {
    fn len(&self) -> usize {
        self.range.start
    }

    fn next_len(&mut self) -> usize {
        self.range.next()
    }

    fn generate(&self) -> String {
        self.range.start.to_string()
    }
}
