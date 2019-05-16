use crate::metrics::Metric;
use crate::Glean;

#[derive(Debug)]
pub struct CounterMetric;

impl CounterMetric {
    pub fn new() -> Self {
        Self
    }

    pub fn add(&self, glean: &Glean, amount: u64) {
        glean.record_with("counter", |old_value| match old_value {
            Some(Metric::Counter(old_value)) => Metric::Counter(old_value + amount),
            _ => Metric::Counter(amount),
        })
    }
}
