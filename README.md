# worker - pool

An attempt to create thread based (**not async/await**) worker pool in rust. The application creates a worker pool with the user provided limit and concurrently schedules the job execution over the worker.

The whole setup consists of
- Worker : Struct simulating an allocated thread to process job.
- Pool: Owns a Worker. Controls number of spawned worker threads.
- Job: An executable function carrying the processing logic making worker agnostic of workloads.

The caller (`fn main` in this case) can spawn a pool of given size and then sumbit workloads

```shell
cargo build
cargo clippy
cargo run
```
          
Ref : https://doc.rust-lang.org/book/


## Summary  (by chatgpt)

This code snippet implements a basic worker pool in Rust, which facilitates efficient task execution through parallel worker threads. Here's a concise overview:

### Worker Struct
- Represents an individual worker thread.
- Stores an ID and a reference to the spawned thread.

### Worker Implementation
- Provides a constructor to create worker instances.
- Threads continually process jobs from the shared receiver.

### Job Type
- Defines a job or workload as a **closure** that can be sent across channels.

### Pool Struct
- Manages a collection of worker instances.
- Contains a sender for submitting jobs to workers.

### Pool Implementation
- Creates a new worker pool.
- Jobs are submitted using closures to the pool.

### `execute` Method
- Sends jobs to workers for execution via the channels.

### Pool Cleanup (Drop trait)
- Ensures a smooth shutdown of workers and threads.

The code effectively utilizes Rust's concurrency features to enable efficient parallel task processing using the concept of a worker pool.

