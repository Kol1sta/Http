use std::{
    thread,
    sync::{mpsc, Arc, Mutex}
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pub workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver): (mpsc::Sender<Job>, mpsc::Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(thread::spawn(move || {
                while let Ok(job) = receiver.lock().unwrap().recv() {
                    job();
                }
            }));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.send(Box::new(f)).unwrap();
    }
}
