use std::sync::{Arc, mpsc, Mutex};
use std::thread;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("worker {} processing job ", id);
                job()
            }
            // loop {
            //     match receiver.lock().unwrap().recv() {
            //         Ok(job) => {
            //             println!("worker {} processing job ", id);
            //             job();
            //         }
            //         Err(_) => {
            //             //eprintln!("worker {} with error {}", id, e);
            //             break;
            //         }
            //     }
            // }
        });
        println!("creating worker {} ", id);
        Worker { id, thread: Some(thread) }
    }
}

/// this type represents a function as a workload
/// Job has following traits
///  - It has static lifetime
///  - It is Send-able across channels
///  - It is a function that can be executed only once
type Job = Box<dyn FnOnce() + Send + 'static>;


/// Pool holds a finite set of worker that execute a given workload
/// exactly once
pub struct Pool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Pool {
    /// new creates a pool with workers with a given size
    pub fn new(size: usize) -> Pool {
        assert!(size > 0);
        let (pool_tx, pool_rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(pool_rx));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Pool {
            workers,
            sender: Some(pool_tx),
        }
    }

    /// execute sends a give job to the sender channel
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static, {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}
