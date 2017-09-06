use std::thread;
use std::sync::{Arc, Mutex};

pub struct Threadpool {}

impl Threadpool {
    pub fn new(_worker_size: u32) -> Threadpool {
        Threadpool {}
    }

    pub fn batch<F, G>(self, collection: &Vec<Arc<Mutex<G>>>, func: F)
    where
        F: Fn(&mut G) + Send + Sync + 'static,
        G: Send + 'static,
    {
        let func = Arc::new(func);
        let tasks: Vec<thread::JoinHandle<()>> = collection
            .iter()
            .map(|i| Task::clone_and_execute(&i, &func))
            .collect();

        for handle in tasks {
            handle.join().unwrap()
        }
    }
}

struct Task<G, H> {
    item: Arc<Mutex<G>>,
    func: Arc<H>,
}

impl<G, H> Task<G, H>
where
    H: Fn(&mut G) + Send + Sync + 'static,
    G: Send + 'static,
{
    fn execute(&self) {
        let mut i = self.item.lock().unwrap();
        (self.func)(&mut i)
    }

    fn clone_and_execute(i: &Arc<Mutex<G>>, func: &Arc<H>) -> thread::JoinHandle<()> {
        let task = Task {
            item: i.clone(),
            func: func.clone(),
        };

        thread::spawn(move || task.execute())
    }
}
