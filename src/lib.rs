use std::fmt::{Formatter, Display, Debug};

pub struct ThreadPool;

pub struct PoolCreationError;

impl Display for PoolCreationError{
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

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool)
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,
    {}
}