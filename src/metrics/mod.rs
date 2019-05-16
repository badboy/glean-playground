use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

mod counter;
mod string;

pub use self::counter::CounterMetric;
pub use self::string::StringMetric;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Metric {
    Counter(u64),
    String(String),
}

impl Metric {
    pub fn category(&self) -> &'static str {
        match self {
            Metric::Counter(_) => "counter",
            Metric::String(_) => "string",
        }
    }

    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::Counter(c) => json!(c),
            Metric::String(s) => json!(s),
        }
    }
}
