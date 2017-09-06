// use std::io;
use std::time;
use std::thread;
use std::sync::{Arc, Mutex};

mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;

use Config;

pub type Images = Vec<Arc<Mutex<Image>>>;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Images,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(&self) {
        let mut threadpool: Threadpool<Arc<Mutex<Image>>> = Threadpool::new(4);
        threadpool.batch(&self.images, |image| {
            let mut i = image.lock().unwrap();
            i.spawn_request();
        });

        threadpool.values().unwrap();
    }
}

#[derive(Debug)]
pub struct Image {
    pub index: i32,
    pub url: String,
    pub request: Option<Request>,
}

impl Image {
    fn filename(&self) -> String {
        let t = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        format!("{}/{}_{}", Config::DATA_DIR, self.index, t)
    }

    pub fn spawn_request(&mut self) {
        let request = Request::build(&self.url, &self.filename());
        self.request = Some(request.perform_and_save());
    }
}

struct Threadpool<T: Send> {
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

    pub fn execute<F>(&mut self, fun: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.state = ThreadpoolState::Waiting;
        self.handles.push(thread::spawn(fun));
    }

    pub fn wait(mut self) -> Threadpool<T> {
        for handle in self.handles {
            handle.join().unwrap();
        }
        self.handles = Vec::new();
        self.state = ThreadpoolState::Done;
        self
    }

    pub fn values(mut self) -> Option<Vec<T>> {
        match self.state {
            ThreadpoolState::Initialized => None,
            ThreadpoolState::Waiting => {
                self = self.wait();
                Some(self.values)
            }
            ThreadpoolState::Done => Some(self.values),
        }
    }

    pub fn batch<Z>(&mut self, collection: &Vec<Arc<Mutex<Image>>>, func: Z)
    where
        Z: Fn(Arc<Mutex<Image>>) + Send + Sync + 'static,
    {
        let func = Arc::new(func);
        collection
            .iter()
            .map(|i| {
                let item = i.clone();
                let funk = func.clone();
                self.execute(move || funk(item))
            })
            .count();
    }
}

enum ThreadpoolState {
    Initialized,
    Waiting,
    Done,
}
