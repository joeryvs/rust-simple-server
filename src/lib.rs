use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new TreadPool
    ///
    /// The size is the number of workers in the pool
    ///
    /// # Panics
    ///
    /// the new function will panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            // create some workers
            workers.push(Worker::new(i));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F) -> ()
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Worker {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}
