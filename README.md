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
