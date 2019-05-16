use glean_playground::*;

fn main() {
    let glean = Glean::new();
    let counter = CounterMetric::new();

    counter.add(&glean, 1);
    println!("Snapshot: {}", glean.snapshot());
}
