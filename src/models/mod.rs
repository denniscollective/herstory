// use std::io;
use std::time;
use std::thread;

mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;

use Config;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Vec<Image>,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(mut self) -> Photoset {

        self.images = {
            let mut threadpool: Threadpool<Image> = Threadpool::new(4);

            let handles: Vec<thread::JoinHandle<()>> = self.images
                .into_iter()
                .map(|mut image: Image| {
                    threadpool.execute(move || { image.request = Some(image.spawn_request()); })
                })
                .collect(); //collect here to spawn all threads


            threadpool.batch(handles).values().unwrap()

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

    pub fn spawn_request(&mut self) -> Request {
        let request = Request::build(&self.url, &self.filename());
        request.perform_and_save()
    }
}

struct Threadpool<T> {
    values: Vec<T>,
    state: ThreadpoolState,
    handles: Option<Vec<thread::JoinHandle<()>>>,
}

impl<T> Threadpool<T> {
    pub fn new(_worker_size: u32) -> Threadpool<T> {
        let values: Vec<T> = Vec::new();

        Threadpool {
            values: values,
            state: ThreadpoolState::Initialized,
            handles: None,
        }
    }

    pub fn execute<F>(&mut self, fun: F) -> thread::JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        self.state = ThreadpoolState::Waiting;
        thread::spawn(fun)
    }

    pub fn wait(mut self) -> Threadpool<T> {
        for handle in self.handles.unwrap() {
            handle.join().unwrap();
        }
        self.handles = None;
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

    pub fn batch(mut self, handles: Vec<thread::JoinHandle<()>>) -> Threadpool<T> {
        self.handles = Some(handles);
        self.wait()

    }
}

enum ThreadpoolState {
    Initialized,
    Waiting,
    Done,
}
