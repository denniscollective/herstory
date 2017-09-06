use std::thread;
use std::sync::{Arc, Mutex};

pub struct Threadpool<I, F>
where
    F: Fn(&mut I) + Send + Sync + 'static,
    I: Send + 'static,
{
    tasks: Option<Vec<Arc<Task<I, F>>>>,
    handles: Option<Vec<thread::JoinHandle<()>>>,
}

impl<I, F> Threadpool<I, F>
where
    F: Fn(&mut I) + Send + Sync + 'static,
    I: Send + 'static,
{
    pub fn new(_worker_size: u32) -> Threadpool<I, F> {
        Threadpool {
            tasks: None,
            handles: None,
        }
    }

    pub fn batch(mut self, collection: &Vec<Arc<Mutex<I>>>, func: F) {
        let func = Arc::new(func);
        let tasks: Vec<Arc<Task<I, F>>> = collection
            .iter()
            .map(|i| Arc::new(Task::new(i.clone(), func.clone())))
            .collect();

        self.handles = Some(tasks.iter().map(|task| self.run(task.clone())).collect());
        self.tasks = Some(tasks);
        self.wait();
    }

    fn wait(self) {
        for handle in self.handles.unwrap() {
            handle.join().unwrap()
        }
    }
    fn run(&self, task: Arc<Task<I, F>>) -> thread::JoinHandle<()> {
        thread::spawn(move || task.execute())
    }
}

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
