use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

pub struct Bucket {
    rate: Arc<Mutex<f64>>,
    current: Arc<Mutex<usize>>,
    max: Arc<Mutex<usize>>,
}

impl Bucket {
    pub fn new(rate: f64, max: usize) -> Bucket {
        Bucket {
            rate: Arc::new(Mutex::new(rate)),
            current: Arc::new(Mutex::new(max)),
            max: Arc::new(Mutex::new(max)),
        }
    }

    pub fn listen(&mut self) {
        let current = Arc::clone(&self.current);
        let max = Arc::clone(&self.max);
        let rate = Arc::clone(&self.rate);

        thread::spawn(move || loop {
            let rate_lock = rate.lock().unwrap();
            let max_lock = max.lock().unwrap();
            sleep(Duration::from_secs((1.0 / *rate_lock) as u64));
            let mut current_lock = current.lock().unwrap();

            if *current_lock < *max_lock {
                *current_lock += 1;
            }
            std::mem::drop(current_lock);
        });
    }

    pub fn decrement(&mut self) -> bool {
        let mut current_lock = self.current.lock().unwrap();
        println!("val: {}", *current_lock);
        if *current_lock > 0 {
            *current_lock -= 1;
            true
        } else {
            false
        }
    }
}
