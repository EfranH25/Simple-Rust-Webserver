use std::fmt::{Debug, Display, Formatter};
use std::{sync::mpsc, thread};
pub struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}
pub struct PoolCreationError;

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "ThreadPool size must be greater than zero")
    }
}

impl Debug for PoolCreationError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "PoolCreationError")
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.

    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     if size > 0 {
    //         Ok(ThreadPool)
    //     } else {
    //         Err(PoolCreationError)
    //     }
    // }

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
