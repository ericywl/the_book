use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    SizeIsZero,
    IOError(io::Error),
}

pub struct ExecuteError;

impl From<io::Error> for PoolCreationError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// The `build` function will return PoolCreationError if size is zero.
    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::SizeIsZero);
        }

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for i in 1..=size {
            workers.push(Worker::build(i, Arc::clone(&receiver))?)
        }

        Ok(Self {
            workers,
            sender: Some(sender),
        })
    }

    /// Execute a closure using the ThreadPool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn build(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Self, io::Error> {
        let builder = thread::Builder::new();
        let handle = builder.spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        })?;

        Ok(Self {
            id,
            handle: Some(handle),
        })
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_error_if_size_zero() {
        let err = ThreadPool::build(0).err().unwrap();
        match err {
            PoolCreationError::SizeIsZero => {}
            _ => panic!("build returned wrong error for size zero"),
        }
    }
}
