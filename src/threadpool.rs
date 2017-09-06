use std::thread;
use std::sync::{Arc, Mutex};

use models::Image; // make generic by removinv this.

pub struct Threadpool<T: Send> {
    values: Vec<T>,
    state: ThreadpoolState,
    handles: Vec<thread::JoinHandle<()>>,
}

impl<T: Send> Threadpool<T> {
    pub fn new(_worker_size: u32) -> Threadpool<T> {
        let values: Vec<T> = Vec::new();

        Threadpool {
            values: values,
            state: ThreadpoolState::Initialized,
            handles: Vec::new(),
        }
    }

    pub fn execute<F>(&mut self, fun: Arc<F>, item: Arc<Mutex<Image>>)
    where
        F: Fn(&mut Image) + Send + Sync + 'static,
    {
        self.state = ThreadpoolState::Waiting;
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

        self.values().unwrap();
    }

    pub fn wait(mut self) -> Threadpool<T> {
        for handle in self.handles {
            handle.join().unwrap();
        }
        self.handles = Vec::new();
        self.state = ThreadpoolState::Done;
        self
    }

    fn values(mut self) -> Option<Vec<T>> {
        match self.state {
            ThreadpoolState::Initialized => None,
            ThreadpoolState::Waiting => {
                self = self.wait();
                Some(self.values)
            }
            ThreadpoolState::Done => Some(self.values),
        }
    }
}

enum ThreadpoolState {
    Initialized,
    Waiting,
    Done,
}
