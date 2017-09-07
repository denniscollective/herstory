use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

pub struct Threadpool<T>
where
    T: Send + 'static + fmt::Debug,
{
    workers: Vec<WorkerThread>,
    tasks: Option<Vec<Arc<Task<T>>>>,
    sender: mpsc::Sender<Message<T>>,
}

type PassableFunc<T> = Arc<Fn(&mut T) + Send + Sync + 'static>;

impl<T> Threadpool<T>
where
    T: Send + 'static + fmt::Debug,
{
    pub fn batch<F>(worker_size: usize, collection: &Vec<Arc<Mutex<T>>>, func: F)
    where
        F: Fn(&mut T) + Send + Sync + 'static,
    {
        Self::new(worker_size).iter_collection(collection, Arc::new(func))
    }

    fn new(worker_size: usize) -> Threadpool<T> {
        let mut workers = Vec::with_capacity(worker_size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..worker_size {
            workers.push(WorkerThread::new(i, receiver.clone()))
        }

        Threadpool {
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
        self.sender.send(Message::Execute(task.clone())).unwrap()
    }
}

struct Task<T> {
    item: Arc<Mutex<T>>,
    func: PassableFunc<T>,
}

impl<T> Task<T> {
    fn execute(&self) {
        let mut i = self.item.lock().unwrap();
        (self.func)(&mut i)
    }

    fn new(i: Arc<Mutex<T>>, func: PassableFunc<T>) -> Task<T> {
        Task {
            item: i,
            func: func,
        }
    }
}

impl<T> fmt::Debug for Task<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task {{ item: {:?} }}", self.item)
    }
}

enum Message<T> {
    Execute(Arc<Task<T>>),
    Terminate,
}

struct WorkerThread {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl WorkerThread {
    fn new<T>(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message<T>>>>) -> WorkerThread
    where
        T: Send + 'static + fmt::Debug,
    {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Execute(task) => {
                    println!("Worker {} - task: {:?}: executing.", id, task.item);
                    task.execute();
                    println!("Worker {} - task: {:?}: done", id, task.item);
                }
                Message::Terminate => break,
            }
        });
        WorkerThread { id, thread }
    }
}
