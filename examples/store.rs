use glean_playground::*;

fn main() {
    let glean = Glean::new();
    let counter = CounterMetric::new();
    let s = StringMetric::new();

    counter.add(&glean, 1);
    s.set(&glean, "glean");
    println!("Snapshot: {}", glean.snapshot());
}
