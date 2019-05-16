use std::collections::HashMap;
use std::sync::Mutex;

use serde_json::Value as JsonValue;

mod metrics;

pub use metrics::CounterMetric;
pub use metrics::Metric;
pub use metrics::StringMetric;

#[derive(Debug)]
pub struct Glean {
    data: Mutex<HashMap<String, Vec<u8>>>,
}

impl Glean {
    pub fn new() -> Self {
        Glean {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn record(&self, typ: &str, value: &Metric) {
        let mut data = self.data.lock().unwrap();
        data.insert(typ.into(), bincode::serialize(value).unwrap());
    }

    pub fn record_with<F>(&self, typ: &str, transform: F)
    where
        F: Fn(Option<Metric>) -> Metric,
    {
        let mut data = self.data.lock().unwrap();
        let old_value = data.get(typ);

        let new_value = match old_value {
            Some(blob) => {
                let old_value = bincode::deserialize(blob).ok();
                transform(old_value)
            }
            _ => transform(None),
        };

        data.insert(typ.into(), bincode::serialize(&new_value).unwrap());
    }

    pub fn snapshot(&self) -> String {
        let mut snapshot: HashMap<String, JsonValue> = HashMap::new();
        let data = self.data.lock().unwrap();

        for (key, value) in &*data {
            let metric: Metric = bincode::deserialize(value).unwrap();
            snapshot.insert(key.to_owned(), metric.as_json());
        }

        ::serde_json::to_string_pretty(&snapshot).unwrap()
    }
}
