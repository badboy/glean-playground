# Glean Playground

## Metric Implementations - Current solution


* Recording implementation per metric
    * Impl
        * [Counter](src/metrics/counter.rs)
        * [String](src/metrics/string.rs)
    * Holds the common metric data
    * Stores into the Glean storage
    * Only knows about `Metrics` enum for serialization
* Storage: `Metric` enum
    * Impl
        * [Metric](src/metrics/mod.rs)
    * List of all stored metric types
    * Lists every category as a string
    * Handles serialization into JSON
    * Deserialization made easy: deserialize byte blob into `Metric` enum.

    
Advantage:

* Simple serialization/deserialization into storage
    * All handled by `bincode` for us
* Easy serialization into JSON
    * Iterate over everything and it tells us what it is

    
Disadvantage:

* Separate files for recording & JSON-serialization
    * Always need to add logic to 2 files when adding new type
