// use std::io;
use std::time;
use std::thread;
use std::sync::{Arc, Mutex};

mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;

use Config;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Vec<Arc<Mutex<Image>>>,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(mut self) -> Photoset {

        self.images = {
            let mut threadpool: Threadpool<Arc<Mutex<Image>>> = Threadpool::new(4);
            self.images
                .into_iter()
                .map(|image| {
                    let img = image.clone();
                    threadpool.execute(move || {
                        let mut image = img.lock().unwrap();
                        image.spawn_request();
                    })
                })
                .count();
            threadpool.values().unwrap()
        };

        self
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

impl<'a, 'f, T: Send> Threadpool<T> {
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
}

enum ThreadpoolState {
    Initialized,
    Waiting,
    Done,
}
