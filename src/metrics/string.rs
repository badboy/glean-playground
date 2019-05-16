use crate::metrics::Metric;
use crate::Glean;

#[derive(Debug)]
pub struct StringMetric;

impl StringMetric {
    pub fn new() -> Self {
        Self
    }

    pub fn set<S: Into<String>>(&self, glean: &Glean, value: S) {
        let s = value.into();
        let value = Metric::String(s);
        glean.record("string", &value);
    }
}
