use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use HasStatus;

pub struct Threadpool<T>
where
    T: Send + 'static + HasStatus,
{
    workers: Vec<WorkerThread>,
    tasks: Option<Vec<Arc<Task<T>>>>,
    sender: mpsc::Sender<Message<T>>,
}

type PassableFunc<T> = Arc<Fn(&mut T) + Send + Sync + 'static>;

impl<T> Threadpool<T>
where
    T: Send + 'static + HasStatus,
{
    pub fn batch<F>(worker_size: usize, collection: &Vec<Arc<Mutex<T>>>, func: F)
    where
        F: Fn(&mut T) + Send + Sync + 'static,
    {
        Self::new(worker_size).iter_collection(collection, Arc::new(func))
    }

    fn new(worker_size: usize) -> Self {
        let mut workers = Vec::with_capacity(worker_size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..worker_size {
            workers.push(WorkerThread::new(i, receiver.clone()))
        }

        Self {
            workers,
            sender,
            tasks: None,
        }
    }

    fn iter_collection(mut self, collection: &Vec<Arc<Mutex<T>>>, func: PassableFunc<T>) {
        let tasks: Vec<Arc<Task<T>>> = collection
            .iter()
            .map(|i| Arc::new(Task::new(i.clone(), func.clone())))
            .collect();

        for task in &tasks {
            self.run(task.clone())
        }

        self.tasks = Some(tasks);
        self.wait()
    }

    fn wait(mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in self.workers {
            worker.thread.join().unwrap();
            println!("Finished: {}", worker.id)
        }
    }

    fn run(&self, task: Arc<Task<T>>) {
        self.sender.send(Message::Execute(task)).unwrap()
    }
}


pub struct Task<T>
where
    T: HasStatus,
{
    item: Arc<Mutex<T>>,
    func: PassableFunc<T>,
}


impl<T> Task<T>
where
    T: HasStatus,
{
    fn execute(&self) {
        let mut i = self.item.lock().unwrap();
        (self.func)(&mut i)
    }

    fn new(i: Arc<Mutex<T>>, func: PassableFunc<T>) -> Self {
        Self {
            item: i,
            func: func,
        }
    }

    fn status(&self) -> String {
        let lock = self.item.try_lock();
        if let Ok(ref item) = lock {
            item.status().to_string()
        } else {
            "Lock Failed".to_string()
        }
    }
}

impl<T> fmt::Debug for Task<T>
where
    T: HasStatus,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{ item: {:?} status: {} }} ",
            self.item,
            self.status()
        )
    }
}

enum Message<T>
where
    T: HasStatus,
{
    Execute(Arc<Task<T>>),
    Terminate,
}

struct WorkerThread {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl WorkerThread {
    fn new<T>(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message<T>>>>) -> Self
    where
        T: Send + 'static + HasStatus,
    {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Execute(task) => {
                    println!("Worker{} - {:?}", &id, &task);
                    task.execute();
                    println!("Worker {} - {:?}", &id, &task);
                }
                Message::Terminate => break,
            }
        });
        Self { id, thread }
    }
}
