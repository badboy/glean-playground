use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

mod counter;
mod string;

pub use self::counter::*;
pub use self::string::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Metric {
    Counter(Counter),
    String(Str),
}

impl Metric {
    pub fn category(&self) -> &'static str {
        match self {
            Metric::Counter(c) => c.category(),
            Metric::String(s) => s.category(),
        }
    }

    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::Counter(c) => json!(c),
            Metric::String(s) => json!(s),
        }
    }
}
