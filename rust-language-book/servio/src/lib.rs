use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
// use std::sync::mpsc::channel;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        println!("Starting worker {}", id);
        let id2 = id.clone();

        let t = thread::spawn(move || loop {
            let message = {
                let rx = receiver.lock().unwrap();
                rx.recv().unwrap()
            };
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; execiting!", id2);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} is terminating", id2);
                    break;
                }
            };
        });
        Worker {
            id,
            thread: Some(t),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..size {
            // Need to create some threads ????
            workers.push(Worker::new(i, Arc::clone(&rx)));
        }
        ThreadPool {
            workers,
            sender: sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("Sending job to the pool");
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn join(self) {}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            // t.thread.take().join().unwrap();
            println!("Awaiting worker #{}, shutdown", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
