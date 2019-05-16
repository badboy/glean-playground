use serde::{Deserialize, Serialize};

use crate::metrics::Metric;
use crate::Glean;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Str(String);

impl Str {
    pub fn category(&self) -> &'static str {
        "string"
    }
}

#[derive(Debug)]
pub struct StringMetric;

impl StringMetric {
    pub fn new() -> Self {
        Self
    }

    pub fn set<S: Into<String>>(&self, glean: &Glean, value: S) {
        let s = value.into();
        let value = Metric::String(Str(s));
        glean.record("string", &value);
    }
}
