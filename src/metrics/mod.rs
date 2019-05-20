use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

mod counter;
mod datetime;
mod string;

pub use self::counter::CounterMetric;
pub use self::datetime::{DateTimeMetric, TimeUnit};
pub use self::string::StringMetric;

use self::datetime::get_iso_time_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Metric {
    Counter(u64),
    String(String),
    DateTime(TimeUnit, DateTime<Local>),
}

impl Metric {
    pub fn category(&self) -> &'static str {
        match self {
            Metric::Counter(_) => "counter",
            Metric::String(_) => "string",
            Metric::DateTime(..) => "datetime",
        }
    }

    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::Counter(c) => json!(c),
            Metric::String(s) => json!(s),
            Metric::DateTime(tu, dt) => json!(get_iso_time_string(dt, *tu).to_string()),
        }
    }
}
