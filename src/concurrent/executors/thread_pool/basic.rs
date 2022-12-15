use super::*;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct BasicThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool for BasicThreadPool {
    fn new(threads: usize) -> Result<Self>
    where
        Self: Sized,
    {
        assert!(threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(threads);

        for id in 0..threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(BasicThreadPool { workers, sender })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::NewJob(Box::new(job))).unwrap();
    }

    fn shutdown(self) {
        drop(self)
    }
}

impl Drop for BasicThreadPool {
    /// Waits for remaining jobs to finish and then terminates all workers
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new worker that will receive a task and run it to completion
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        // Setup
        let pool = BasicThreadPool::new(1).unwrap();
        let (send, receive) = mpsc::channel();

        // Act: spawn a task in the thread-pool which sends a value over the channel
        let value = 999;
        pool.spawn(move || send.send(value).unwrap());

        // Shut the thread-pool down
        drop(pool);

        // ensure that the task has completed by fetching the result
        let result = receive.recv().unwrap();
        assert_eq!(value, result)
    }

    #[test]
    fn run_many() {
        // Setup
        let pool = BasicThreadPool::new(1).unwrap();

        for i in 0..10 {
            pool.spawn(move || println!("Task {} completed", i));
        }

        drop(pool);
    }
}
