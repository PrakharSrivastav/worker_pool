# worker - pool

An attempt to create thread based worker pool in rust. The application creates a worker pool with the user provided limit and concurrently schedules the job execution over the worker.

The worker takes a function as input instead of a concrete struct or enum, thus making it workload agnostic.



```shell
cargo build
cargo clippy
cargo run
```
