use glean_playground::*;

fn main() {
    let glean = Glean::new();
    let counter = CounterMetric::new();

    counter.add(&glean, 1);

    let created = DateTimeMetric::new(TimeUnit::Day);
    created.set(&glean);

    println!("Snapshot: {}", glean.snapshot());
}
