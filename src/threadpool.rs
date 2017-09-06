use std::thread;
use std::sync::{Arc, Mutex};

use models::Image; // make generic by removinv this.

pub struct Threadpool {
    handles: Vec<thread::JoinHandle<()>>,
}

impl Threadpool {
    pub fn new(_worker_size: u32) -> Threadpool {
        Threadpool { handles: Vec::new() }
    }

    pub fn execute<F>(&mut self, fun: Arc<F>, item: Arc<Mutex<Image>>)
    where
        F: Fn(&mut Image) + Send + Sync + 'static,
    {
        self.handles.push(thread::spawn(move || {
            let mut i = item.lock().unwrap();
            fun(&mut i)
        }));
    }

    pub fn batch<Z>(mut self, collection: &Vec<Arc<Mutex<Image>>>, func: Z)
    where
        Z: Fn(&mut Image) + Send + Sync + 'static,
    {
        let func = Arc::new(func);
        collection
            .iter()
            .map(|i| {
                let item = i.clone();
                let funk = func.clone();
                self.execute(funk, item)
            })
            .count();

        self.wait();
    }

    pub fn wait(mut self) -> Threadpool {
        for handle in self.handles {
            handle.join().unwrap();
        }
        self.handles = Vec::new();

        self
    }
}
