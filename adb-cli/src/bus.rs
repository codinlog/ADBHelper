#![allow(unused)]

use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

enum NewJob {
    Job(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<NewJob>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                NewJob::Job(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                NewJob::Terminate => {
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

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct MsgBus {
    workers: Vec<Worker>,
    sender: Sender<NewJob>,
}

impl MsgBus {
    pub fn new() -> MsgBus {
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        MsgBus {
            workers: vec![Worker::new(1, receiver.clone())],
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(NewJob::Job(Box::new(f))).unwrap();
    }
}

impl Drop for MsgBus {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(NewJob::Terminate).unwrap();
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
