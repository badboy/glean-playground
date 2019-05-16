# Glean Playground

## Metric Implementations - Alternative solution


* Recording implementation per metric AND metric store (2 types!)
    * Impl
        * [Counter](src/metrics/counter.rs)
        * [String](src/metrics/string.rs)
    * Recording impl
        * Holds the common metric data
        * Stores into the Glean storage
    * Store impl
        * Simple wrapper around value
        * Implements category and JSON serialization
* Storage: `Metric` enum wrapping the Metric type
    * Impl
        * [Metric](src/metrics/mod.rs)
    * List of all stored metric types
    * Defers category to metric type (always the same)
    * Defers JSON serialization to metric type (always the same)
    * No additional logic required
    
Advantage:

* Simple serialization/deserialization into storage (same as before)
    * All handled by `bincode` for us
* Easy serialization into JSON (same as before)
    * Iterate over everything and it tells us what it is

    
Disadvantage:

* Still 2 files to modify when adding new type
    * but logic only in one file
