use std::thread;
use std::sync::{Arc, Mutex};

pub struct Threadpool {
    handles: Vec<thread::JoinHandle<()>>,
}

impl Threadpool {
    pub fn new(_worker_size: u32) -> Threadpool {
        Threadpool { handles: Vec::new() }
    }

    fn execute<F, G>(&mut self, task: Task<G, F>)
    where
        F: Fn(&mut G) + Send + Sync + 'static,
        G: Send + 'static,
    {
        self.handles.push(thread::spawn(move || task.execute()));
    }

    pub fn batch<F, G>(mut self, collection: &Vec<Arc<Mutex<G>>>, func: F)
    where
        F: Fn(&mut G) + Send + Sync + 'static,
        G: Send + 'static,
    {
        let func = Arc::new(func);
        collection
            .iter()
            .map(|i| {

                self.execute(Task {
                    item: i.clone(),
                    func: func.clone(),
                })
            })
            .count();

        self.wait();
    }

    fn wait(self) {
        for handle in self.handles {
            handle.join().unwrap();
        }
    }
}

struct Task<G, H> {
    item: Arc<Mutex<G>>,
    func: Arc<H>,
}

impl<G, H> Task<G, H>
where
    H: Fn(&mut G),
{
    fn execute(&self) {
        let mut i = self.item.lock().unwrap();
        (self.func)(&mut i)
    }
}
