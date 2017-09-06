use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

pub struct Threadpool<T, F>
where
    F: Fn(&mut T) + Send + Sync + 'static,
    T: Send + 'static + Debug,
{
    workers: Vec<WorkerThread>,
    tasks: Option<Vec<Arc<Task<T, F>>>>,
    sender: mpsc::Sender<Message<T, F>>,
}

impl<T, F> Threadpool<T, F>
where
    F: Fn(&mut T) + Send + Sync + 'static,
    T: Send + 'static + Debug,
{
    pub fn new(worker_size: usize) -> Threadpool<T, F> {
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

    pub fn batch(mut self, collection: &Vec<Arc<Mutex<T>>>, func: F) {
        let func = Arc::new(func);
        let tasks: Vec<Arc<Task<T, F>>> = collection
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

    fn run(&self, task: Arc<Task<T, F>>) {
        self.sender.send(Message::Execute(task.clone())).unwrap()
    }
}

#[derive(Debug)]
struct Task<T, F> {
    item: Arc<Mutex<T>>,
    func: Arc<F>,
}

impl<T, F> Task<T, F>
where
    F: Fn(&mut T) + Send + Sync + 'static,
{
    fn execute(&self) {
        let mut i = self.item.lock().unwrap();
        (self.func)(&mut i)
    }

    fn new(i: Arc<Mutex<T>>, func: Arc<F>) -> Task<T, F> {
        Task {
            item: i,
            func: func,
        }
    }
}

enum Message<T, F> {
    Execute(Arc<Task<T, F>>),
    Terminate,
}

struct WorkerThread {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl WorkerThread {
    fn new<T, F>(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message<T, F>>>>) -> WorkerThread
    where
        F: Fn(&mut T) + Send + Sync + 'static,
        T: Send + 'static + Debug,
    {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Execute(task) => {
                    println!("Worker {} - task: {:?}: executing.", id, &task.item);
                    task.execute();
                    println!("Worker {} - task: {:?}: done", id, &task.item);
                }
                Message::Terminate => break,
            }
        });
        WorkerThread { id, thread }
    }
}
