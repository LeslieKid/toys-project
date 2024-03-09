use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    let mut workers: Vec<Worker> = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel(); 
    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
      let worker = Worker::new(id, Arc::clone(&receiver)); 
      workers.push(worker);      
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F) 
  where
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f); 
    self.sender.send(job).unwrap();
  }
}

struct Worker {
  id: usize,
  worker: thread::JoinHandle<()>
}
impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();
      println!("Receive the job which id is {id} successfully.");

      job()
    });    
    Worker { id, worker: thread}
  }
}